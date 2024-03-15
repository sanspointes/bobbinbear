import { Input } from '../../components/ui/input';
import { useBobbinBear } from '../../hooks/useBobbinBear';
import { Card, CardTitle } from '../../components/ui/card';

type PositionProps = {
    uid: string;
    position: [number, number];
};

export function Position(props: PositionProps) {
    const { document } = useBobbinBear();
    const { setPosition } = document;

    const validateNumberString = (value: string) => {
        const num = Number.parseInt(value);
        if (!Number.isNaN(num) && Number.isFinite(num)) return num;
        else return 0;
    };

    return (
        <Card>
            <CardTitle class="mb-2">Position</CardTitle>
            <div class="flex">
                <Input
                    type="number"
                    value={props.position[0]}
                    onInput={(e) =>
                        setPosition(
                            props.uid,
                            validateNumberString(e.currentTarget.value),
                            props.position[1],
                        )
                    }
                />
                <Input
                    type="number"
                    value={props.position[1]}
                    onInput={(e) =>
                        setPosition(
                            props.uid,
                            props.position[0],
                            validateNumberString(e.currentTarget.value),
                        )
                    }
                />
            </div>
        </Card>
    );
}
