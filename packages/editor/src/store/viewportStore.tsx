import { Point } from "@pixi/core"
import { Viewport } from "pixi-viewport"
import { AllMessages, GeneralHandler, generateStore } from "."
import { produce } from "solid-js/store"

export type ViewportMessage = {
  'viewport:move-to': Point,
  'viewport:set-bounds': {
    left: number,
    right: number,
    top: number,
    bottom: number,
  }
}

export type ViewportModel = {
  viewportRef: Viewport|undefined,
  position: Point,
  left: number,
  right: number,
  top: number,
  bottom: number,
}

export const createViewportStore = (dispatch: GeneralHandler<AllMessages>) => {
  const model: ViewportModel = {
    viewportRef: undefined,
    position: new Point(),
    left: 0,
    right: 0,
    top: 0,
    bottom: 0,
  }

  const viewportStore = generateStore<ViewportModel, ViewportMessage>(model, {
    'viewport:move-to': (_store, set, point, _response) => {
      set(produce(store => store.position.copyFrom(point)));
    },
    'viewport:set-bounds': (_store, set, bounds, _respond) => {
      set(produce(store => {
        store.left = bounds.left;
        store.right = bounds.right;
        store.top = bounds.top;
        store.bottom = bounds.bottom;
      }))
    }
  })

  return viewportStore;
}
