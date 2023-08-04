/* eslint-disable solid/reactivity */
import { SetStoreFunction, createStore, produce } from "solid-js/store";
import { Uuid } from "../../utils/uuid";
import { ColorSource, Point } from "pixi.js";
import { SceneCommands } from "./commands";
import { Command } from "../commands";
import { arrayLast } from "../../utils/array";
import { StoreHandler } from "..";

export type BaseSceneObject = {
  id: Uuid<SceneObject>,
  name: string,
  position: Point,
  parent?: Uuid<SceneObject>,
  locked: boolean,
  children: SceneObject[],
}
export type GraphicSceneObject = BaseSceneObject & {
  type: 'graphic',
}
export type CanvasSceneObject = BaseSceneObject & {
  type: 'canvas',
  size: Point,
  backgroundColor: ColorSource,
}

export type SceneObject = (GraphicSceneObject | CanvasSceneObject);
export type SceneObjectType = SceneObject['type'];

export type SceneObjectPropsLookup = {
  'canvas': CanvasSceneObject,
  'graphic': GraphicSceneObject,
}

export type ObjectMapData = {
  object: SceneObject,
  set: SetStoreFunction<SceneObject>,
}

  export const traverse = (obj: SceneObject, handler: (obj: SceneObject) => void) => {
    handler(obj);
    if (obj.children) {
      for (const child of obj.children) {
        traverse(child, handler);
      }
    }
  }

export type SceneStore = {
  root: SceneObject[],
}

export const createSceneStore = (): StoreHandler<SceneCommands, SceneStore> => {
  const [store, setStore] = createStore<SceneStore>({
    root: [],
  });

  const objMap = new Map<Uuid<SceneObject>, {
    object: SceneObject,
    set: SetStoreFunction<SceneObject>,
  }>();

  return {
    store,
    performCommand: (_history, command) => {
      command.perform(store, setStore, objMap);
    },
    undoCommand: (_history, command) => {
      command.undo(store, setStore, objMap);
    }
  }
}
