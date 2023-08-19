import { Point } from "@pixi/core";
import { newUuid, Uuid, uuid } from "../utils/uuid";
import {
  CreateObjectCommand,
  DeleteObjectCommand,
  SetSceneObjectFieldCommand,
} from "./commands";
import {
  BaseSceneObject,
  CanvasSceneObject,
  GraphicSceneObject,
  GroupSceneObject,
  HasInspectSceneObject,
  NodeSceneObject,
} from "../types/scene";
import { AppDispatcher } from ".";
import { getObject, SceneModel } from "./sceneStore";
import { MultiCommand } from "./commands/shared";

export const sceneObjectDefaults = <
  TObject extends BaseSceneObject = BaseSceneObject,
>(): Omit<BaseSceneObject, "name"> => ({
  id: newUuid<TObject>(),
  visible: true,
  children: [],
  parent: uuid("root"),

  locked: false,
  shallowLocked: false,
  selected: false,
  position: new Point(),
  hovered: false,
});

export const createCanvas = (
  dispatch: AppDispatcher,
  name?: string,
  size = new Point(512, 512),
) => {
  const canvas: CanvasSceneObject = {
    ...sceneObjectDefaults(),
    type: "canvas",
    name: name ?? "Canvas",
    size,
    fill: {
      color: 0xffffff,
    },
  };
  dispatch("scene:do-command", new CreateObjectCommand(canvas));
};
