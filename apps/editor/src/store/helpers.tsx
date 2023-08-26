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
import {
  arrayFindFromBackwardsCircular,
  arrayFindFromCircular,
  arrayGetCircular,
  arrayGetOffset,
  arrayOffsetIterCircular,
} from "../utils/array";
import { assertDefined, MultiCommand } from "./commands/shared";
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
  if (!assertDefined('tryMakeGraphicsNodeACurve', obj, 'NodeSceneObject')) return;
  const node = obj.node as BasicGraphicsNode;
  const graphics = store.objects.get(obj.relatesTo) as GraphicSceneObject;
  if (!assertDefined('tryMakeGraphicsNodeACurve', graphics, 'Related graphics object')) return;
  const nodeIndex = graphics.shape.findIndex((n) => n.id === nodeId);
  if (nodeIndex === -1) {
    console.warn(
      `tryMakeGraphicsNodeACurve: Can't find node (${obj.id}) position in related graphics object ${obj.relatesTo}.`,
    );
    return false;
  }
  const prevPoint = arrayFindFromBackwardsCircular(
    graphics.shape,
    nodeIndex - 1,
    (el) => el.type === GraphicNodeTypes.Point,
  );
  const nextPoint = arrayFindFromCircular(graphics.shape, nodeIndex + 1, (el) => el.type === GraphicNodeTypes.Point);

  const cmds: Command<GraphicSceneObject>[] = [];

  if (!prevPoint || !nextPoint) return;

  const { ownsPrev, ownsNext } = node;
  // Insert control nodes after node
  if (!ownsNext) {
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
        "shape",
        nodeIndex + 1,
        {
          toDelete: 0,
          toInsert: [control1],
          circularInsert: true,
        },
      ),
    );
  }

  if (!ownsPrev) {
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
        },
      ),
    );
  }

  if (!ownsPrev || !ownsNext) {
    const updatedNodeData = { ...obj.node } as BasicGraphicsNode;
    if (!ownsPrev) updatedNodeData.ownsPrev = true;
    if (!ownsNext) updatedNodeData.ownsNext = true;
    if (!ownsNext && !ownsPrev) {
      updatedNodeData.isControlPaired = true;
    }
    cmds.push(new SetSceneObjectFieldCommand(obj.id, "node", updatedNodeData));
  }

  if (cmds.length) {
    dispatch("scene:do-command", new MultiCommand(...cmds));
  }
};
