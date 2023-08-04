import { Point } from "pixi.js";
import { newUuid } from "../utils/uuid";
import { dispatch } from ".";
import { CreateObjectCommand } from "./commands/object";
import { CanvasSceneObject, SceneObject } from "../types/scene";

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
  dispatch('scene:do-command', new CreateObjectCommand(canvas))
}
