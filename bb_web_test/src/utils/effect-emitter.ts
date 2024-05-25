/* eslint-disable @typescript-eslint/no-explicit-any */
import { Effect } from 'bb_core';
import { onCleanup, onMount } from 'solid-js';
export type Handler<T = unknown> = (event: T) => void;

// Maps from a union with `tag` and `value` to a Record<tag, value>
type EffectMap = { [K in Effect['tag']]: Extract<Effect, { tag: K }>['value'] };

export interface EffectEmitter {
    on<Key extends keyof EffectMap>(
        type: Key,
        handler: Handler<EffectMap[Key]>,
    ): void;

    off<Key extends keyof EffectMap>(
        type: Key,
        handler: Handler<EffectMap[Key]>,
    ): void;

    emit<Key extends keyof EffectMap>(type: Key, event: EffectMap[Key]): void;
}

export default function effectEmitter(): EffectEmitter {
    const all = new Map();
    return {
        on<Key extends keyof EffectMap>(
            key: Key,
            handler: Handler<EffectMap[Key]>,
        ) {
            const handlers: Array<Handler<EffectMap[Key]>> | undefined =
                all!.get(key);
            if (handlers) {
                handlers.push(handler);
            } else {
                all!.set(key, [handler]);
            }
        },
        off<Key extends keyof EffectMap>(
            key: Key,
            handler: Handler<EffectMap[Key]>,
        ) {
            const handlers: Array<Handler<EffectMap[Key]>> | undefined =
                all!.get(key);
            if (handlers) {
                handlers.splice(handlers.indexOf(handler) >>> 0, 1);
            }
        },
        emit<Key extends keyof EffectMap>(key: Key, evt: EffectMap[Key]) {
            const handlers = all!.get(key);
            if (handlers) {
                (handlers as Array<Handler<EffectMap[keyof EffectMap]>>)
                    .slice()
                    .map((handler) => {
                        handler(evt!);
                    });
            }
        },
    };
}

export function useEffectEmitter<Key extends keyof EffectMap>(
    effectEmitter: EffectEmitter,
    key: Key,
    handler: Handler<EffectMap[Key]>,
) {
    onMount(() => {
        effectEmitter.on(key, handler)

        onCleanup(() => {
            effectEmitter.off(key, handler)
        })
    })

}
