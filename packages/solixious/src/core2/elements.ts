import { Container } from "@pixi/display";
import { Mesh, MeshGeometry, MeshMaterial } from "@pixi/mesh";
import * as interaction from "@pixi/interaction";
import {
  createMemo,
  createRenderEffect,
  JSX,
  mapArray,
  splitProps,
} from "solid-js";
import { usePixi } from "./store";
import {
  AttachStrategy,
  Constructable,
  NonFunctionKeys,

  SxiInstance,
  SxiObject,
  SxiState,
} from "./types";
import { ObservablePoint, Point, Texture } from "@pixi/core";

export const INTERNAL_PROPS = ["children", "ref"];

// Gets only instance props from proxy-component
export function getInstanceProps<T extends Constructable>(
  queue: Record<string, unknown>,
): SxiInstance<T>["props"] {
  // SOLID-THREE-NOTE:  solid-three has to use splitProps so getters are not resolved
  const [_, props] = splitProps(queue, INTERNAL_PROPS);
  return props;
}

// export const applyProp = <T extends Constructable>(
//   object: SxiObject<T>,
//   prop: string,
//   value: unknown,
// ) => {
//   const instance = object.__sxi as SxiInstance<T>;
// };

export const applyProps = <T extends Constructable>(
  object: SxiObject<T>,
  props: SxiInstance<T>["props"],
) =>
  createRenderEffect(mapArray(() => Object.keys(props), (key) => {
    /* We wrap it in an effect only if a prop is a getter or a function */
    // const descriptors = Object.getOwnPropertyDescriptor(props, key);
    // const isDynamic =
    //   !!(descriptors?.get || typeof descriptors?.value === "function");
    // const update = (value: unknown) => applyProp(object, key, value);
    // isDynamic
    //   ? createRenderEffect(on(() => props[key], update))
    //   : update(props[key]);
  }));

/**
 * Wraps an object in a SxiInstance<T>
 * @template T extends Constructable - Type of obj to wrap
 * @param target - Obj to wrap
 * @param state - Shared SxiState
 * @param type - Type string
 * @param props - Props
 * @returns
 */
const prepareObject = <
  TSource extends Constructable,
  TObject extends InstanceType<TSource>,
>(
  target: TObject & { __sxi?: SxiInstance<TSource, TObject> },
  state: SxiState,
  type: string,
  props: SxiInstance<TSource>["props"],
) => {
  const object: TObject & { __sxi?: SxiInstance<TSource> } = target;

  const instance: SxiInstance<TSource> = object?.__sxi ?? {
    solixi: state,
    type,
    parent: null,
    object: object as SxiObject<TSource>,
    children: [],
    props: getInstanceProps(props),
  };

  if (object) {
    object.__sxi = instance;
    if (type) {
      // TODO Apply props to object
    }
  }
  return instance;
};

type ExtraPropHandler<
  TSource extends Constructable,
  V = unknown,
> = (
  state: SxiState,
  parent: SxiObject<Constructable>,
  object: SxiObject<TSource>,
  value: V,
) => void | (() => void);

type ExtraPropsHandlers<
  TSource extends Constructable,
> = { [k: string]: ExtraPropHandler<TSource> };

type ExtraPropsSignature<T extends ExtraPropsHandlers<Constructable>> = {
  [K in keyof T]: Parameters<T[K]>[3]
}
export type ClassTypeProps<
  TSource extends Constructable,
  TExtraProps extends Record<string, ExtraPropHandler<TSource>> = Record<string, ExtraPropHandler<Constructable>>,
  TObject extends InstanceType<TSource> = InstanceType<TSource>,
> =
  & { args: ConstructorParameters<TSource> } // Constructor args in ['args'] prop
  & Pick<TObject, NonFunctionKeys<TObject>> // All fields can be set
  & ExtraPropsSignature<TExtraProps>; // All extra props represented

// const x = {
//   onClick: (state, parent, object, value: (event: MouseEvent) => void) => {
//     console.log('onClick', value);
//     return () => {};
//   }
// } satisfies ExtraPropsHandlers<typeof Mesh, Mesh>;
// type v = ExtraPropsSignature<typeof x>;


type WrapConstructableOptions<
  TSource extends Constructable,
  TExtraProps extends Record<string, ExtraPropHandler<TSource>> = Record<string, ExtraPropHandler<TSource>>,
> = {
  // How to attach this object to the parent
  attach: AttachStrategy<TSource, InstanceType<TSource>>;
  // Extra props and their handlers
  extraProps: TExtraProps;
};


/**
 * Wraps a Constructable class in a SolidJS component, to be used in JSX.
 *
 * @template TSource extends Constructable -
 * @param source - Class to wrap
 * @param options - Options defining how to attach parent -> children + add extra behaviours like
 * @returns
 */
export const wrapConstructable = <
  TSource extends Constructable,
  TExtraProps extends Record<string, ExtraPropHandler<TSource>>,
  TObject extends InstanceType<TSource> = InstanceType<TSource>,
>(
  source: TSource,
  options: WrapConstructableOptions<TSource, TExtraProps>,
) => {
  const Component = (
    props: ClassTypeProps<TSource, TExtraProps>
  ) => {
    const store = usePixi();

    const object = createMemo(() => {
      const args: ConstructorParameters<TSource> | unknown[] = props.args ?? [];

      const sourceObject: TObject = new source(args);
      const instance = prepareObject<TSource, TObject>(
        sourceObject,
        store,
        source.name,
        props,
      );
      return instance.object;
    });

    return object as unknown as JSX.Element;
  };
  return Component;
};

const MyMesh = wrapConstructable(Mesh, {
  attach: (parent: SxiObject<typeof Container>, child) => {
    parent.addChild(child);
    return () => parent.removeChild(child);
  },
  extraProps: {
    onClick: (_state, _parent, object, value: (e: MouseEvent) => void) => {
      // @ts-expect-error; Bad typing on interactivity.
      child.on("click", value);
      return () => {
        // @ts-expect-error; Bad typing on interactivity.
        child.off("click", value);
      };
    },
    otherX: (_state, _parent, object, value: number) => {
      object.x = value;
    }
  },
});
const MyContainer = wrapConstructable(Container, {
  attach: (parent: SxiObject<typeof Container>, child) => {
    if (!(parent instanceof Container)) {
      throw new Error("Container must be a child of a Container.");
    }
    parent.addChild(child);
    return () => parent.removeChild(child);
  },
  extraProps: {
    onClick: (state, parent, child, value: any) => {
      child.on("click", value);
      return () => {
        child.off("click", value);
      };
    },
  },
});
