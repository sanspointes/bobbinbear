import { Point } from "pixi.js";
import { store } from "."
import { newUuid } from "../utils/uuid";
import { CanvasSceneObject, SceneObject } from "./scene";
import { CreateObjectCommand } from "./scene/commands"

export const createCanvas = (name?: string, size = new Point(512, 512)) => {
  const canvas: CanvasSceneObject = {
    id: newUuid<SceneObject>(),
    type: 'canvas',
    name: name ?? "Canvas",
    size,
    position: new Point(0, 0),
    locked: false,
    children: [],
    backgroundColor: 'white',
  };
  store.dispatch('perform-command', new CreateObjectCommand(canvas))
}
