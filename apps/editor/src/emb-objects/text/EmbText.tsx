import { PTextInput } from '@/sxi-components/PTextInput';
import { EmbState } from '../shared';
import { EmbText } from './shared';
import { P } from '@bearbroidery/solixi';
import { PixiInput } from '@/pixi-classes/PixiInput';
import { onMount } from 'solid-js';
import { Text } from '@pixi/text';

type EmbTextProps = EmbText &
    EmbState & {
        order: number;
    };

export function EmbTextView(props: EmbTextProps) {
    let textInput: PixiInput | undefined;
    let graphicsHitbox: Text | undefined;

    onMount(() => {
        if (!textInput || graphicsHitbox) return;
    });

    return (
        <P.Container zIndex={props.order} position={props.position}>
            <P.Text
                ref={graphicsHitbox}
                visible={!props.inspecting}
                interactive={!props.inspecting}
                text={props.value}
            />
            <PTextInput
                ref={textInput}
                id={props.id}
                soType={props.type}
                alpha={1}
                interactive={props.inspecting}
                value={props.value}
            >
                {/*<P.Text text={`${props.node.type}:${props.id} ${props.order}`} scale={[5, 5]} /> */}
            </PTextInput>
        </P.Container>
    );
}
