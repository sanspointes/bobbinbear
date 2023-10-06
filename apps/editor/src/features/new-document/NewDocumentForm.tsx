import { Show, createMemo, createSignal } from 'solid-js';
import {
    EMB_DOC_PRESETS,
    EmbDocument,
    EmbDocumentPreset,
} from '@/store/documentStore';
import {
    UnitNumberInputInput,
    UnitNumberInputLabel,
    UnitNumberInputRoot,
} from '@/components/generics/UnitNumberInput';
import { Button } from '@/components/generics/Button';
import {
    SelectList,
    SelectRoot,
    SelectLabel,
    SelectTrigger,
} from '@/components/generics/NewSelect';
import {
    TextInputInput,
    TextInputLabel,
    TextInputRoot,
} from '@/components/generics/TextInput';
import { StringUtils } from '@/utils/string';

type NewDocumentFormProps = {
    onCancel(): void;
    onCreate(document: EmbDocument): void;
};
export function NewDocumentForm(props: NewDocumentFormProps) {
    const [name, setName] = createSignal('My Design');
    const nameSlug = createMemo(() => StringUtils.slugify(name()));
    const [width, setWidth] = createSignal<number>(EMB_DOC_PRESETS[0]!.width);
    const [height, setHeight] = createSignal<number>(
        EMB_DOC_PRESETS[0]!.height,
    );

    const selectedPreset = createMemo<EmbDocumentPreset>(() => {
        const preset = EMB_DOC_PRESETS.find(
            (p) => p.width === width() && p.height === height(),
        );

        return (
            preset ??
            ({
                id: -1,
                width: width(),
                height: height(),
                name: 'Custom',
                brand: 'Custom',
            } as EmbDocumentPreset)
        );
    });
    return (
        <>
            <h3 class="mt-6 mb-4 w-full text-lg text-orange-950">
                What would you like to call your document?
            </h3>
            <div class="flex justify-center items-center">
                <TextInputRoot
                    class="w-96 max-w-[90%]"
                    value={name()}
                    onChange={setName}
                >
                    <TextInputLabel class="hidden">Name</TextInputLabel>
                    <TextInputInput />
                </TextInputRoot>
            </div>

            <div class="my-8 border-b border-orange-300 border-solid" />

            <h3 class="mt-6 mb-4 w-full text-lg text-orange-950">
                Select a size for your embroidery design
            </h3>
            <div class="flex gap-4 justify-between">
                <div class="text-orange-900 sm:w-[300px]">
                    <SelectRoot
                        class="flex gap-4 items-center w-full"
                        value={selectedPreset()}
                        options={EMB_DOC_PRESETS}
                        onChange={(value) => {
                            setWidth(value.width);
                            setHeight(value.height);
                        }}
                        itemRenderer={(option) => (
                            <Show when={option} fallback="Custom">
                                {(option) => (
                                    <span>
                                        <p class="text-sm">
                                            <span class="mr-4">
                                                {option().width}x
                                                {option().height}
                                            </span>
                                            <span>{option().brand}</span>
                                        </p>{' '}
                                        <p class="w-full font-medium">
                                            {option().name}
                                        </p>
                                    </span>
                                )}
                            </Show>
                        )}
                        optionValue={(v) => v.id}
                        multiple={false}
                    >
                        <SelectLabel class="w-24">Preset</SelectLabel>
                        <SelectTrigger class="flex-grow min-h-[60px]" />
                        <SelectList usePortal={false} class="z-50" />
                    </SelectRoot>

                    <div class="my-4 border-b border-orange-300 border-solid" />

                    <UnitNumberInputRoot
                        class="mb-4 w-full"
                        value={width()}
                        onChange={setWidth}
                    >
                        <UnitNumberInputLabel class="w-24">
                            Width
                        </UnitNumberInputLabel>
                        <UnitNumberInputInput unit="mm" class="w-32" />
                    </UnitNumberInputRoot>

                    <UnitNumberInputRoot
                        class="w-full"
                        value={height()}
                        onChange={setHeight}
                    >
                        <UnitNumberInputLabel class="w-24">
                            Height
                        </UnitNumberInputLabel>
                        <UnitNumberInputInput unit="mm" class="w-32" />
                    </UnitNumberInputRoot>
                </div>

                <div class="hidden justify-center items-center sm:flex w-[200px] box-border">
                    <div class="relative w-40 h-40">
                        <div
                            class="absolute top-1/2 left-1/2 bg-white shadow-xl -translate-x-1/2 -translate-y-1/2 shadow-orange-600/30"
                            classList={{
                                'w-full': width() >= height(),
                                'h-full': height() > width(),
                            }}
                            style={{
                                'aspect-ratio': `${width()} / ${height()}`,
                            }}
                        >
                            <span class="absolute left-1/2 bottom-full -translate-x-1/2">
                                {width()}mm
                            </span>
                            <span class="absolute top-1/2 right-[calc(100%-5px)] -rotate-90">
                                {height()}mm
                            </span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex gap-4 justify-end items-center mt-8">
                <Button onClick={() => props.onCancel()}>Cancel</Button>
                <Button
                    inverted
                    onClick={() =>
                        props.onCreate({
                            name: name(),
                            slug: nameSlug(),
                            width: width(),
                            height: height(),
                        })
                    }
                >
                    Create new document
                </Button>
            </div>
        </>
    );
}
