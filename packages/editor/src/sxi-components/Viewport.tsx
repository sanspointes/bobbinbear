import { Solixi } from '@bearbroidery/solixi'
import { Viewport as PixiViewport, IViewportOptions, Drag, Pinch } from 'pixi-viewport'
import { useContext, type JSX } from 'solid-js';
import { createMemo } from 'solid-js';
import { Cursor } from '../store/toolStore';
import { AppContext } from '../store';
import { Container } from '@pixi/display';

const PViewport = Solixi.wrapConstructable(PixiViewport, {
  attach: (_, parent: Container, object) => {
    parent.addChild(object);
    return () => parent.removeChild(object);
  },
  defaultArgs: (ctx) => [{
    events: ctx.app.renderer.events,
    ticker: ctx.ticker,
  }] as [options: IViewportOptions],
  extraProps: {
    pinch: (_1, _2, object, enable: boolean) => {
      const pinchPlugin = new Pinch(object);
      if (enable) {
        object.plugins.add('pinch', pinchPlugin);
      } else {
        object.plugins.remove('pinch');
      }
      return () => {
        if (object.plugins.list.includes(pinchPlugin)) object.plugins.remove('pinch');
      }
    },
    drag: (_1, _2, object, enable: boolean) => {
      const dragPlugin = new Drag(object);
      if (enable) {
        object.plugins.add('drag', dragPlugin);
      } else {
        object.plugins.remove('drag');
      }
      return () => {
        if (object.plugins.list.includes(dragPlugin)) object.plugins.remove('drag');
      }
    },
  },
})

type ViewportProps = {
  children: JSX.Element
}
export const Viewport = (props: ViewportProps) => {
  const { toolStore } = useContext(AppContext);
  const viewportPaused = createMemo(() => {
    const { Grab, Grabbing} = Cursor;
    const currentTool = toolStore.currentCursor;
    return currentTool !== Grab && currentTool !== Grabbing;
  })
  return <PViewport pause={viewportPaused} drag pinch children={props.children} />
}
