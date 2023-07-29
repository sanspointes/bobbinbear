/* @jsxImportSource solid-js */
// import { render } from '@solidjs/testing-library';
import { describe, expect, it } from 'vitest';
import '@testing-library/jest-dom'; // 👈 this is imported in order to use the jest-dom matchers 
import { render } from '@solidjs/testing-library';

import { createRenderer, SxiObject } from '../src';
import { JSX } from 'solid-js/jsx-runtime';
import { onMount, splitProps } from 'solid-js';
import { SolixiRoot } from '../src/renderer';
import { Constructable } from '../src/types';

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
  children: JSX.Element | JSX.Element[],
  onCreated: (root: SolixiRoot<typeof initialState, TRootObj>) => void,
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

describe('createRenderer', () => {
  it('Should set the root object correctly', (): Promise<void> => {
    return new Promise((res) => {
      const rootObject = new ClassGraphNode(0);

      let parentNode: ClassGraphNode|undefined;
      let childNode: ClassGraphNode|undefined;

      render(() => (<BasicRoot rootObject={rootObject} onCreated={(root) => {
        expect(root.rootObject).toBe(rootObject);
        res();
      }}>
        <GraphNode ref={parentNode} args={[1]}>
          <GraphNode ref={childNode} args={[3]} />
        </GraphNode>
      </BasicRoot>
      ));
    })
  });
  it('Should set refs correctly.', (): Promise<void> => {
    return new Promise((res) => {
      const rootObject = new ClassGraphNode(0);

      let parentNode: ClassGraphNode|undefined;
      let childNode: ClassGraphNode|undefined;

      render(() => (<BasicRoot rootObject={rootObject} onCreated={() => {
        expect(parentNode).not.toBeUndefined();
        expect(childNode).not.toBeUndefined();
        res();
      }}>
        <GraphNode ref={parentNode} args={[1]}>
          <GraphNode ref={childNode} args={[3]} />
        </GraphNode>
      </BasicRoot>
      ));
    })
  });
});
