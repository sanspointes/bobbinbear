import * as core from "@pixi/core";
import * as app from "@pixi/app";
import * as interaction from "@pixi/interaction";
import * as display from "@pixi/display";
import * as ticker from "@pixi/ticker";

/*
 * UTILITY TYPES
 */
export type NonFunctionKeys<T> = {
  // eslint-disable-next-line @typescript-eslint/ban-types
  [K in keyof T]: T[K] extends Function ? never : K;
}[keyof T];
export type Overwrite<T, O> = Omit<T, NonFunctionKeys<O>> & O;
export type Constructable = new (...args: unknown[]) => unknown;
export type Args<T> = T extends Constructable ? ConstructorParameters<T>
  : unknown[];

// eslint-disable-next-line @typescript-eslint/ban-types
export interface ClassType<T> extends Function {
  new (...args: unknown[]): T;
}

/*
 * GENERAL TYPES
 */
export interface EventHandlers {
  onClick?: (event: interaction.InteractionEvent) => void;
  onContextMenu?: (event: interaction.InteractionEvent) => void;
  onDoubleClick?: (event: interaction.InteractionEvent) => void;
  onPointerUp?: (event: interaction.InteractionEvent) => void;
  onPointerDown?: (event: interaction.InteractionEvent) => void;
  onPointerOver?: (event: interaction.InteractionEvent) => void;
  onPointerOut?: (event: interaction.InteractionEvent) => void;
  onPointerEnter?: (event: interaction.InteractionEvent) => void;
  onPointerLeave?: (event: interaction.InteractionEvent) => void;
  onPointerMove?: (event: interaction.InteractionEvent) => void;
  onPointerMissed?: (event: interaction.InteractionEvent) => void;
  onPointerCancel?: (event: interaction.InteractionEvent) => void;
  onWheel?: (event: interaction.InteractionEvent) => void;
}

/*
 * MATH TYPES
 */
export interface MathRepresentation {
  set(...args: number[]): unknown;
}
export type Matrix =
  | core.Matrix
  | Parameters<core.Matrix["set"]>
  | Readonly<core.Matrix["set"]>;
export type MathType<T extends MathRepresentation | core.Color> = T extends
  core.Color
  ? ConstructorParameters<typeof core.Color> | core.Color | core.ColorSource
  : T extends MathRepresentation ? T | Parameters<T["set"]> | number
  : T;

export type Point = ConstructorParameters<typeof core.Point> | core.Point;
export type Color =
  | ConstructorParameters<typeof core.Color>
  | core.Color
  | core.ColorSource;

// Attach
export type AttachFnStrategy<
  TSource extends Constructable,
  TObject extends InstanceType<TSource> = InstanceType<TSource>,
> = (parent: SxiObject<Constructable, unknown>, child: SxiObject<TSource, TObject>) => () => void;
/**
 * Strategy for attaching/detatching a child to a parent.  Can either be a string, representing the function field on the parent
 * where the child is passed in as a parameter, or a method that provides access to both the parent and child.
 */
export type AttachStrategy<
  TSource extends Constructable,
  TObject extends InstanceType<TSource> = InstanceType<TSource>,
> = string | AttachFnStrategy<TSource, TObject>;

/*
 * STATE TYPES
 */

/**
 * Root state of the Pixi app
 */
export type SxiState = {
  // Pixi objects
  app: app.Application;
  stage: display.Container;
  ticker: ticker.Ticker;

  /** Whether or not this SxiState content is mounted to the page */
  active: boolean;
};

// INSTANCE TYPES
//

export type SxiInstanceReservedProps<
  TSource extends Constructable,
  TObject extends InstanceType<TSource>,
  O extends SxiObject<TSource, TObject> = SxiObject<TSource, TObject>,
> = {
    args?: ConstructorParameters<TSource>;
    object?: O;
    visible?: boolean;
    attach?: AttachStrategy<TSource, TObject>;
  };

export type SxiObjectMetadata<TSource extends Constructable> = {
  __sxi: SxiInstance<TSource>;
}
export type SxiObject<TSource extends Constructable, TObject extends InstanceType<TSource> = InstanceType<TSource>> =
  & TObject 
  & SxiObjectMetadata<TSource>;

/**
 * Internal state for a SxiObject, stored under the object's `__sxi` iey.
 */
export type SxiInstance<
  TSource extends Constructable,
  TObject extends InstanceType<TSource> = InstanceType<TSource>,
> = {
  solixi: SxiState;
  type: string;
  parent?: SxiInstance<Constructable, unknown>;
  object: SxiObject<TSource, TObject>;
  children: SxiInstance<Constructable, unknown>[];
  props: SxiInstanceReservedProps<TSource, TObject>;
};
