import { BobbinTool, ToolApi } from "bb_core";
import { createSignal } from "solid-js";
import { EffectEmitter, useEffectEmitter } from "~/utils/effect-emitter";

export function useBBTools(effectEmitter: EffectEmitter) {
    const toolsApi = new ToolApi();

    const [currentTool, setCurrentTool] = createSignal<BobbinTool>('Select');

    useEffectEmitter(effectEmitter, 'ToolChanged', (tool) => {
        setCurrentTool(tool);
    })

    const switchTool = (tool: BobbinTool) => {
        toolsApi.set_base_tool(tool);
    }

    return {
        switchTool,
        currentTool,
    }
}
