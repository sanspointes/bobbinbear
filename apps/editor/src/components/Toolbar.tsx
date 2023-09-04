import { useContext } from "solid-js";
import { TbPointer } from "solid-icons/tb";
import { ImCheckboxUnchecked } from 'solid-icons/im'
import { FiCircle } from 'solid-icons/fi'
import { BiRegularPen } from 'solid-icons/bi'

import { Button } from "./generics/Button";
import { AppContext } from "../store";
import { Tool } from "../store/toolStore";
import { CommandStack } from "./CommandStack";
import { MainMenu } from "./MainMenu";

export const Toolbar = () => {
  const {toolStore, sceneStore, dispatch} = useContext(AppContext);
  return (
    <div class="flex justify-between p-2 bg-orange-500 border-b border-orange-700 border-solid">
      <div class="flex items-center gap-2">
        <MainMenu />
        <div class="h-full w-[1px] border-[0.5px] border-solid border-orange-300" />
        <Button
          variant="default"
          class="w-12 h-12"
          classList={{
            'outline outline-2 outline-orange-700': toolStore.tool === Tool.Select,
          }}
          highlighted={toolStore.tool === Tool.Select}
          onClick={() => dispatch("tool:switch", Tool.Select)}
        >
          <TbPointer class="stroke-orange-800 w-[22px] h-[22px]" />
        </Button>
        <Button
          variant="default"
          class="w-12 h-12 outline-2"
          classList={{
            'outline outline-2 outline-orange-700': toolStore.tool === Tool.Ellipse,
          }}
          highlighted={toolStore.tool === Tool.Ellipse}
          onClick={() => dispatch("tool:switch", Tool.Ellipse)}
        >
          <FiCircle class="fill-orange-800 w-4 h-4" />
        </Button>
        <Button
          variant="default"
          class="w-12 h-12 outline-2"
          classList={{
            'outline outline-2 outline-orange-700': toolStore.tool === Tool.Box,
          }}
          highlighted={toolStore.tool === Tool.Box}
          onClick={() => dispatch("tool:switch", Tool.Box)}
        >
          <ImCheckboxUnchecked class="fill-orange-800 w-4 h-4" />
        </Button>
        <Button
          variant="default"
          class="w-12 h-12"
          classList={{
            'outline outline-2 outline-orange-700': toolStore.tool === Tool.Pen,
          }}
          highlighted={toolStore.tool === Tool.Box}
          onClick={() => dispatch("tool:switch", Tool.Pen)}
        >
          <BiRegularPen class="fill-orange-800 w-[22px] h-[22px]" />
        </Button>
      </div>
      <div class="flex gap-2">
        <CommandStack stack={sceneStore.undoStack} />
      </div>
    </div>
  );
};
