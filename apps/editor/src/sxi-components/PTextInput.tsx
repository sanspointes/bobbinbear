import { Solixi, PropFragments } from '@bearbroidery/solixi';
import { Container } from '@pixi/display';
import { PixiInput } from '@/pixi-classes/PixiInput';
import { Sprite } from '@pixi/sprite';
import { Texture } from '@pixi/core';

// TODO: Upstream type fixes to pixi ui
// + flesh out the input component to support tap at any location to edit their etc.

export const PTextInput = Solixi.wrapConstructable(PixiInput, {
    attach: (_, parent, object) => {
        if (parent instanceof Container) {
            parent.addChild(object);
        }
        return () => {
            if (parent instanceof Container) parent.removeChild(object);
        };
    },
    defaultArgs: () => {
        const bg = new Sprite(Texture.WHITE);
        return [{ bg }] as ConstructorParameters<typeof PixiInput>;
    },
    extraProps: {
        ...PropFragments.HasNameFragment,
        ...PropFragments.HasPositionFragment,
        ...PropFragments.HasRotationFragment,
        ...PropFragments.HasScaleFragment,
        ...PropFragments.HasVisibilityFragment,
    },
});
