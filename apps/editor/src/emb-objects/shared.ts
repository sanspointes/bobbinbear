import { Point } from "@pixi/core";
import { Uuid } from "../utils/uuid";
import { IFillStyleOptions, ILineStyleOptions } from "@pixi/graphics";
import { Command } from "../store/commands";

export type EmbBase = {
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

/**
 * Partials / Fragments
 */

export type EmbHasVirtual = {
  virtual: true,
  virtualCreator: () => Command,
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
