import { Point } from "@pixi/core";
import { newUuid, Uuid, uuid } from "../utils/uuid";
import {
  Command,
  CreateObjectCommand,
  MutateSceneObjectArrayFieldCommand,
  SetSceneObjectFieldCommand,
} from "./commands";
import { EmbState } from "../emb-objects/shared";
import { AppDispatcher } from ".";
import { SceneModel } from "./sceneStore";
import {
  arrayFindFromBackwardsCircular,
  arrayFindFromCircular,
} from "../utils/array";
import { assertDefined, MultiCommand } from "./commands/shared";
import { addPoint, lerpPoint, subPoint } from "../utils/math";
import { EmbCanvas, EmbNode, VectorNodeType, EmbVector, isNodePointVirtual, VectorNode } from "../emb-objects";
import { isEmbNode } from "../emb-objects/utils";
import { EMB_STATE_DEFAULTS } from "../emb-objects/shared";
import { hslFromRgb } from "../utils/color";

export const createCanvas = (
  dispatch: AppDispatcher,
  name?: string,
  size = new Point(512, 512),
) => {
  const canvas: EmbCanvas & EmbState = {
    id: newUuid<EmbCanvas>(),
    children: [],
    parent: uuid("root"),
    position: new Point(),
    ...EMB_STATE_DEFAULTS,
    type: "canvas",
    name: name ?? "Canvas",
    size,
    fill: {
      color: hslFromRgb({ r: 255, g: 255, b: 255 }),
    },
  };
  dispatch("scene:do-command", new CreateObjectCommand(canvas));
};

export const tryMakeGraphicsNodeACurve = (
  dispatch: AppDispatcher,
  store: SceneModel,
  nodeId: Uuid<EmbNode>,
) => {
  const obj = store.objects.get(nodeId);
  if (!assertDefined("tryMakeGraphicsNodeACurve", obj, "NodeSceneObject")) {
    return;
  }
  if (!isEmbNode(obj)) return;
  const node = obj.node; 

  if (!isNodePointVirtual(node)) return;

  const graphics = store.objects.get(obj.relatesTo) as EmbVector;
  if (
    !assertDefined(
      "tryMakeGraphicsNodeACurve",
      graphics,
      "Related graphics object",
    )
  ) return;
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
    (el) => el.type === VectorNodeType.Point,
  );
  const nextPoint = arrayFindFromCircular(
    graphics.shape,
    nodeIndex + 1,
    (el) => el.type === VectorNodeType.Point,
  );

  const cmds: Command<EmbVector>[] = [];

  if (!prevPoint || !nextPoint) return;

  const { ownsPrev, ownsNext } = node;
  // Insert control nodes after node
  if (!ownsNext) {
    const newPosition = new Point();
    lerpPoint(prevPoint, nextPoint, 1.35, newPosition);
    subPoint(newPosition, nextPoint, newPosition);
    addPoint(obj.node, newPosition, newPosition);

    const id1 = newUuid<VectorNode>();
    const control1: VectorNode = {
      id: id1,
      type: VectorNodeType.Control,
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

    const id1 = newUuid<VectorNode>();
    const control1: VectorNode = {
      id: id1,
      type: VectorNodeType.Control,
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
    const updatedNodeData = { ...obj.node } as RealNode;
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
