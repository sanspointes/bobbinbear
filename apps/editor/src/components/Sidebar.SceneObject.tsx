import { useContext } from 'solid-js';
import { AccordionItem } from './generics/Accordian';
import { AppContext } from '../store';
import { SetSceneObjectFieldCommand } from '../store/commands';
import { Checkbox } from './generics/Checkbox';
import { EmbObject } from '@/emb-objects';

type SidebarSceneObjectProps = {
    object: EmbObject;
};
export function SidebarSceneObject(props: SidebarSceneObjectProps) {
    const { dispatch } = useContext(AppContext);

    const setVisible = (visible: boolean) => {
        const obj = props.object;
        if (!obj) return;
        const cmd = new SetSceneObjectFieldCommand(obj.id, 'visible', visible);
        dispatch('scene:do-command', cmd);
    };

    return (
        <AccordionItem
            value="scene-object"
            header={props.object.name}
            innerClass="grid grid-cols-2 gap-4"
        >
            <Checkbox
                class="col-span-2"
                label="Visible"
                checked={props.object.visible}
                onChange={setVisible}
            />
        </AccordionItem>
    );
}
