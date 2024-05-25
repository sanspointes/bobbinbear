import clsx from 'clsx';
import { TbPencil, TbPointer } from 'solid-icons/tb';
import { Button } from '~/components/ui/button';
import { useBobbinBear } from '~/hooks/useBobbinBear';

type ToolsProps = {
    class?: string;
}
export default function Tools(props: ToolsProps) {
    const { tools } = useBobbinBear();
    return (
        <div class={clsx("flex gap-2 justify-center items-center", props.class)}>
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
