import { Accessor, createContext, useContext } from 'solid-js';
import { BBDocument } from '../../hooks/useBobbinBear/document';

type DocTreeContextModel = ReturnType<typeof createDocTreeContext>;
export const DocTreeContext = createContext<DocTreeContextModel>(null!);

export function createDocTreeContext(document: Accessor<BBDocument>) {
    return document;
}

export function useDocTreeContext() {
    const ctx = useContext(DocTreeContext);
    if (!ctx) throw new Error('Must be used within a DocTreeContext.Provider.');
    return ctx();
}
