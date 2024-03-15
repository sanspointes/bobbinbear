import { Input } from '../../components/ui/input';
import { useBobbinBear } from '../../hooks/useBobbinBear';
import { Card, CardTitle } from '../../components/ui/card';

type NameProps = {
    uid: string;
    name: string | undefined;
};

export function Name(props: NameProps) {
    const { document } = useBobbinBear();
    const { setName } = document;
    return (
        <Card>
            <CardTitle class="mb-2">Name</CardTitle>
            <Input
                placeholder="Name"
                value={props.name}
                onChange={(e) => setName(props.uid, e.currentTarget.value)}
            />
        </Card>
    );
}
