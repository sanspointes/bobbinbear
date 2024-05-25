import { MainMenu } from "./MainMenu"

import LogoLightImage from '../../assets/logo__light.svg';
import LogoWinkLightImage from '../../assets/logo_wink__light.svg';
import Tools from "./Tools";

import { DebugApi, UndoRedoApi } from "bb_core";
import { Button } from "~/components/ui/button";

export function Toolbar() {
    const debugApi = new DebugApi();
    const undoRedoApi = new UndoRedoApi();

    return <div class="box-content flex justify-start items-center gap-2 p-2 bg-orange-500 border-b border-orange-700 border-solid">
        <MainMenu>
            <img
                src={LogoLightImage}
                class="block relative top-1 scale-150 w-14 h-auto group-hover:hidden drop-shadow-md shadow-orange-900"
            />
            <img
                src={LogoWinkLightImage}
                class="hidden relative top-1 scale-150 w-14 h-auto group-hover:block drop-shadow-md shadow-orange-900"
            />
        </MainMenu>
        <Tools class="ml-4" />
        <div class="w-full" />
        <Button variant='toolbar' onClick={() => debugApi.spawn_line()}>Spawn Line</Button>
        <Button variant='toolbar' onClick={() => debugApi.spawn_box()}>Spawn Box</Button>
        <Button variant='toolbar' onClick={() => undoRedoApi.undo()}>Undo</Button>
        <Button variant='toolbar' onClick={() => undoRedoApi.redo()}>Redo</Button>
    </div>
}
