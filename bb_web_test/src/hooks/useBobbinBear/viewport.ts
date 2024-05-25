import { ViewportApi } from "bb_core";
import { EffectEmitter } from "~/utils/effect-emitter";

export function useBBViewport(_effectEmitter: EffectEmitter) {
    const viewportApi = new ViewportApi();

    const setResolution = (width: number, height: number) => {
        return viewportApi.set_resolution(width, height);
    }
    
    return {
        setResolution,
    }
}
