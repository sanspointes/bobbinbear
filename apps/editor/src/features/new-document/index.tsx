import { EmbDocument } from '@/store/documentStore';
import { Show, createSignal } from 'solid-js';
import { NewDocumentModal } from './NewDocumentModal';
import { ParkedPromise, PromiseUtils } from '@/utils/promise';

export * from './NewDocumentForm';
export * from './NewDocumentModal';

const [promiseHandles, setPromiseHandles] = createSignal<
    (RequestNewDocumentOptions & ParkedPromise<EmbDocument>) | undefined
>();
export function NewDocumentLauncher() {
    return (
        <Show when={promiseHandles()}>
            {(handle) => (
                <NewDocumentModal
                    isCancellable={handle().cancellable}
                    onCreate={(document) => {
                        const h = handle();
                        setPromiseHandles(undefined);
                        h.resolve(document);
                    }}
                    onClose={() => {
                        const h = handle();
                        setPromiseHandles(undefined);
                        h.reject(new Error('Cancelled.'));
                    }}
                />
            )}
        </Show>
    );
}

/**
 * Exposes a promise based api for creating new documents, returning the output here.
 */
type RequestNewDocumentOptions = {
    cancellable: boolean;
};
export function requestNewDocument(
    opts?: Partial<RequestNewDocumentOptions>,
): Promise<EmbDocument> {
    const handle = promiseHandles();
    if (handle) return handle.promise;

    const parkedPromise = PromiseUtils.createParkable<EmbDocument>();
    setPromiseHandles({
        ...parkedPromise,
        cancellable: opts?.cancellable ?? true,
    });

    return parkedPromise.promise;
}
