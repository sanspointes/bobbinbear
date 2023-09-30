import { useContext } from 'solid-js';
import { AccordionItem } from './generics/Accordian';
import { AppContext } from '../store';
import { SetSceneObjectFieldCommand } from '../store/commands';
import { EmbText } from '@/emb-objects/text';
import { TextInput } from './generics/TextInput';
import { Select } from './generics/Select';

const GOOGLE_FONT_OPTIONS = [
    'Roboto',
    'Young Serif',
    'AR One Sans',
    'Inter',
    'Martian Mono',
    'Lora',
    'PT Serif',
];

type SidebarTransformProps = {
    object: EmbText;
};
export function SidebarText(props: SidebarTransformProps) {
    const { dispatch } = useContext(AppContext);

    const handleTextChange = (value: string) => {
        const cmd = new SetSceneObjectFieldCommand(
            props.object.id,
            'value',
            value,
        );
        dispatch('scene:do-command', cmd);
    };

    const handleFontFamilyChange = (fontFamily: string) => {
        const cmd = new SetSceneObjectFieldCommand(
            props.object.id,
            'fontFace',
            { ...props.object.fontFace, fontFamily },
        );
        dispatch('scene:do-command', cmd);
    };

    return (
        <AccordionItem
            value="transform"
            header="Text"
            innerClass="grid grid-cols-2 gap-4"
        >
            <TextInput
                class="col-span-2"
                label="Text"
                value={props.object.value}
                onChange={(e) => handleTextChange(e)}
            />
            <Select
                class="col-span-2"
                multiple={false}
                value={props.object.fontFace.fontFamily}
                options={GOOGLE_FONT_OPTIONS}
                onChange={(e) => handleFontFamilyChange(e)}
            >
                {(value) => <span>{value}</span>}
            </Select>
        </AccordionItem>
    );
}
