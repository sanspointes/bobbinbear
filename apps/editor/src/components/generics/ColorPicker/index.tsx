import { createEffect, createMemo, useContext } from 'solid-js';
import { Popover } from '../Popover';
import clsx from 'clsx';
import { SVSwatch } from './SVSwatch';
import { HueSwatch } from './HueSwatch';
import {
    HslColor,
    HsvColor,
    hslFromHsv,
    hslToCssString,
    hsvFromHsl,
} from '../../../utils/color';
import { ColorInput } from './ColorInput';
import { AppContext } from '@/store';

export type ColorPickerProps = {
    class?: string;
    label: string;
    color: HslColor;
    onChange: (value: HslColor) => void;
};
export function ColorPicker(props: ColorPickerProps) {
    const hsv = createMemo(() => hsvFromHsl(props.color));

    const { settingsStore, dispatch } = useContext(AppContext);

    const handleNewColor = (hsv: HsvColor) => {
        const hsl = hslFromHsv(hsv);
        props.onChange(hsl);
    };

    return (
        <div class={clsx('flex gap-4', props.class)}>
            <span class="text-orange-50">{props.label}</span>
            <Popover
                trigger={
                    <div
                        class="w-8 h-8 rounded-md outline-white outline-2 hover:outline"
                        style={{
                            'background-color': hslToCssString(props.color),
                        }}
                    />
                }
                title="Color"
                class="w-[250px]"
            >
                <SVSwatch
                    class="w-full aspect-square"
                    color={hsv()}
                    onChange={handleNewColor}
                />
                <HueSwatch
                    class="w-full h-8"
                    color={hsv()}
                    onChange={handleNewColor}
                />
                <ColorInput
                    strategy={settingsStore.colorInputStrategy}
                    onStrategyChange={(v) =>
                        dispatch('settings:set-color-input-strategy', v)
                    }
                    color={hsv()}
                    onChange={handleNewColor}
                />
            </Popover>
        </div>
    );
}
