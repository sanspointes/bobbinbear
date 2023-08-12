import { useContext } from "solid-js";
import { TbPointer } from "solid-icons/tb";
import { CgSquare } from "solid-icons/cg";

import * as helpers from "../store/helpers";
import { Button } from "./generics/Button";
import { AppContext } from "../store";
import { Tool } from "../store/toolStore";
import { CommandStack } from "./CommandStack";

export const Toolbar = () => {
  const app = useContext(AppContext);
  return (
    <div class="flex justify-between p-2 bg-yellow-400 border-b border-yellow-500 border-solid">
      <div class="flex gap-2">
        <Button onClick={() => helpers.createCanvas(app.dispatch)}>
          New Canvas
        </Button>
        <Button
          variant="default"
          highlighted={app.toolStore.tool === Tool.Select}
          onClick={() => app.dispatch("tool:switch", Tool.Select)}
        >
          <TbPointer />
        </Button>
        <Button
          variant="default"
          highlighted={app.toolStore.tool === Tool.Box}
          onClick={() => app.dispatch("tool:switch", Tool.Box)}
        >
          <CgSquare />
        </Button>
      </div>
      <div class="flex gap-2">
        <CommandStack stack={app.sceneStore.undoStack} />
      </div>
    </div>
  );
};
