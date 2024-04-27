import { Input } from '../../components/ui/input';
import { useBobbinBear } from '../../hooks/useBobbinBear';
import { Card, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';
import { TbFocus, TbX } from 'solid-icons/tb';

type NameProps = {
    uid: string;
    name: string | undefined;
};

export function Name(props: NameProps) {
    const { document } = useBobbinBear();
    const { setName, inspect, deleteObject } = document;
    return (
        <Card>
            <CardTitle class="mb-2">
                Name{' '}
                <Button
                    size="sm"
                    class="bg-yellow-600 hover:bg-yellow-700"
                    onClick={() => inspect(props.uid)}
                >
                    <TbFocus />
                </Button>
                <Button
                    size="sm"
                    class="bg-red-600 hover:bg-red-700"
                    onClick={() => deleteObject(props.uid)}
                >
                    <TbX />
                </Button>
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
