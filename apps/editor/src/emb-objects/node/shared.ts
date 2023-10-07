import { Uuid, newUuid } from '../../utils/uuid';
import { EmbBase } from '../shared';

export enum VectorNodeType {
    Control = 0,
    Point = 1,
}

type NodeBase = {
    x: number;
    y: number;
};
export type NodePoint = NodeBase & {
    id: Uuid;
    type: VectorNodeType.Point;
    ownsNext?: true;
    ownsPrev?: true;
};
export type NodePointVirtual = NodeBase & {
    id: Uuid;
    type: VectorNodeType.Point;
    virtual: true;
};
export type NodeControl = NodeBase & {
    id: Uuid;
    type: VectorNodeType.Control;
};

export type NodeVirtual = NodeBase & {
    virtual: true;
};

export type VectorNode = NodePoint | NodePointVirtual | NodeControl;
/**
 * NODE SCENE OBJECT
 */
export type EmbNode = EmbBase & {
    /** Internal States */
    /** Unique ID for each scene object */
    id: Uuid;
    type: 'node';
    node: VectorNode;
    /** The uuid this node object is bound to (i.e. makes up part of a GraphicSceneObject path) */
    relatesTo: Uuid;
};

export const isNodePoint = (node: VectorNode): node is NodePoint => {
    return node.type === VectorNodeType.Point;
};
export const isNodePointVirtual = (
    node: VectorNode,
): node is NodePointVirtual => {
    return (
        node.type === VectorNodeType.Point && (node as NodePointVirtual).virtual
    );
};
export const isNodeControl = (node: VectorNode): node is NodeControl => {
    return node.type === VectorNodeType.Control;
};

export const NodeUtils = {
    newControl(x: number, y: number): NodeControl {
        return {
            id: newUuid(),
            type: VectorNodeType.Control,
            x,
            y,
        };
    },
    newPoint(x: number, y: number): NodePoint {
        return {
            id: newUuid(),
            type: VectorNodeType.Point,
            x,
            y,
        };
    },
};
