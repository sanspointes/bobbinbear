import { TbPointer } from 'solid-icons/tb';
import { ImCheckboxUnchecked } from 'solid-icons/im';

import { Button } from './generics/Button';
import { MainMenu } from './MainMenu';
import { useAppStore } from '@/stores';
import { BBTool } from '@bearbroidery/bobbinbear-core';

export const Toolbar = () => {
    const [{ core }, { core: coreApi }] = useAppStore();
    return (
        <div class="flex justify-between p-2 bg-orange-500 border-b border-orange-700 border-solid">
            <div class="flex items-center gap-2">
                <MainMenu />
                <div class="h-full w-[1px] border-[0.5px] border-solid border-orange-300" />
                <Button
                    variant="default"
                    class="w-12 h-12"
                    classList={{
                        'outline outline-2 outline-orange-700':
                            core.currentTool === BBTool.Select,
                    }}
                    highlighted={core.currentTool === BBTool.Select}
                    onClick={() => coreApi.setTool(BBTool.Select)}
                >
                    <TbPointer class="stroke-orange-800 w-[22px] h-[22px]" />
                </Button>
                <Button
                    variant="default"
                    class="w-12 h-12 outline-2"
                    classList={{
                        'outline outline-2 outline-orange-700':
                            core.currentTool === BBTool.Box,
                    }}
                    highlighted={core.currentTool === BBTool.Box}
                    onClick={() => coreApi.setTool(BBTool.Box)}
                >
                    <ImCheckboxUnchecked class="fill-orange-800 w-4 h-4" />
                </Button>
            </div>
        </div>
    );
};
