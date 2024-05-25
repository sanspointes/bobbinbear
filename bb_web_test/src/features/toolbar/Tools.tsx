import { TbPencil, TbPointer } from 'solid-icons/tb';
import { Button } from '~/components/ui/button';
import { useBobbinBear } from '~/hooks/useBobbinBear';

export default function Tools() {
    const { tools } = useBobbinBear();
    return (
        <div class="flex gap-2 justify-center items-center">
            <div class="h-full border-orange-300 border-solid w-[1px] border-[0.5px]" />
            <Button
                variant={tools.currentTool() === 'Select' ? 'toolbar-active' : 'toolbar'}
                class="w-12 h-12"
                onClick={() => tools.switchTool('Select')}
            >
                <TbPointer class="w-[22px] h-[22px]" />
            </Button>
            <Button
                variant={tools.currentTool() === 'Pen' ? 'toolbar-active' : 'toolbar'}
                class="w-12 h-12"
                onClick={() => tools.switchTool('Pen')}
            >
                <TbPencil class="w-5 h-5" />
            </Button>
        </div>
    );
}
