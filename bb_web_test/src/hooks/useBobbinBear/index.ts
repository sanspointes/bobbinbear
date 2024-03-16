import { createContext, useContext } from 'solid-js';
import { useBBDocument } from './document';

export type BobbinBearModel = ReturnType<typeof createBobbinBearContext>;
export const BobbinBearContext = createContext<BobbinBearModel | undefined>(
    undefined,
);

export function createBobbinBearContext() {
    const document = useBBDocument();

    return {
        document,
    };
}

export function useBobbinBear() {
    const ctx = useContext(BobbinBearContext);
    if (!ctx) throw new Error();
    return ctx;
}
