import { createMemo, Show, useContext } from 'solid-js';
import { AppContext } from '../store';
import { arrayFirst } from '../utils/array';
import { SidebarTransform } from './Sidebar.Transform';
import { Accordion } from './generics/Accordian';
import { SidebarDebug } from './Sidebar.Debug';
import { SidebarSceneObject } from './Sidebar.SceneObject';
import { SidebarStyle } from './Sidebar.Style';
import { Resizable } from './generics/Resizable';

export function Sidebar() {
    const { sceneStore } = useContext(AppContext);
    const firstObject = createMemo(() =>
        arrayFirst(sceneStore.selectedObjects),
    );

    return (
        <Resizable
            class="p-4 bg-orange-500 border-l border-orange-700 border-solid w-[400px] box-border"
            handlePosition="left"
            defaultWidth={200}
            minWidth={200}
            maxWidth={400}
        >
            <div>
                <div class="grid grid-cols-2 gap-4">
                    <Accordion
                        class="col-span-2"
                        multiple
                        collapsible
                        defaultValue={['scene-object', 'transform', 'style']}
                    >
                        <Show when={firstObject()}>
                            {(obj) => (
                                <>
                                    <SidebarSceneObject object={obj()} />
                                    <SidebarTransform object={obj()} />
                                    <SidebarStyle object={obj()} />
                                </>
                            )}
                        </Show>
                        <SidebarDebug />
                    </Accordion>
                </div>
            </div>
        </Resizable>
    );
}
