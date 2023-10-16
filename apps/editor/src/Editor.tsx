import { Canvas, SolixiState, useSolixi } from '@bearbroidery/solixi';
import {
    ErrorBoundary,
    createMemo,
    createRenderEffect,
    createSignal,
    onMount,
    useContext,
} from 'solid-js';

import { preventDefault } from '@solid-primitives/event-listener';
import { DragDropProvider, DragDropSensors } from '@thisbeyond/solid-dnd';

import { AppContext, createAppStore } from './store';
import { Cursor } from './store/toolStore';

import { ErrorView } from './components/ErrorView';
import { SidebarLeft } from './components/SidebarLeft';
import { Sidebar } from './components/Sidebar';
import { Toolbar } from './components/Toolbar';

import { EmbCanvasView, EmbCanvas } from './emb-objects';
import { CursorTest } from './sxi-components/CursorTest';
import { SelectBox } from './sxi-components/SelectBox';
import { Viewport } from './sxi-components/Viewport';

import { uuid } from './utils/uuid';
import {
    NewDocumentLauncher,
    requestNewDocument,
} from './features/new-document';
import { Renderer } from './features/core';

export const [appError, setAppError] = createSignal<Error>();

const EditorView = () => {
    // Report errors
    createRenderEffect(() => {
        if (appError()) {
            throw new Error('An Async error occured: ', { cause: appError() });
        }
    });

    const { sceneStore, dispatch } = useContext(AppContext);
    const pixi = useSolixi();

    onMount(() => {
        dispatch('input:set-source', {
            pointer: pixi.app.view as unknown as HTMLCanvasElement,
        });
    });

    const rootObject = createMemo(() => {
        return sceneStore.objects.get(uuid('root')) as EmbCanvas;
    });

    return (
        <>
            <CursorTest />
            <SelectBox />
            <Viewport>
                <EmbCanvasView {...rootObject()} order={0} />
            </Viewport>
        </>
    );
};

export const Editor = () => {
    const [solixi, setSolixi] = createSignal<SolixiState>();
    const onCreated = (state: SolixiState) => {
        setSolixi(state);
    };

    const contextModel = createAppStore(solixi);
    const { dispatch, toolStore, documentStore } = contextModel;
    let wrapperEl: HTMLDivElement | undefined;
    onMount(() => {
        console.log(documentStore.activeDocumentSlug);
        if (documentStore.activeDocumentSlug === undefined) {
            // requestNewDocument({ cancellable: false }).then((document) => {
            //     dispatch('document:new', document);
            // });
        }
    });

    const onWheel = preventDefault(() => {});

    const [justTapped, setJustTapped] = createSignal(false);
    const handlePointerDown = () => {
        setJustTapped(true);
        setTimeout(() => {
            setJustTapped(false);
        }, 80);
    };
    return (
        <div
            ref={wrapperEl}
            tabindex={0}
            class="flex flex-col items-stretch w-full h-full text-orange-50 fill-orange-50 stroke-orange-50"
            onWheel={onWheel}
        >
            <ErrorBoundary
                fallback={(error) => (
                    <ErrorView
                        error={error}
                        stack={contextModel.sceneStore.undoStack}
                    />
                )}
            >
                <AppContext.Provider value={contextModel}>
                    <Toolbar />
                    <div class="flex flex-grow">
                        <DragDropProvider>
                            <DragDropSensors>
                                <SidebarLeft />
                            </DragDropSensors>
                        </DragDropProvider>
                        <Renderer />
                        <Sidebar />
                    </div>
                    <div>
                        <NewDocumentLauncher />
                    </div>
                </AppContext.Provider>
            </ErrorBoundary>
        </div>
    );
};
