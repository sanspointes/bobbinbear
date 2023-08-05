import { Point } from '@pixi/core';
import { AllMessages, GeneralHandler, generateStore } from '.';
import { produce } from 'solid-js/store';
import { createEffect } from 'solid-js';
import { pointDistance } from '../utils/point';
import { ToolInputs } from './tools/shared';
import { createEventListener } from '@solid-primitives/event-listener';

export type InputMessages = {
  'input:set-source': {
    element: HTMLElement,
  },
  'input:pointerdown': {
    position: Point,
  },
  'input:pointermove': {
    position: Point,
    downPosition: Point,
  }
  'input:pointerup': {
    position: Point,
    downPosition: Point,
  },
  'input:keydown': {
    key: KeyboardEvent['key']
  },
  'input:keypress': {
    key: KeyboardEvent['key']
  }
  'input:keyup': {
    key: KeyboardEvent['key']
  }
}

export type InputToolSettings = {
  dragThreshold: number;
}

export type InputModel = {
  settings: InputToolSettings,

  source: HTMLElement|undefined,
  isDragging: boolean,
  keys: Set<string>,
  downPosition?: Point,
}

const makeToolInputResponse = <K extends keyof ToolInputs, M extends ToolInputs[K]>(type: K, data: M): AllMessages['tool:input'] => {
  return {
    type,
    data,
  }
}

export const createInputStore = (dispatch: GeneralHandler<AllMessages>) => {
  const store = generateStore<InputModel, InputMessages>({
    settings: {
      dragThreshold: 2,
    },
    source: undefined,
    isDragging: false,
    keys: new Set(),
    downPosition: undefined,
  }, {
    'input:set-source': (_, set, message) => {
        set(produce(store => store.source = message.element));
    },
    'input:pointerdown': (_, set, message, respond) => {
      set(produce(store => store.downPosition = message.position));
      respond!('tool:input', makeToolInputResponse('pointer1-down', {
        position: message.position,
      }))
    },
    'input:pointermove': (store, _, message, respond) => {
      const dragDistance = store.downPosition && pointDistance(store.downPosition, message.position)
      if (store.isDragging) {
        respond!('tool:input', makeToolInputResponse('pointer1-dragmove', {
          position: message.position,
          downPosition: store.downPosition as Point,
        }))
      } else if (dragDistance && dragDistance > store.settings.dragThreshold) {
        respond!('tool:input', makeToolInputResponse('pointer1-dragstart', {
          position: message.position,
          downPosition: store.downPosition as Point,
        }))
      } else {
        respond!('tool:input', makeToolInputResponse('pointer1-move', {
          position: message.position,
          downPosition: store.downPosition as Point,
        }))
      }
    },
    'input:pointerup': (store, set, message, respond) => {
      if (store.isDragging) {
        respond!('tool:input', makeToolInputResponse('pointer1-dragend', {
          position: message.position,
          downPosition: store.downPosition as Point,
        }))
      } else {
        respond!('tool:input', makeToolInputResponse('pointer1-click', {
          position: message.position,
        }))
      }
      respond!('tool:input', makeToolInputResponse('pointer1-up', {
        position: message.position,
        downPosition: store.downPosition as Point,
      }))
      set(produce(store => store.downPosition = undefined));
    },
    'input:keypress': (_1, _2, message, respond) => {
      respond!('tool:input', makeToolInputResponse('keypress', {
        key: message.key,
      }))
    },
    'input:keydown': (store, set, message, respond) => {
      set(produce(store => store.keys.add(message.key)));
      respond!('tool:input', makeToolInputResponse('keydown', {
        key: message.key,
        keys: store.keys,
      }))
    },
    'input:keyup': (store, set, message, respond) => {
      set(produce(store => store.keys.delete(message.key)));
      respond!('tool:input', makeToolInputResponse('keyup', {
        key: message.key,
        keys: store.keys,
      }))
    }
  })

  // Bind all the event listeners when source is provided.
  createEffect(() => {
    if (store.store.source) {
      createEventListener(store.store.source, 'pointerdown', (e) => store.handle('input:pointerdown', {
        position: new Point(e.clientX, e.clientY),
      }, dispatch));
      createEventListener(store.store.source, 'pointermove', (e) => store.handle('input:pointermove', {
        position: new Point(e.clientX, e.clientY),
      }, dispatch));
      createEventListener(store.store.source, 'pointerup', (e) => store.handle('input:pointerup', {
        position: new Point(e.clientX, e.clientY),
      }, dispatch));
      createEventListener(store.store.source, 'keydown', (e) => store.handle('input:keydown', {
        key: e.key,
      }, dispatch));
      createEventListener(store.store.source, 'keyup', (e) => store.handle('input:keyup', {
        key: e.key,
      }, dispatch));
      createEventListener(store.store.source, 'keypress', (e) => store.handle('input:keypress', {
        key: e.key,
      }, dispatch));
    }
  })

  return store;
}
