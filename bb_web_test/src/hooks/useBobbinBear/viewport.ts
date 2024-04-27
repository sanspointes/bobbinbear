import { ViewportApi } from "bb_core";

export function useBBViewport() {
    const viewportApi = new ViewportApi();

    const setResolution = (width: number, height: number) => {
        return viewportApi.set_resolution(width, height);
    }
    
    return {
        setResolution,
    }
}
