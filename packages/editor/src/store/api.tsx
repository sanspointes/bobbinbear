import { Point } from "@pixi/core";
import { newUuid, uuid } from "../utils/uuid";
import { CreateObjectCommand } from "./commands/object";
import { CanvasSceneObject, SceneObject } from "../types/scene";
import { AppDispatcher } from ".";

export const createCanvas = (dispatch: AppDispatcher, name?: string, size = new Point(512, 512)) => {
  const canvas: CanvasSceneObject = {
    id: newUuid<SceneObject>(),
    type: 'canvas',
    name: name ?? "Canvas",
    size,
    position: new Point(0, 0),
    locked: false,
    children: [],
    visible: true,
    parent: uuid('root'),
    shallowLocked: false,
    fill: {
      color: 0xffffff,
    },
    selected: false,
    hovered: false,
  };
  dispatch('scene:do-command', new CreateObjectCommand(canvas))
}
