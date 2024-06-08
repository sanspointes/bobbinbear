import { BobbinCursor, BobbinTool, ToolApi } from "bb_core";
import { createSignal } from "solid-js";
import { EffectEmitter, useEffectEmitter } from "~/utils/effect-emitter";

export function useBBTools(effectEmitter: EffectEmitter) {
    const toolsApi = new ToolApi();

    const [currentTool, setCurrentTool] = createSignal<BobbinTool>('Select');
    const [cursor, setCursor] = createSignal<BobbinCursor>('Default');

    useEffectEmitter(effectEmitter, 'ToolChanged', (tool) => {
        setCurrentTool(tool);
    })
    useEffectEmitter(effectEmitter, 'CursorChanged', (cursor) => {
        setCursor(cursor);
    });

    const switchTool = (tool: BobbinTool) => {
        toolsApi.set_base_tool(tool);
    }

    return {
        cursor,
        switchTool,
        currentTool,
    }
}
