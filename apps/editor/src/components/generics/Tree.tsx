import {
  createContext,
  createMemo,
  For,
  type JSX,
  Show,
  splitProps,
  useContext,
  createSignal,
  mergeProps,
  Accessor,
} from "solid-js";
import {
  createDraggable,
  createDroppable,
  DragEventHandler,
  useDragDropContext,
  Droppable,
  Id as DndId,
} from "@thisbeyond/solid-dnd";
import { arrayLast } from "../../utils/array";

declare module "solid-js" {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace JSX {
    interface Directives { // use:model
      draggable: ReturnType<typeof createDraggable>;
      droppable: ReturnType<typeof createDroppable>;
    }
  }
}

/*
 * Base Drag Drop type must have a unique ID
 */
type BaseDragDroppable = {
  id: string;
};

/*
 * Context shared down elements within tree
 */
type TreeContextModel<T> = {
  isDroppablePredicate: (node: T) => boolean,
  childResolver: (node: T) => T[];
  nodeTemplate: (node: T, children: () => JSX.Element) => JSX.Element;
  droppableTemplate: (active: boolean) => JSX.Element;
  currentlyDragging: Accessor<DndId>,
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const TreeContext = createContext<TreeContextModel<any>>(
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  null as unknown as TreeContextModel<any>,
);

/*
 * Root Component, should be able to implement entire tree with props
 */
type TreeProps<T extends BaseDragDroppable> = {
  root: T;
  isDroppablePredicate: (node: T) => boolean,
  childResolver: (node: T) => T[];
  nodeTemplate: (node: T, children: () => JSX.Element) => JSX.Element;
  droppableTemplate: (active: boolean) => JSX.Element;

  onDragEnd?: DragEventHandler;
};
export function Tree<T extends BaseDragDroppable>(props: TreeProps<T>) {
  const [, { onDragEnd, onDragStart }] = useDragDropContext()!;

  const [currentlyDragging, setCurrentlyDragging] = createSignal<DndId>();
  onDragStart((ev) => {
    setCurrentlyDragging(ev.draggable.id);
  })
  onDragEnd(() => {
    setCurrentlyDragging(undefined);
  })

  // eslint-disable-next-line solid/reactivity
  if (props.onDragEnd) onDragEnd(props.onDragEnd);

  const [contextProps] = splitProps(props, [
    "isDroppablePredicate",
    "droppableTemplate",
    "nodeTemplate",
    "childResolver",
  ]);
  const contextData = mergeProps(contextProps, {
    currentlyDragging,
  })

  return (
    <TreeContext.Provider
      value={contextData}
    >
      <TreeChildren children={props.childResolver(props.root)} />
    </TreeContext.Provider>
  );
}

/*
 * Each tree node should be able to be rendered by itself
 */
type TreeNodeProps<T extends BaseDragDroppable> = {
  node: T;
};
function TreeNode<T extends BaseDragDroppable>(props: TreeNodeProps<T>) {
  const treeCtx = useContext(TreeContext);

  // eslint-disable-next-line solid/reactivity
  const draggable = createDraggable(props.node.id, props.node);
  // eslint-disable-next-line solid/reactivity
  const droppable = treeCtx.isDroppablePredicate(props.node) ? createDroppable(props.node.id, props.node) : (() => { return undefined }) as unknown as ReturnType<typeof createDroppable>;


  return (
    <div use:draggable={draggable} use:droppable={treeCtx.currentlyDragging() !== props.node.id ? droppable : undefined}>
      {treeCtx.nodeTemplate(
        props.node,
        () => <TreeChildren children={treeCtx.childResolver(props.node)} />,
      )}
    </div>
  );
}

/**
 * TreeDropable exists before/after nodes so that they can be moved around
 */
enum RelatedNodePosition {
  After = "after",
  Before = "before",
}
type TreeDropableProps<T extends BaseDragDroppable> = {
  after: T;
  before?: undefined;
} | {
  after?: undefined;
  before: T;
};
function TreeDroppableInbetween<T extends BaseDragDroppable>(
  props: TreeDropableProps<T>,
) {
  const [state] = useDragDropContext()!;
  const treeCtx = useContext(TreeContext);
  const relatedNodePosition = createMemo(() => {
    if (props.before) return RelatedNodePosition.Before;
    else if (props.after) return RelatedNodePosition.After;
  });
  const node = createMemo<T>(() => {
    return props.before ?? props.after as T;
  });
  const id = createMemo(() => `${relatedNodePosition()}-${node().id}`);
  // eslint-disable-next-line solid/reactivity
  const droppable = createDroppable(id(), node());

  return (
    <div use:droppable={droppable}>
      {treeCtx.droppableTemplate(state.active.droppable?.id === id())}
    </div>
  );
}

/**
 * Tree children, renders the children of a node
 */
type TreeChildrenProps<T extends BaseDragDroppable> = {
  children: T[];
};
function TreeChildren<T extends BaseDragDroppable>(
  props: TreeChildrenProps<T>,
) {
  return (
    <>
      <For each={props.children}>
        {(child) => (
          <>
            <TreeDroppableInbetween before={child} />
            <TreeNode node={child} />
          </>
        )}
      </For>
      <Show when={arrayLast(props.children)}>
        {(last) => <TreeDroppableInbetween after={last()} />}
      </Show>
    </>
  );
}
