import { Resizable } from './generics/Resizable';
import { SceneTree } from './SidebarLeft.SceneTree';
import { SidebarLeftSwitcher } from './SidebarLeft.Switcher';

export function SidebarLeft() {
    return (
        <Resizable
            handlePosition="right"
            defaultWidth={200}
            class="bg-orange-50"
            minWidth={200}
            maxWidth={400}
        >
            <SidebarLeftSwitcher />
            <SceneTree />
        </Resizable>
    );
}
