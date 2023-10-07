import { Point } from '@pixi/core';
import { ILineStyleOptions } from '@pixi/graphics';
import { Command } from '../store/commands';
import { HslColor } from '../utils/color';
import { Uuid } from '../utils/uuid';

export type EmbStatePersistable = {
    /** Internal locking used for blocking the user from interacting with this element (but not children) */
    shallowLocked: boolean;
    /** Selected state */
    selected: boolean;
    /** User controlled States */
    /** Whether the scene object is visible */
    visible: boolean;
    /** User-displaying name of object */
    name: string;
    /** User controls locking, disables interacitivity */
    locked: boolean;
};

export type EmbState = EmbStatePersistable & {
    /** Hover state */
    hovered: boolean;
    /** Is this element inspecting */
    inspecting: boolean;

    /** Whether or not the object can be moved. */
    disableMove: boolean;
};
export const EMB_STATE_DEFAULTS: EmbState = {
    hovered: false,
    selected: false,
    shallowLocked: false,
    disableMove: false,
    inspecting: false,
    visible: true,
    name: 'Object',
    locked: false,
};

export type EmbBase = EmbState & {
    /** X-Y position of object */
    position: Point;
    /** Optional parent, if no parent provided, it is at the top level. */
    parent: Uuid;
    /** Children ids */
    children: Uuid[];
};

/**
 * Partials / Fragments
 */

export type EmbHasVirtual = {
    virtual: true;
    virtualCreator: () => Command;
};

export type FillOptions = {
    color: HslColor;
};
export type EmbHasFill = {
    fill: FillOptions;
};

export type LineOptions = Omit<
    ILineStyleOptions,
    'color' | 'alpha' | 'texture' | 'matrix'
> & {
    color: HslColor;
};
export type EmbHasLine = {
    line: LineOptions;
};
export type EmbHasInspecting = {
    inspecting: boolean;
};
