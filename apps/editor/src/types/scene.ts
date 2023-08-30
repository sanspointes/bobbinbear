import { Point } from "@pixi/core";
import { Uuid } from "../utils/uuid";
import { IFillStyleOptions, ILineStyleOptions } from "@pixi/graphics";
import { Command } from "../store/commands";

export type EmbBase = {
  /** Internal States */
  /** Unique ID for each scene object */
  id: Uuid<EmbBase>;
  /** Internal locking used for blocking the user from interacting with this element (but not children) */
  shallowLocked: boolean;
  /** Hover state */
  hovered: boolean;
  /** Selected state */
  selected: boolean;

  /** User controlled States */
  /** Whether the scene object is visible */
  visible: boolean;
  /** User-displaying name of object */
  name: string;
  /** X-Y position of object */
  position: Point;
  /** Optional parent, if no parent provided, it is at the top level. */
  parent: Uuid<EmbBase>;
  /** User controls locking, disables interacitivity */
  locked: boolean;
  /** Children ids */
  children: Uuid<EmbBase>[];
};

export type EmbHasVirtual = {
  virtual: true,
  virtualCreator: <T extends EmbBase>() => Command<T>,
}

export type EmbHasFill = {
  fill: IFillStyleOptions;
};

export type EmbHasStroke = {
  stroke: ILineStyleOptions;
};
export type EmbHasInspecting = {
  inspecting: boolean;
};

/**
 * GRAPHICS SCENE OBJECT
 */
export enum EmbNodeType {
  Jump = 0,
  Control = 1,
  Point = 2,
}

type NodeBase = {
  x: number;
  y: number;
}
export type NodePoint = NodeBase & {
  id: Uuid<NodePoint>,
  type: EmbNodeType.Point;
  ownsNext?: true;
  ownsPrev?: true;
  isControlPaired?: true;
}
export type NodePointVirtual = NodeBase & {
  id: Uuid<NodePointVirtual>,
  virtual: true,
  type: EmbNodeType.Point;
  after: Uuid<VectorNode>;
  close: never;
}
export type NodeControl = NodeBase & {
  id: Uuid<NodeControl>,
  type: EmbNodeType.Control;
}
export type NodeJump = NodeBase & {
  id: Uuid<NodeJump>,
  type: EmbNodeType.Jump,
}

export type VectorNode = NodePoint | NodePointVirtual | NodeControl | NodeJump;

export type EmbVector =
  & EmbBase
  & EmbHasFill
  & EmbHasStroke
  & EmbHasInspecting
  & {
    /** Internal States */
    /** Unique ID for each scene object */
    id: Uuid<EmbVector>;

    type: "graphic";
    shape: VectorNode[];
    close: boolean;
  };
/**
 * NODE SCENE OBJECT
 */
export type EmbNode = EmbBase & {
  /** Internal States */
  /** Unique ID for each scene object */
  id: Uuid<EmbNode>;
  type: "node";
  node: VectorNode;
  /** The uuid this node object is bound to (i.e. makes up part of a GraphicSceneObject path) */
  relatesTo: Uuid<EmbVector>;

  data: number[];
};
/**
 * CANVAS SCENE OBJECT
 */
export type EmbCanvas = EmbBase & EmbHasFill & {
  /** Internal States */
  /** Unique ID for each scene object */
  id: Uuid<EmbCanvas>;

  type: "canvas";
  size: Point;
};

/**
 * GROUP SCENE OBJECT
 */
export type EmbGroup = EmbBase & {
  /** Internal States */
  /** Unique ID for each scene object */
  id: Uuid<EmbCanvas>;

  type: "group";
};

export type EmbObject =
  (EmbVector
  | EmbCanvas
  | EmbNode
  | EmbGroup) & EmbHasVirtual;
export type EmbObjectType = EmbObject["type"];

export type SceneObjectPropsLookup = {
  "canvas": EmbCanvas;
  "graphic": EmbVector;
  "group": EmbGroup;
  "node": EmbNode;
};
