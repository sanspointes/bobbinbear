import { Uuid } from "../../utils/uuid";
import { VectorNode } from "../node";
import {
  EmbBase,
  EmbHasFill,
  EmbHasInspecting,
  EmbHasStroke,
} from "../shared";

export * from "./EmbVector";

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
