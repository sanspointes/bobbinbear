import { Point } from "@pixi/core";
import { AllMessages, GeneralHandler, generateStore } from ".";
import { produce } from "solid-js/store";
import { createEffect } from "solid-js";
import { pointDistance } from "../utils/point";
import { ToolInputs } from "./tools/shared";
import { createEventListener } from "@solid-primitives/event-listener";
import { ViewportModel } from "./viewportStore";
import { SceneModel } from "./sceneStore";

export type InputMessages = {
  "input:set-source": {
    pointer?: HTMLElement;
    keys?: HTMLElement;
  };
  "input:pointerdown": {
    position: Point;
  };
  "input:pointermove": {
    position: Point;
    downPosition: Point;
  };
  "input:pointerup": {
    position: Point;
    downPosition: Point;
  };
  "input:keydown": {
    key: KeyboardEvent["key"];
  };
  "input:keypress": {
    key: KeyboardEvent["key"];
  };
  "input:keyup": {
    key: KeyboardEvent["key"];
  };
};

export type InputToolSettings = {
  dragThreshold: number;
};

export type InputModel = {
  settings: InputToolSettings;

  pointerSource: HTMLElement | undefined;
  keySource: HTMLElement | undefined;
  isDragging: boolean;
  keys: Set<string>;
  position: Point;
  downPosition?: Point;
};

const makeToolInputResponse = <
  K extends keyof ToolInputs,
  M extends ToolInputs[K],
>(type: K, data: M): AllMessages["tool:input"] => {
  return {
    type,
    data,
  };
};

export const createInputStore = (
  dispatch: GeneralHandler<AllMessages>,
) => {
  const store = generateStore<InputModel, InputMessages>({
    settings: {
      dragThreshold: 2,
    },
    pointerSource: undefined,
    keySource: undefined,
    isDragging: false,
    keys: new Set(),
    position: new Point(),
    downPosition: undefined,
  }, {
    "input:set-source": (_, set, message) => {
      set(produce((store) => {
        if (message.pointer) {
          store.pointerSource = message.pointer;
        }
        if (message.keys) {
          store.keySource = message.keys;
        }
      }));
    },

    "input:pointerdown": (_, set, message, respond) => {
      set(produce((store) => store.downPosition = message.position));
      respond!(
        "tool:input",
        makeToolInputResponse("pointer1-down", {
          position: message.position,
        }),
      );
    },

    "input:pointermove": (store, set, message, respond) => {
      set(produce((store) => store.position = message.position));

      const dragDistance = store.downPosition &&
        pointDistance(store.downPosition, message.position);
      if (store.isDragging) {
        respond!(
          "tool:input",
          makeToolInputResponse("pointer1-dragmove", {
            position: message.position,
            downPosition: store.downPosition as Point,
          }),
        );
      } else if (dragDistance && dragDistance > store.settings.dragThreshold) {
        set('isDragging', true);
        respond!(
          "tool:input",
          makeToolInputResponse("pointer1-dragstart", {
            position: message.position,
            downPosition: store.downPosition as Point,
          }),
        );
      } else {
        respond!(
          "tool:input",
          makeToolInputResponse("pointer1-move", {
            position: message.position,
            downPosition: store.downPosition as Point,
          }),
        );
      }
    },

    "input:pointerup": (store, set, message, respond) => {
      if (store.isDragging) {
        respond!(
          "tool:input",
          makeToolInputResponse("pointer1-dragend", {
            position: message.position,
            downPosition: store.downPosition as Point,
          }),
        );
        set('isDragging', false);
      } else {
        respond!(
          "tool:input",
          makeToolInputResponse("pointer1-click", {
            position: message.position,
          }),
        );
      }
      respond!(
        "tool:input",
        makeToolInputResponse("pointer1-up", {
          position: message.position,
          downPosition: store.downPosition as Point,
        }),
      );
      set(produce((store) => store.downPosition = undefined));
    },

    "input:keypress": (_1, _2, message, respond) => {
      respond!(
        "tool:input",
        makeToolInputResponse("keypress", {
          key: message.key,
        }),
      );
    },

    "input:keydown": (store, set, message, respond) => {
      if (!store.keys.has(message.key)) {
        set(produce((store) => store.keys.add(message.key)));
        respond!(
          "tool:input",
          makeToolInputResponse("keydown", {
            key: message.key,
            keys: store.keys,
          }),
        );
      }
    },

    "input:keyup": (store, set, message, respond) => {
      set(produce((store) => store.keys.delete(message.key)));
      respond!(
        "tool:input",
        makeToolInputResponse("keyup", {
          key: message.key,
          keys: store.keys,
        }),
      );
    },
  });

  // Pointer events are handled by the Viewport.tsx class.

  // Bind key events on the key source
  createEffect(() => {
    if (store.store.keySource) {
      createEventListener(store.store.keySource, "keydown", (e) => {
        store.handle("input:keydown", {
          key: e.key,
        }, dispatch);
      });
      createEventListener(
        store.store.keySource,
        "keyup",
        (e) =>
          store.handle("input:keyup", {
            key: e.key,
          }, dispatch),
      );
      createEventListener(
        store.store.keySource,
        "keypress",
        (e) =>
          store.handle("input:keypress", {
            key: e.key,
          }, dispatch),
      );
    }
  });

  return store;
};
