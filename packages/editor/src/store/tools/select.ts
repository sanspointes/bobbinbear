import { BaseStore, generateStore } from ".."
import { createStateMachine } from "../../primitives/createStateMachine";
import { ToolModel, ToolStoreMessage } from "../toolStore";
import { ToolInputMessage, ToolInputs, generateViewportStateMachine } from "./shared";

export type SelectToolMessage = {
  'activate': HTMLElement,
  'deactivate': HTMLElement,
  'input': ToolInputMessage,
}
type SelectToolModel = Record<string, unknown>

export type SelectToolStore = BaseStore<SelectToolModel, SelectToolMessage>;

export const createSelectToolStore = (toolStore: BaseStore<ToolModel, ToolStoreMessage>) => {

  const { events, states, transitions } = generateViewportStateMachine(toolStore.handle);

  const { can, dispatch } = createStateMachine(states.Blocked, transitions);

  return generateStore<SelectToolModel, SelectToolMessage>({}, {
    'input': (_1, _2, msg) => {
      msg.data
      if (msg.type === 'pointer1-down' && can(events.PointerDown)) {
        dispatch(events.PointerDown);
      } else if (msg.type === 'pointer1-up' && can(events.PointerUp)) {
        dispatch(events.PointerUp);
      } else if (msg.type === 'keydown' && (msg.data as ToolInputs['keydown']).key === ' ') {
        dispatch(events.SpaceDown);
      } else if (msg.type === 'keyup' && (msg.data as ToolInputs['keyup']).key === ' ') {
        dispatch(events.SpaceDown);
      }
    },
    'activate': (_1, _2) => {
      dispatch(events.Unblock);
    },
    'deactivate': (_1, _2) => {
      dispatch(events.Block);
    }
  })
}
