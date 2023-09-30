/** @jsxImportSource solid-js */
import { QueryClient, QueryClientProvider } from '@tanstack/solid-query';
import { ErrorBoundary } from 'solid-js';
import { render } from 'solid-js/web';
import './styles.css';

import bobbinWasmModule from '@bearbroidery/bobbin-wasm-utils/bobbin-wasm-utils/pkg/bobbin_wasm_utils_bg.wasm?url';
import initWasm from '@bearbroidery/bobbin-wasm-utils';
initWasm(bobbinWasmModule);

import { Editor } from './Editor';
import { ErrorView } from './components/ErrorView';

import { attachDevtoolsOverlay } from '@solid-devtools/overlay';
attachDevtoolsOverlay();

const queryClient = new QueryClient();

const root = document.getElementById('root');

const App = () => {
    return (
        <ErrorBoundary fallback={(error) => <ErrorView error={error} />}>
            <QueryClientProvider client={queryClient}>
                <Editor />
            </QueryClientProvider>
        </ErrorBoundary>
    );
};
render(() => <App />, root!);
