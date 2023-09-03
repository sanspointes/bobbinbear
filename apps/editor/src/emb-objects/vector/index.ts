import { Uuid } from "../../utils/uuid";
import {
  EmbBase,
  EmbHasFill,
  EmbHasInspecting,
  EmbHasLine,
} from "../shared";
import { VectorSegment } from "../vec-seg";

export * from "./EmbVector";

export type EmbVector =
  & EmbBase
  & EmbHasFill
  & EmbHasLine
  & EmbHasInspecting
  & {
    /** Internal States */
    /** Unique ID for each scene object */
    id: Uuid<EmbVector>;

    type: "vector";
    segments: VectorSegment[];
    close: boolean;
  };
