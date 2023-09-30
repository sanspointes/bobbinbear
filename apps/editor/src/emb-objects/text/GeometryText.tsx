import { GeometryText, FontHandle } from '@bearbroidery/pixi-geometry-text';
import { Solixi, PropFragments } from '@bearbroidery/solixi';
import { Container } from '@pixi/display';
import { Show, createResource, splitProps } from 'solid-js';

const InternalGeometryText = Solixi.wrapConstructable(GeometryText, {
    attach: (_, parent, object) => {
        if (parent instanceof Container) {
            parent.addChild(object);
            return () => parent.removeChild(object);
        }
        throw new Error(
            'PGeometryText: Parent must be instance of PIXI.Container.',
        );
    },
    defaultArgs: () => {
        throw new Error('PGeometryText required args to be provided.');
    },
    extraProps: {
        ...PropFragments.HasPositionFragment,
        ...PropFragments.HasRotationFragment,
        ...PropFragments.HasScaleFragment,
        ...PropFragments.HasNameFragment,
        ...PropFragments.HasVisibilityFragment,
    },
});

type GeometryTextProps = Parameters<typeof InternalGeometryText>[0] & {
    fontPath: string;
};

export function PGeometryText(props: GeometryTextProps) {
    const [_, classProps] = splitProps(props, ['fontPath']);

    const [fontHandle] = createResource(
        () => props.fontPath,
        async (path) => {
            const handleResult = await FontHandle.fromUrl(path);
            if (handleResult.ok) return handleResult.value;
        },
    );

    return (
        <Show when={fontHandle()}>
            {(fontHandle) => (
                <InternalGeometryText
                    {...classProps}
                    args={[fontHandle(), true]}
                />
            )}
        </Show>
    );
}
