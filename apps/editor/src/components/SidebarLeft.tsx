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
            <div class="flex justify-between items-center border-b border-solid border-b-orange-300">
                <SidebarLeftSwitcher />
            </div>
            <SceneTree />
        </Resizable>
    );
}
