import { DebugApi, ScheduleGraphDirection, ScheduleLabel } from 'bb_core';
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '../ui/dialog';
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from '../ui/select';
import { JSX, createEffect, createSignal } from 'solid-js';
import { Checkbox } from '../ui/checkbox';
import { Label } from '../ui/label';
import { MermaidGraph } from '../ui/mermaid-graph';

// When we have more modals we'll
type ScheduleGraphProps = {
    debugApi: DebugApi;
    children: JSX.Element;
};
export function ScheduleGraph(props: ScheduleGraphProps) {
    const options: ScheduleLabel[] = [
        'First',
        'PreUpdate',
        'Update',
        'PostUpdate',
        'Last',
    ];

    const [collapsedSingle, setCollapseSingle] = createSignal(true);
    const [graphDirection, setGraphDirection] =
        createSignal<ScheduleGraphDirection>('LeftRight');
    const [prettify, setPrettify] = createSignal(true);
    const [ambiguity, setAmbiguity] = createSignal(false);
    const [ambiguityOnWorld, setAmbiguityOnWorld] = createSignal(false);
    const [schedule, setSchedule] = createSignal<ScheduleLabel>('Update'); 

    const [graphSource, setGraphSource] = createSignal('');
    createEffect(() => {
        props.debugApi
            .graph_schedule(schedule(), {
                graph_direction: graphDirection(),
                collapse_single_system_sets: collapsedSingle(),
                prettify_system_names: prettify(),
                ambiguity_enable: ambiguity(),
                ambiguity_enable_on_world: ambiguityOnWorld(),
            })
            .then((graphString) => {
                setGraphSource(graphString);
            });
    });

    return (
        <Dialog>
            <DialogTrigger>{props.children}</DialogTrigger>
            <DialogContent class="max-w-full">
                <DialogHeader>
                    <DialogTitle>View Schedule Graphs</DialogTitle>
                </DialogHeader>
                <div class="flex gap-2 items-center">
                    <Select
                        options={options}
                        value={schedule()}
                        onChange={setSchedule}
                        itemComponent={(props) => (
                            <SelectItem item={props.item}>
                                {props.item.rawValue}
                            </SelectItem>
                        )}
                    >
                        <SelectTrigger aria-label="Fruit" class="w-[180px]">
                            <SelectValue<string>>
                                {(state) => state.selectedOption()}
                            </SelectValue>
                        </SelectTrigger>
                        <SelectContent />
                    </Select>
                    <Select
                        options={['TopBottom', 'LeftRight']}
                        value={graphDirection()}
                        onChange={setGraphDirection}
                        itemComponent={(props) => (
                            <SelectItem item={props.item}>
                                {props.item.rawValue}
                            </SelectItem>
                        )}
                    >
                        <SelectTrigger
                            aria-label="Schedule Graph Direction"
                            class="w-[180px]"
                        >
                            <SelectValue<string>>
                                {(state) => state.selectedOption()}
                            </SelectValue>
                        </SelectTrigger>
                        <SelectContent />
                    </Select>
                    <Checkbox
                        checked={collapsedSingle()}
                        onChange={setCollapseSingle}
                    />
                    <Label>Collapse Single</Label>
                    <Checkbox checked={prettify()} onChange={setPrettify} />
                    <Label>Prettify</Label>
                    <Checkbox checked={ambiguity()} onChange={setAmbiguity} />
                    <Label>Ambiguity</Label>
                    <Checkbox
                        checked={ambiguityOnWorld()}
                        onChange={setAmbiguityOnWorld}
                    />
                    <Label>Ambiguity in World</Label>
                </div>
                <div class="overflow-auto h-[1000px]">
                    <MermaidGraph graph={graphSource()} />
                </div>
            </DialogContent>
        </Dialog>
    );
}
