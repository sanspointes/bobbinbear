import { Container } from "@pixi/display";
import { EmbBase } from "../types/scene";
import { createEffect } from "solid-js";
import { OutlineFilter } from "@pixi/filter-outline";
import { arrayRemoveEl } from "../utils/array";

const HOVER_OUTLINE = new OutlineFilter(1, 0x41A3E9, 1.0);
const SELECT_OUTLINE = new OutlineFilter(2, 0x0A8CE9, 1.0);

enum State {
  None,
  Hover,
  Select,
}

export const useHoverSelectOutline = (
  ref: Container,
  props: EmbBase,
) => {
  let state: State;
  if (!ref.filters) ref.filters = [];
  createEffect(() => {
    if (props.selected && state !== State.Select) {
      if (state === State.Hover) {
        arrayRemoveEl(ref.filters!, HOVER_OUTLINE);
      }
      ref.filters!.push(SELECT_OUTLINE);
      state = State.Select;
    } else if (props.hovered && state !== State.Hover) {
      if (state == State.Select) {
        arrayRemoveEl(ref.filters!, SELECT_OUTLINE);
      }
      ref.filters!.push(HOVER_OUTLINE);
      state = State.Hover;
    } else {
      if (ref.filters?.includes(SELECT_OUTLINE)) {
        arrayRemoveEl(ref.filters!, SELECT_OUTLINE);
      } else if (ref.filters?.includes(HOVER_OUTLINE)) {
        arrayRemoveEl(ref.filters!, HOVER_OUTLINE);
      }
      state = State.None;
    }
  });
};
