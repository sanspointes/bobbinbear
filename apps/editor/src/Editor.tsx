import { ErrorBoundary, createSignal } from 'solid-js';

import { preventDefault } from '@solid-primitives/event-listener';
// import { DragDropProvider, DragDropSensors } from '@thisbeyond/solid-dnd';
// import { SidebarLeft } from './components/SidebarLeft';

// import { AppContext, createAppStore } from './stores';

import { ErrorView } from './components/ErrorView';
// import { Sidebar } from './components/Sidebar';
import { Toolbar } from './components/Toolbar';
// import {
//     NewDocumentLauncher,
//     requestNewDocument,
// } from './features/new-document';
import { Renderer } from './features/renderer';
import { AppContext, createAppStore } from './stores';

export const [appError, setAppError] = createSignal<Error>();

export const Editor = () => {
    let wrapperEl: HTMLDivElement | undefined;
    // onMount(() => {
    //     console.log(documentStore.activeDocumentSlug);
    //     if (documentStore.activeDocumentSlug === undefined) {
    //         requestNewDocument({ cancellable: false }).then((document) => {
    //             dispatch('document:new', document);
    //         });
    //     }
    // });

    const onWheel = preventDefault(() => {});

    const contextModel = createAppStore();

    // const [justTapped, setJustTapped] = createSignal(false);
    // const handlePointerDown = () => {
    //     setJustTapped(true);
    //     setTimeout(() => {
    //         setJustTapped(false);
    //     }, 80);
    // };
    return (
        <div
            ref={wrapperEl}
            tabindex={0}
            class="flex flex-col items-stretch w-full h-full text-orange-50 fill-orange-50 stroke-orange-50"
            onWheel={onWheel}
        >
            <ErrorBoundary fallback={(error) => <ErrorView error={error} />}>
                <AppContext.Provider value={contextModel}>
                    <Toolbar />
                    <div class="flex flex-grow">
                        {/*
                        <DragDropProvider>
                            <DragDropSensors>
                                <SidebarLeft />
                            </DragDropSensors>
                        </DragDropProvider>
                        */}
                        <Renderer />
                        {/* <Sidebar /> */}
                    </div>
                    {/*
                    <div>
                        <NewDocumentLauncher />
                    </div>
                    */}
                </AppContext.Provider>
            </ErrorBoundary>
        </div>
    );
};
