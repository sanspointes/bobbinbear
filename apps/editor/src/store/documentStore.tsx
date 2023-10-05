import { AllMessages, BaseStore, GeneralHandler, generateStore } from '.';
import { SceneModel } from './sceneStore';

export type EmbDocumentPreset = {
    id: number;
    brand: string;
    name: string;
    width: number;
    height: number;
};

export type EmbDocument = {
    name: string;
    slug: string;
    width: number;
    height: number;
};

export const EMB_DOC_PRESETS: EmbDocumentPreset[] = [
    {
        id: 0,
        brand: 'Janome',
        name: 'Memory Craft 500E',
        width: 280,
        height: 200,
    },
    {
        id: 1,
        brand: 'Janome',
        name: 'Continental M17',
        width: 460,
        height: 280,
    },
    {
        id: 2,
        brand: 'Janome',
        name: 'Skyline S9',
        width: 170,
        height: 200,
    },
];

export type DocumentMessage = {
    'document:new': EmbDocument;
    'document:load': string;
};
export type DocumentModel = {
    documents: Record<string, EmbDocument>;
    activeDocumentSlug?: string;
    activeDocument?: EmbDocument;
};

export function createDocumentStore(
    dispatch: GeneralHandler<AllMessages>,
    sceneStore: SceneModel,
) {
    const model: DocumentModel = {
        documents: {},
        get activeDocument() {
            if (!this.activeDocumentSlug) return undefined;
            const v = this.documents[this.activeDocumentSlug];
            if (v) return v;
            throw new Error(
                `DocumentStore: activeDocumentSlug is "${this.activeDocumentSlug}" but does not map to a document.`,
            );
        },
    };

    const result = generateStore<DocumentModel, DocumentMessage>(model, {
        'document:new': (store, set, data) => {
            if (store.documents[data.slug]) {
                console.warn('DocumentStore: Overwriting existing document');
            }
            set('documents', data.slug, data);
        },
        'document:load': (store, set, data) => {
            dispatch('scene:reset');
        },
    });

    return result;
}
