import { Canvas, SolixiState, useSolixi } from '@bearbroidery/solixi';
import {
    ErrorBoundary,
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

import { SceneObjectChildren } from './emb-objects';
import { CursorTest } from './sxi-components/CursorTest';
import { SelectBox } from './sxi-components/SelectBox';
import { Viewport } from './sxi-components/Viewport';

import { uuid } from './utils/uuid';

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

    const rootObject = sceneStore.objects.get(uuid('root'));

    return (
        <>
            <CursorTest />
            <SelectBox />
            <Viewport>
                <SceneObjectChildren children={rootObject!.children} />
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
    const { toolStore } = contextModel;
    let wrapperEl: HTMLDivElement | undefined;
    onMount(() => {
        contextModel.dispatch('input:set-source', {
            keys: wrapperEl,
        });
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
                        <Canvas
                            onPointerDown={handlePointerDown}
                            classList={{
                                'b-cursor-default':
                                    toolStore.currentCursor === Cursor.Default,
                                'b-cursor-default-tap':
                                    toolStore.currentCursor ===
                                        Cursor.Default && justTapped(),
                                'b-cursor-grab':
                                    toolStore.currentCursor === Cursor.Grab,
                                'b-cursor-grabbing':
                                    toolStore.currentCursor === Cursor.Grabbing,
                                'b-cursor-pointer':
                                    toolStore.currentCursor === Cursor.Point,
                                'b-cursor-pointer-tap':
                                    toolStore.currentCursor === Cursor.Point &&
                                    justTapped(),
                                'b-cursor-box':
                                    toolStore.currentCursor === Cursor.Box,
                                'b-cursor-cross':
                                    toolStore.currentCursor === Cursor.Cross,
                                'b-cursor-pen':
                                    toolStore.currentCursor === Cursor.Pen,
                                'b-cursor-moving':
                                    toolStore.currentCursor === Cursor.Moving,
                            }}
                            devtools={true}
                            onCreated={onCreated}
                            app={{
                                backgroundColor: 0xe1e1e1,
                                resolution: window.devicePixelRatio,
                            }}
                        >
                            <EditorView />
                        </Canvas>
                        <Sidebar />
                    </div>
                </AppContext.Provider>
            </ErrorBoundary>
        </div>
    );
};
