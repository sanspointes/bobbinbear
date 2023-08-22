import { Point } from "@pixi/core";
import { newUuid, Uuid, uuid } from "../utils/uuid";
import {
  Command,
  CreateObjectCommand,
  MutateSceneObjectArrayFieldCommand,
} from "./commands";
import {
  BaseSceneObject,
  CanvasSceneObject,
  GraphicNodeTypes,
  GraphicSceneObject,
  GraphicsNode,
  NodeSceneObject,
} from "../types/scene";
import { AppDispatcher } from ".";
import { SceneModel } from "./sceneStore";
import { arrayGetOffset } from "../utils/array";
import { MultiCommand } from "./commands/shared";
import { lerp } from "../utils/math";

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

export const tryMakeGraphicsNodeACurve = (
  dispatch: AppDispatcher,
  store: SceneModel,
  nodeId: Uuid<GraphicsNode>,
) => {
  const obj = store.objects.get(nodeId) as NodeSceneObject;
  if (!obj) {
    console.warn(`tryMakeGraphicsNodeACurve: Can't get object ${nodeId}.`);
    return false;
  }
  const graphics = store.objects.get(obj.relatesTo) as GraphicSceneObject;
  if (!graphics) {
    console.warn(
      `tryMakeGraphicsNodeACurve: Can't get related graphics object ${obj.relatesTo}.`,
    );
    return false;
  }
  const nodeIndex = graphics.shape.findIndex((n) => n.id === nodeId);
  if (nodeIndex === -1) {
    console.warn(
      `tryMakeGraphicsNodeACurve: Can't find node (${obj.id}) position in related graphics object ${obj.relatesTo}.`,
    );
    return false;
  }
  const before2 = arrayGetOffset(graphics.shape, nodeIndex, -2, true);
  const before1 = arrayGetOffset(graphics.shape, nodeIndex, -1, true);
  const after1 = arrayGetOffset(graphics.shape, nodeIndex, 1, true);
  const after2 = arrayGetOffset(graphics.shape, nodeIndex, 1, true);

  const cmds: Command[] = [];

  const canInsertAfter = after1.type !== GraphicNodeTypes.Control &&
    after2.type !== GraphicNodeTypes.Control;
  if (canInsertAfter) {
    const id1 = newUuid<GraphicsNode>();
    const control1: GraphicsNode = {
      id: id1,
      type: GraphicNodeTypes.Control,
      x: lerp(after2.x, after1.x, 1.25),
      y: lerp(after2.y, after1.y, 1.25),
    };
    const id2 = newUuid<GraphicsNode>();
    const control2: GraphicsNode = {
      id: id2,
      type: GraphicNodeTypes.Control,
      x: lerp(after1.x, obj.node.x, 0.75),
      y: lerp(after1.y, obj.node.y, 0.75),
    };
    cmds.push(
      new MutateSceneObjectArrayFieldCommand(
        obj.relatesTo,
        "shape",
        nodeIndex+1,
        0,
        [control2, control1],
      ),
    );
  }

  const canInsertBefore = before1.type !== GraphicNodeTypes.Control &&
    before2.type !== GraphicNodeTypes.Control;
  if (canInsertBefore) {
    const id1 = newUuid<GraphicsNode>();
    const control1: GraphicsNode = {
      id: id1,
      type: GraphicNodeTypes.Control,
      x: lerp(before2.x, before1.x, 1.25),
      y: lerp(before2.y, before1.y, 1.25),
    };
    const id2 = newUuid<GraphicsNode>();
    const control2: GraphicsNode = {
      id: id2,
      type: GraphicNodeTypes.Control,
      x: lerp(before1.x, obj.node.x, 0.75),
      y: lerp(before1.y, obj.node.y, 0.75),
    };
    cmds.push(
      new MutateSceneObjectArrayFieldCommand(
        obj.relatesTo,
        "shape",
        nodeIndex,
        0,
        [control1, control2],
      ),
    );
  }

  if (cmds.length) {
    dispatch('scene:do-command', new MultiCommand(...cmds));
  }
};
