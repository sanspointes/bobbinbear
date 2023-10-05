import { Button } from '@/components/generics/Button';
import { Select } from '@/components/generics/Select';
import { AppContext } from '@/store';
import { EmbDocument } from '@/store/documentStore';
import { FaSolidPlus } from 'solid-icons/fa';
import { Show, createMemo, createSignal, useContext } from 'solid-js';
import { NewDocumentModal } from './Modals.NewDocument';

export function SidebarLeftSwitcher() {
    const { dispatch, documentStore } = useContext(AppContext);
    const documents = createMemo(() => Object.values(documentStore.documents));

    const setCurrentDocument = (v: EmbDocument) => {
        dispatch('document:load', v);
    };

    const [showNew, setShowNew] = createSignal(false);

    return (
        <div class="flex justify-between items-center border-b border-solid border-b-orange-300">
            <Show when={documentStore.activeDocument}>
                {(activeDocument) => (
                    <Select
                        value={activeDocument()}
                        options={documents()}
                        onChange={(v) => setCurrentDocument(v)}
                        multiple={false}
                    >
                        {(option) => <span>{option.name}</span>}
                    </Select>
                )}
            </Show>
            <Button onClick={() => setShowNew(true)}>
                <FaSolidPlus />
            </Button>
            <Show when={showNew()}>
                <NewDocumentModal onClose={() => setShowNew(false)} />
            </Show>
        </div>
    );
}
