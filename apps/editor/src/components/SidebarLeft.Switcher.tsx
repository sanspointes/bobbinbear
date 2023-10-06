import { Button } from '@/components/generics/Button';
import { Select } from '@/components/generics/Select';
import { AppContext } from '@/store';
import { EmbDocument } from '@/store/documentStore';
import { FaSolidPlus } from 'solid-icons/fa';
import { Show, createMemo, useContext } from 'solid-js';
import { requestNewDocument } from '@/features/new-document';

export function SidebarLeftSwitcher() {
    const { dispatch, documentStore } = useContext(AppContext);
    const documents = createMemo(() => Object.values(documentStore.documents));

    const setCurrentDocument = (v: EmbDocument) => {
        dispatch('document:load-by-slug', v.slug);
    };

    const handleNew = async () => {
        const document = await requestNewDocument();
        dispatch('document:new', document);
    };

    return (
        <div class="flex justify-between items-center border-b border-solid border-b-orange-300">
            <Show when={documentStore.activeDocument}>
                {(activeDocument) => (
                    <Select
                        class="flex-grow"
                        value={activeDocument()}
                        options={documents()}
                        optionValue="slug"
                        onChange={(v) => setCurrentDocument(v)}
                        multiple={false}
                    >
                        {(option) => <span>{option.name}</span>}
                    </Select>
                )}
            </Show>
            <Button onClick={handleNew}>
                <FaSolidPlus />
            </Button>
        </div>
    );
}
