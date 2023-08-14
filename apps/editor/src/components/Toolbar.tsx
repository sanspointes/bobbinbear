import { useContext } from "solid-js";
import { TbPointer } from "solid-icons/tb";
import { ImCheckboxUnchecked } from 'solid-icons/im'

import * as helpers from "../store/helpers";
import { Button } from "./generics/Button";
import { AppContext } from "../store";
import { Tool } from "../store/toolStore";
import { CommandStack } from "./CommandStack";

export const Toolbar = () => {
  const app = useContext(AppContext);
  return (
    <div class="flex justify-between p-2 bg-orange-500 border-b border-orange-700 border-solid">
      <div class="flex gap-2">
        <Button onClick={() => helpers.createCanvas(app.dispatch)}>
          New Canvas
        </Button>
        <Button
          variant="default"
          highlighted={app.toolStore.tool === Tool.Select}
          onClick={() => app.dispatch("tool:switch", Tool.Select)}
        >
          <TbPointer class="stroke-orange-800 w-5 h-5" />
        </Button>
        <Button
          variant="default"
          highlighted={app.toolStore.tool === Tool.Box}
          onClick={() => app.dispatch("tool:switch", Tool.Box)}
        >
          <ImCheckboxUnchecked class="fill-orange-800 w-4 h-4" />
        </Button>
      </div>
      <div class="flex gap-2">
        <CommandStack stack={app.sceneStore.undoStack} />
      </div>
    </div>
  );
};
