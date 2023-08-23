import { Point } from "@pixi/core";
import { newUuid, Uuid, uuid } from "../utils/uuid";
import {
  Command,
  CreateObjectCommand,
  MutateSceneObjectArrayFieldCommand,
  SetSceneObjectFieldCommand,
} from "./commands";
import {
  BaseSceneObject,
  BasicGraphicsNode,
  CanvasSceneObject,
  GraphicNodeTypes,
  GraphicSceneObject,
  GraphicsNode,
  NodeSceneObject,
} from "../types/scene";
import { AppDispatcher } from ".";
import { SceneModel } from "./sceneStore";
import { arrayGetOffset, arrayOffsetIterCircular } from "../utils/array";
import { MultiCommand } from "./commands/shared";
import { addPoint, lerpPoint, subPoint } from "../utils/math";
import { iterFind } from "../utils/iter";

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
  nodeId: Uuid<NodeSceneObject & GraphicsNode>,
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
  const backIter = arrayOffsetIterCircular(graphics.shape, nodeIndex - 1, -1);
  const prevPoint = iterFind(
    backIter,
    (el) => {
      if (el === undefined) debugger;
      return el.type === GraphicNodeTypes.Point;
    },
  );
  const before2 = arrayGetOffset(graphics.shape, nodeIndex, -2, true);
  const before1 = arrayGetOffset(graphics.shape, nodeIndex, -1, true);
  const forwardIter = arrayOffsetIterCircular(graphics.shape, nodeIndex + 1);
  const nextPoint = iterFind(
    forwardIter,
    (el) => {
      if (el === undefined) debugger;
      return el.type === GraphicNodeTypes.Point;
    },
  );

  const cmds: Command<GraphicSceneObject>[] = [];

  if (!prevPoint || !nextPoint) return;

  const after1 = arrayGetOffset(graphics.shape, nodeIndex, 1, true);
  const after2 = arrayGetOffset(graphics.shape, nodeIndex, 2, true);
  // Insert control nodes after node
  const canInsertAfter = after1.type !== GraphicNodeTypes.Control ||
    after2.type !== GraphicNodeTypes.Control;
  if (canInsertAfter) {
    const newPosition = new Point();
    lerpPoint(prevPoint, nextPoint, 1.35, newPosition);
    subPoint(newPosition, nextPoint, newPosition);
    addPoint(obj.node, newPosition, newPosition);

    const id1 = newUuid<GraphicsNode>();
    const control1: GraphicsNode = {
      id: id1,
      type: GraphicNodeTypes.Control,
      x: newPosition.x,
      y: newPosition.y,
    };
    cmds.push(
      new MutateSceneObjectArrayFieldCommand(
        obj.relatesTo,
        'shape',
        nodeIndex + 1,
        {
          toDelete: 0,
          toInsert: [control1],
          circularInsert: true,
        }
      ),
    );
  }

  const canInsertBefore = before1.type !== GraphicNodeTypes.Control ||
    before2.type !== GraphicNodeTypes.Control;
  if (canInsertBefore) {
    const newPosition = new Point();
    lerpPoint(nextPoint, prevPoint, 1.35, newPosition);
    subPoint(newPosition, prevPoint, newPosition);
    addPoint(obj.node, newPosition, newPosition);

    const id1 = newUuid<GraphicsNode>();
    const control1: GraphicsNode = {
      id: id1,
      type: GraphicNodeTypes.Control,
      x: newPosition.x,
      y: newPosition.y,
    };
    cmds.push(
      new MutateSceneObjectArrayFieldCommand(
        obj.relatesTo,
        "shape",
        nodeIndex === 0 ? -1 : nodeIndex,
        {
          toDelete: 0,
          toInsert: [control1],
          circularInsert: true,
        }
      ),
    );
  }

  if (canInsertBefore || canInsertAfter) {
    const updatedNodeData = {...obj.node} as BasicGraphicsNode;
    if (canInsertBefore) updatedNodeData.ownsPrev = true;
    if (canInsertAfter) updatedNodeData.ownsNext = true;
    cmds.push(new SetSceneObjectFieldCommand(obj.id, 'node', updatedNodeData));
  }

  if (cmds.length) {
    dispatch("scene:do-command", new MultiCommand(...cmds));
  }
};
