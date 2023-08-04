import { Solixi } from '@bearbroidery/solixi'
import { Viewport as PixiViewport, IViewportOptions, Drag, Pinch, Decelerate } from 'pixi-viewport'

export const Viewport = Solixi.wrapConstructable(PixiViewport, {
  attach: (_, parent, object) => {
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
    }
  },
})
