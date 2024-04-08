import { Input } from '../../components/ui/input';
import { useBobbinBear } from '../../hooks/useBobbinBear';
import { Card, CardTitle } from '../../components/ui/card';
import { Button } from '../../components/ui/button';

type NameProps = {
    uid: string;
    name: string | undefined;
};

export function Name(props: NameProps) {
    const { document } = useBobbinBear();
    const { setName, inspect } = document;
    return (
        <Card>
            <CardTitle class="mb-2">
                Name{' '}
                <span class="text-xs font-thin text-gray-400">{props.uid}</span>
            </CardTitle>
            <Input
                placeholder="Name"
                value={props.name}
                onChange={(e) => setName(props.uid, e.currentTarget.value)}
            />
            <Button onClick={() => inspect(props.uid)}>Inspect</Button>
        </Card>
    );
}
