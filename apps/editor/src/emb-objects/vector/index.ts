import { Uuid } from "../../utils/uuid";
import {
  EmbBase,
  EmbHasFill,
  EmbHasInspecting,
  EmbHasStroke,
} from "../shared";
import { VectorSegment } from "../vec-seg";

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
    segments: VectorSegment[];
    close: boolean;
  };
