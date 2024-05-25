import { batch, createContext, useContext } from 'solid-js';
import { useBBDocument } from './document';
import { useBBViewport } from './viewport';
import { useBBTools } from './tools';
import { Effect } from 'bb_core';
import effectEmitter from '~/utils/effect-emitter';

export type BobbinBearModel = ReturnType<typeof createBobbinBearContext>;
export const BobbinBearContext = createContext<BobbinBearModel | undefined>(
    undefined,
);


export function createBobbinBearContext() {
    const emitter = effectEmitter();
    // @ts-expect-error: untyped...
    window.receiveRustEvents = (effects: Effect[]) => {
        batch(() => {
            console.debug(`Received ${effects.length} effects to handle.`);
            for (const eff of effects) {
                emitter.emit(eff.tag, eff.value);
            }
        });
    };


    const document = useBBDocument(emitter);
    const viewport = useBBViewport(emitter);
    const tools = useBBTools(emitter);

    return {
        document,
        viewport,
        tools,
    };
}

export function useBobbinBear() {
    const ctx = useContext(BobbinBearContext);
    if (!ctx) throw new Error();
    return ctx;
}
