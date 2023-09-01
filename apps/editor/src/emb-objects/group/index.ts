import { Uuid } from "../../utils/uuid";
import { EmbBase } from "../shared";

/**
 * GROUP SCENE OBJECT
 */
export type EmbGroup = EmbBase & {
  /** Internal States */
  /** Unique ID for each scene object */
  id: Uuid<EmbGroup>;

  type: "group";
};

export * from './EmbGroup';
