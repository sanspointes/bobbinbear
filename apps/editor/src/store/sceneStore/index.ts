/* eslint-disable solid/reactivity */
import { createStore, produce, SetStoreFunction } from 'solid-js/store';
import { batch } from 'solid-js';
import { Point } from '@pixi/core';
import { ReactiveMap } from '@solid-primitives/map';

import { Uuid, uuid } from '../../utils/uuid';
import { Command } from '../commands';
import { generateStore } from '..';
import { arrayLast } from '../../utils/array';
import {
    EMB_STATE_DEFAULTS,
    EmbBase,
    EmbHasInspecting,
    EmbState,
} from '../../emb-objects/shared';
import { EmbCanvas, EmbObject } from '../../emb-objects';
import { SceneStoreSerialisable } from './utils';
import { EmbDocument } from '../documentStore';
import { hslFromRgb } from '@/utils/color';

export const getObject = <T extends EmbObject & EmbState>(
    store: SceneModel,
    uuid: Uuid<T> | undefined,
): T | undefined => {
    if (uuid === undefined) return undefined;
    return store.objects.get(uuid) as T | undefined;
};
export const getObjectSetter = <T extends EmbObject & EmbState>(
    store: SceneModel,
    uuid: Uuid<T> | undefined,
): SetStoreFunction<T> | undefined => {
    if (uuid === undefined) return undefined;
    const setter = store.objectSetters.get(uuid);
    if (!setter) return undefined;
    return setter as SetStoreFunction<T>;
};

export const isInspectable = <T extends EmbBase>(
    obj: T,
): obj is T & EmbHasInspecting => {
    const o = obj as unknown as T & EmbHasInspecting;
    if (o.inspecting !== undefined) return true;
    return false;
};
/**
 * Keep a flat reference to every object on the scene and its setter function
 */
export type ObjectMapData<T extends EmbObject = EmbObject> = {
    object: T;
    set: SetStoreFunction<T>;
};

export type SceneStoreMessages = {
    'scene:hover': Uuid<EmbObject & EmbState>;
    'scene:unhover': Uuid<EmbObject & EmbState>;
    'scene:do-command': Command<EmbObject & EmbState>;
    'scene:undo': void;
    'scene:redo': void;
    'scene:reset': EmbDocument;
    'scene:load': SceneStoreSerialisable;
};

export type SceneModel = {
    /** UUID of object that we're currently inspecting */
    inspecting: Uuid<EmbBase> | undefined;
    /** UUID of inspect root object, used for storing temporary parts of the document i.e. nodes */
    inspectRoot: Uuid<EmbBase> | undefined;
    /* List of selected ids */
    selectedIds: Uuid<EmbObject & EmbState>[];
    selectedObjects: (EmbObject & EmbState)[];
    undoStack: Command[];
    redoStack: Command[];
    objects: ReactiveMap<Uuid<EmbObject>, EmbObject & EmbState>;
    objectSetters: Map<Uuid<EmbObject>, SetStoreFunction<EmbObject & EmbState>>;
    root: EmbBase;
};

export const createSceneStore = () => {
    // Set the root object, this can't be edited

    const generateDefaultModel = (document?: EmbDocument): SceneModel => {
        const doc: EmbDocument = document ?? {
            name: 'Default',
            slug: 'default',
            width: 0,
            height: 0,
        };
        const [object, set] = createStore<EmbCanvas & EmbState>({
            ...EMB_STATE_DEFAULTS,
            type: 'canvas',
            id: uuid('root'),
            name: doc.name,
            parent: undefined as unknown as Uuid<EmbObject>,
            children: [],
            size: new Point(doc.width, doc.height),
            fill: {
                color: hslFromRgb({ r: 255, g: 255, b: 255 }),
                alpha: 1,
            },
            position: new Point(0, 0),
            shallowLocked: true,
        }) as [
            object: EmbObject & EmbState,
            set: SetStoreFunction<EmbObject & EmbState>,
        ];

        return {
            inspecting: undefined,
            inspectRoot: undefined,
            selectedIds: [],
            selectedObjects: [],
            undoStack: [],
            redoStack: [],
            objects: new ReactiveMap([[uuid('root'), object]]),
            objectSetters: new Map([[uuid('root'), set]]),
            root: object,
        } as SceneModel;
    };

    const result = generateStore<SceneModel, SceneStoreMessages>(
        generateDefaultModel(),
        {
            'scene:hover': (store, _2, uuid) => {
                const set = getObjectSetter(store, uuid);
                if (set) set('hovered', true);
            },
            'scene:unhover': (store, _2, uuid) => {
                const set = getObjectSetter(store, uuid);
                if (set) set('hovered', false);
            },
            'scene:do-command': (store, set, command) => {
                const lastCommand = arrayLast(store.undoStack);
                if (lastCommand) {
                    const sameType = lastCommand.type === command.type;
                    const needsPush = lastCommand.final;
                    const needsUpdate = !lastCommand.final && sameType;

                    // Error if not an update of previous or a new command entirely
                    if (!needsPush && !needsUpdate) {
                        throw new Error(
                            'perform-command: Invalid lastCommand/command.  Maybe you forgot to finalize the previous command?',
                        );
                    }

                    if (needsUpdate) {
                        if (!lastCommand.updateData) {
                            throw new Error(
                                `perform-command: Last Command marked as non final but no update method for ${lastCommand.type}:${lastCommand.name}`,
                            );
                        }

                        // @ts-expect-error; Type is asserted to be same by the `sameType` check above.
                        lastCommand.updateData(command);
                        lastCommand.final = command.final;
                    }

                    // Perform the command
                    const commandToPerform = needsUpdate
                        ? lastCommand
                        : command;
                    commandToPerform.perform(store, set);

                    // Push undo stack, clear redo stack
                    if (needsPush) {
                        set(
                            produce((store) => {
                                store.undoStack.push(command);
                                store.redoStack = [];
                            }),
                        );
                    }
                } else {
                    command.perform(store, set);

                    set(
                        produce((store) => {
                            store.undoStack.push(command);
                            store.redoStack = [];
                        }),
                    );
                }
            },
            'scene:undo': (store, set) => {
                batch(() => {
                    let command: Command | undefined;
                    set(
                        produce((store) => {
                            command = store.undoStack.pop();
                        }),
                    );
                    if (command) {
                        command.undo(store, set);

                        set(
                            produce((store) => {
                                store.redoStack.push(command!);
                            }),
                        );
                    }
                });
            },
            'scene:redo': (store, set) => {
                batch(() => {
                    let command: Command | undefined;
                    set(
                        produce((store) => {
                            command = store.redoStack.pop();
                        }),
                    );
                    if (command) {
                        command.perform(store, set);

                        set(
                            produce((store) => {
                                store.undoStack.push(command!);
                            }),
                        );
                    }
                });
            },
            'scene:reset': (store, set, document) => {
                batch(() => {
                    const defaults = generateDefaultModel(document);
                    for (const key in store) {
                        const k = key as keyof SceneModel;
                        const v = store[k];
                        // Clear and copy map values one by one
                        if (v instanceof Map) {
                            v.clear();
                            const defaultValue = defaults[k] as ReactiveMap<
                                Uuid<EmbObject>,
                                EmbObject & EmbState
                            >;
                            for (const [key, value] of defaultValue.entries()) {
                                v.set(key, value);
                            }
                        } else {
                            if (defaults[k] !== undefined) {
                                set(k, defaults[k]);
                            }
                        }
                    }
                });
            },
            'scene:load': (store, set, model) => {
                // TODO Reset tool stores etc.
                result.handle('scene:reset');
                batch(() => {
                    set('selectedIds', model.selectedIds);

                    for (const uuid in model.objects) {
                        const data = model.objects[uuid]!;

                        const [obj, setter] = createStore(data);
                        store.objects.set(uuid as Uuid<EmbBase>, obj);
                        store.objectSetters.set(uuid as Uuid<EmbBase>, setter);
                    }
                });
            },
        },
    );

    // @ts-expect-error Debug only inspection
    window.sceneStore = result.store;

    return result;
};
