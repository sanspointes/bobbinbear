import { P } from '@bearbroidery/solixi';
import { EmbGroup } from '.';
import { SceneObjectChildren } from '..';

type EmbGroupProps = EmbGroup & {
    order: number;
};

export function EmbGroupView(props: EmbGroupProps) {
    return (
        <P.Container
            name={`${props.name}-${props.id}`}
            zIndex={props.order}
            visible={props.visible}
            position={props.position}
            interactive={false}
        >
            <SceneObjectChildren children={props.children} />
        </P.Container>
    );
}
