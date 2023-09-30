import { EmbState } from '../shared';
import { EmbText, fontFaceDescToGFVariant } from './shared';
import { P } from '@bearbroidery/solixi';
import { PixiInput } from '@/pixi-classes/PixiInput';
import { Show, createMemo, onMount } from 'solid-js';
import { Text } from '@pixi/text';
import { PGeometryText } from './GeometryText';
import { useGoogleFont } from './useGoogleFont';

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

    const googleFontResult = useGoogleFont(() => props.fontFace.fontFamily);
    const fontPath = createMemo(() => {
        if (googleFontResult.data) {
            const { files } = googleFontResult.data;
            const target = fontFaceDescToGFVariant(props.fontFace);
            if (files[target]) return files[target];
            else return files['regular'];
        }
    });

    return (
        <P.Container zIndex={props.order} position={props.position}>
            <Show when={fontPath()}>
                {(fontPath) => (
                    <PGeometryText
                        fontPath={fontPath()}
                        value={props.value}
                        interactive={true}
                        onclick={() => {
                            console.log('Clicked text');
                        }}
                    />
                )}
            </Show>
        </P.Container>
    );
}
