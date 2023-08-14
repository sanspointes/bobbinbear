import { Point } from "@pixi/core";
import { newUuid, Uuid, uuid } from "../utils/uuid";
import {
  CreateObjectCommand,
  DeleteObjectCommand,
  MultiCommand,
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

export const inspectGraphicsObject = (
  dispatch: AppDispatcher,
  sceneStore: SceneModel,
  id: Uuid<GraphicSceneObject>,
) => {
  const object = getObject(sceneStore, id);
  if (!object) {
    throw new Error(
      `Cannot inspect scene object ${id}.  Not found in store.`,
    );
  }
  const inspectingRootObjectId = newUuid<GroupSceneObject>();

  const nodes: NodeSceneObject[] = object.shape.map((node) => ({
    ...sceneObjectDefaults<NodeSceneObject>(),
    type: "node",
    node,
    name: `${node.type} Node`,
    position: new Point(node.x, node.y),
    parent: inspectingRootObjectId,
    relatesTo: object.id as Uuid<GraphicSceneObject>,
  }));

  const parent: GroupSceneObject = {
    type: "group",
    name: `Nodes of ${object.name}`,
    position: object.position.clone(),
    id: inspectingRootObjectId,
    hovered: false,
    selected: false,
    visible: true,
    shallowLocked: true,
    locked: false,
    parent: uuid("root"),
    children: nodes.map((c) => c.id),
  };

  const commands = [
    new CreateObjectCommand(parent, 'first'),
    ...nodes.map((so) =>
      new CreateObjectCommand(so)
    ),
    new SetSceneObjectFieldCommand(
      object.id as Uuid<GraphicSceneObject>,
      "inspectingObject",
      inspectingRootObjectId,
    ),
    new SetSceneObjectFieldCommand(
      object.id as Uuid<GraphicSceneObject>,
      "inspecting",
      true,
    ),
  ];

  // @ts-expect-error; Needs better inheritance
  const cmd = new MultiCommand(...commands);
  cmd.name = 'Inspect Graphics Object';
  // @ts-expect-error; Needs better inheritance
  dispatch("scene:do-command", cmd);
};

export const uninspectObject = (
  dispatch: AppDispatcher,
  store: SceneModel,
  id: Uuid<BaseSceneObject & HasInspectSceneObject>,
) => {
  const obj = getObject(store, id);
  const inspectRoot = getObject(store, obj?.inspectingObject);

  if (inspectRoot) {
    const cmd = new DeleteObjectCommand(inspectRoot.id);
    dispatch("scene:do-command", cmd);
  }
};
