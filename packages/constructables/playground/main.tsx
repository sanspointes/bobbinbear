/* @jsxImportSource solid-js */
import { createRenderer, SxiObject } from '../src';
import { JSX } from 'solid-js/jsx-runtime';
import { onMount, splitProps } from 'solid-js';
import { SolixiRoot } from '../src/renderer';
import { Constructable } from '../src/types';
import { render } from 'solid-js/web';

class ClassGraphNode {
  public id: number;
  constructor(id: number) {
    console.log(`Constructing ClassGraphNode(id: ${id})`);
    this.id = id;
  }
  children: ClassGraphNode[];
  addChild(child: ClassGraphNode) {
    this.children.push(child);
  }
  removeChild(child: ClassGraphNode) {
    const childIndex = this.children.findIndex(c => c === child);
    if (childIndex >= 0) {
      this.children.splice(childIndex, 1);
    }
  }
}

const initialState = {
  mountedNodes: new Set<number>(),
}

const {
  createRoot,
  wrapConstructable,
} = createRenderer(initialState);

const GraphNode = wrapConstructable(ClassGraphNode, {
  defaultArgs: [0],
  attach: (state, parent: SxiObject<typeof initialState, Constructable>, child) => {
    parent.addChild(child);
    state.mountedNodes.add(child.id);
    return () => { 
      parent.removeChild(child);
      state.mountedNodes.delete(child.id);
    }
  },
  extraProps: {},
})

type BasicRootProps<TRootObj> = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  rootObject: TRootObj,
  children: JSX.Element | null,
  onCreated?: (root: SolixiRoot<typeof initialState, TRootObj>) => void,
}
const BasicRoot = <TRootObj,>(props: BasicRootProps<TRootObj>) => {
  onMount(() => {
    const [propsWithChildren, rootProps] = splitProps(props, ['children']);
    // eslint-disable-next-line solid/reactivity
    const root = createRoot(rootProps.rootObject)

    root.render(propsWithChildren)

    // eslint-disable-next-line solid/reactivity
    if (rootProps.onCreated) rootProps.onCreated(root);
  })
  return <div>Root</div>
}

const App = () => {
  let graphNode1: ClassGraphNode|undefined;

  onMount(() => {
    console.log('graphNode1: ', graphNode1);
  })
  return (
    <BasicRoot rootObject={new ClassGraphNode(0)}>
      <GraphNode ref={graphNode1} args={[1]}>
        <GraphNode args={[2]} />
      </GraphNode>
    </BasicRoot> 
  );
}

render(() => (<App />), document.body);
