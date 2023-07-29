/* @jsxImportSource solid-js */
// import { render } from '@solidjs/testing-library';
import { describe, expect, it } from 'vitest';
import '@testing-library/jest-dom'; // 👈 this is imported in order to use the jest-dom matchers 
import { render } from '@solidjs/testing-library';

import { createRenderer, SxiObject } from '../src';
import { JSX } from 'solid-js/jsx-runtime';
import { onMount, splitProps } from 'solid-js';
import { SolixiRoot } from '../src/renderer';

class ClassGraphNode {
  public id: number;
  constructor(id: number) {
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
  attach: (state: typeof initialState, parent: SxiObject<typeof ClassGraphNode, typeof initialState>, child) => {
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
  children: JSX.Element | JSX.Element[],
  onCreated: (root: SolixiRoot<TRootObj, typeof initialState>) => void,
}
const BasicRoot = <TRootObj,>(props: BasicRootProps<TRootObj>) => {
  onMount(() => {
    const [propsWithChildren, rootProps] = splitProps(props, ['children']);
    // eslint-disable-next-line solid/reactivity
    const root = createRoot(rootProps.rootObject)

    root.render(propsWithChildren)

    // eslint-disable-next-line solid/reactivity
    rootProps.onCreated(root);
  })
  return <div>Root</div>
}

describe('App', () => {
  it('should render the app', (): Promise<void> => {
    return new Promise((res) => {
      render(() => (<BasicRoot rootObject={new ClassGraphNode(0)} onCreated={(root) => {
        expect(root.state.mountedNodes.size).toBe(2);
        res();
      }}>
        <GraphNode args={[1]}>
          <GraphNode args={[2]} />
        </GraphNode>
      </BasicRoot>
      ));
    })
  });
});
