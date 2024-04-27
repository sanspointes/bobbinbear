import { createContext, useContext } from 'solid-js';
import { useBBDocument } from './document';
import { useBBViewport } from './viewport';

export type BobbinBearModel = ReturnType<typeof createBobbinBearContext>;
export const BobbinBearContext = createContext<BobbinBearModel | undefined>(
    undefined,
);

export function createBobbinBearContext() {
    const document = useBBDocument();
    const viewport = useBBViewport();

    return {
        document,
        viewport,
    };
}

export function useBobbinBear() {
    const ctx = useContext(BobbinBearContext);
    if (!ctx) throw new Error();
    return ctx;
}
