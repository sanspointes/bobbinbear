import { Input } from '../../components/ui/input';
import { useBobbinBear } from '../../hooks/useBobbinBear';
import { Card, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { TbFocus, TbX } from 'solid-icons/tb';
import { Show } from 'solid-js';
import { ObjectType } from 'bb_core';

type NameProps = {
    uid: string;
    ty: ObjectType,
    name: string | undefined;
};

export function Name(props: NameProps) {
    const { document } = useBobbinBear();
    const { setName, inspect, deleteObject } = document;
    return (
        <Card>
            <CardTitle class="flex justify-between items-center mb-2">
                Name{' '}
                <div class="flex gap-2 items-center">
                    <Show when={props.ty === 'Vector'}>
                        <Button
                            variant='outline'
                            size="sm"
                            onClick={() => inspect(props.uid)}
                        >
                            <TbFocus /> Inspect
                        </Button>
                    </Show>
                    <Button
                        variant='destructive'
                        size="sm"
                        onClick={() => deleteObject(props.uid)}
                    >
                        <TbX /> Delete
                    </Button>
                </div>
            </CardTitle>

            <span class="text-xs font-thin text-gray-400">{props.uid}</span>
            <Input
                placeholder="Name"
                value={props.name}
                onChange={(e) => setName(props.uid, e.currentTarget.value)}
            />
        </Card>
    );
}
