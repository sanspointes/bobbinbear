import { Uuid } from "../../utils/uuid";
import {
  EmbBase,
  EmbHasFill,
  EmbHasInspecting,
  EmbHasLine,
} from "../shared";
import { VectorShape } from "../vec-seg";

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
    shape: VectorShape;
    close: boolean;
  };
