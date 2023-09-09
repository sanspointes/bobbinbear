import { EmbCanvas } from './canvas';
import { EmbGroup } from './group';
import { EmbNode } from './node';
import { EmbBase } from './shared';
import { EmbVecSeg } from './vec-seg';
import { EmbVector } from './vector';

export const isEmbNode = (object: EmbBase): object is EmbNode => {
    return (object as EmbNode).type === 'node';
};

export const isEmbVector = (object: EmbBase): object is EmbVector => {
    return (object as EmbVector).type === 'vector';
};

export const isEmbVecSeg = (object: EmbBase): object is EmbVecSeg => {
    return (object as EmbVecSeg).type === 'vec-seg';
};

export const isEmbGroup = (object: EmbBase): object is EmbGroup => {
    return (object as EmbGroup).type === 'group';
};

export const isEmbCanvas = (object: EmbBase): object is EmbCanvas => {
    return (object as EmbCanvas).type === 'canvas';
};
