import { createMemo, useContext, For, Show } from 'solid-js';
import { EmbObject } from '../emb-objects/shared';
import { AccordionItem } from './generics/Accordian';
import { AppContext } from '../store';
import { Cursor } from '../store/toolStore';
import { arrayFirst } from '../utils/array';

export function SidebarDebug() {
    const { toolStore, sceneStore } = useContext(AppContext);

    const { boxTool, selectTool } = toolStore;

    const first = createMemo(() => arrayFirst(sceneStore.selectedObjects));

    return (
        <AccordionItem value="debug" header="Debug">
            <p>tool: {toolStore.tool.toString()}</p>
            <p>
                cursor:{' '}
                <For each={toolStore.cursorStack}>
                    {(c) => <span>{Cursor[c]}</span>}
                </For>
            </p>
            <h3>SelectTool: {selectTool.state().toString()}</h3>
            <h3>BoxTool: {boxTool.state().toString()}</h3>
            <div class="border-b border-orange-800 border-solid" />
            <h3>Scene</h3>
            <p>Inspecting: {sceneStore.inspecting}</p>
            <p>Selected IDs: {sceneStore.selectedIds.join(',')}</p>
            <Show when={first()}>
                {(first) => (
                    <p>
                        Selected: ({first().position.x},{first().position.y}{' '}
                    </p>
                )}
            </Show>
        </AccordionItem>
    );
}
