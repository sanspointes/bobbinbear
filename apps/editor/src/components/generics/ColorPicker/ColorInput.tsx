import { Select } from '../Select';
import { Match, Switch } from 'solid-js';
import { TextInput } from '../TextInput';
import {
    AllColorInputStrategies,
    ColorInputStrategy,
} from '@/store/settingsStore';
import {
    HsvColor,
    RgbColor,
    hexFromHsv,
    hsvFromRgb,
    rgbFromHsv,
} from '@/utils/color';
import { NumberInput } from '../NumberInput';

type ColorInputProps = {
    strategy: ColorInputStrategy;
    canChangeStrategy?: boolean;
    onStrategyChange?: (strategy: ColorInputStrategy) => void;
    color: HsvColor;
    onChange: (value: HsvColor) => void;
};
export function ColorInput(props: ColorInputProps) {
    const handleHsvChange = (key: keyof HsvColor, value: number) => {
        const rgb = {
            ...props.color,
            [key]: value,
        };
        props.onChange(rgb);
    };
    const handleRgbChange = (key: keyof RgbColor, value: number) => {
        const rgb = {
            ...rgbFromHsv(props.color),
            [key]: value,
        };
        props.onChange(hsvFromRgb(rgb));
    };
    return (
        <div class="flex items-center p-2">
            <Select
                class="w-16"
                value={props.strategy}
                onChange={props.onStrategyChange}
                options={AllColorInputStrategies}
            >
                {(v) => v}
            </Select>
            <Switch>
                <Match when={props.strategy === ColorInputStrategy.Hex}>
                    <TextInput
                        class="flex-grow"
                        label="Hex Value"
                        hideLabel={true}
                        value={hexFromHsv(props.color)}
                    />
                </Match>
                <Match when={props.strategy === ColorInputStrategy.Rgb}>
                    <NumberInput
                        class="flex-grow"
                        label="Red"
                        hideLabel={true}
                        precision={0}
                        min={0}
                        max={255}
                        value={rgbFromHsv(props.color).r}
                        onChange={(v) => handleRgbChange('r', v)}
                    />
                    <NumberInput
                        class="flex-grow"
                        label="Green"
                        hideLabel={true}
                        precision={0}
                        min={0}
                        max={255}
                        value={rgbFromHsv(props.color).g}
                        onChange={(v) => handleRgbChange('g', v)}
                    />
                    <NumberInput
                        class="flex-grow"
                        label="Blue"
                        hideLabel={true}
                        precision={0}
                        min={0}
                        max={255}
                        value={rgbFromHsv(props.color).b}
                        onChange={(v) => handleRgbChange('b', v)}
                    />
                </Match>
                <Match when={props.strategy === ColorInputStrategy.Hsv}>
                    <NumberInput
                        class="flex-grow"
                        label="Hue"
                        hideLabel={true}
                        min={0}
                        max={360}
                        value={props.color.h}
                        onChange={(v) => handleHsvChange('h', v)}
                    />
                    <NumberInput
                        class="flex-grow"
                        label="Saturation"
                        hideLabel={true}
                        min={0}
                        max={100}
                        value={props.color.s}
                        onChange={(v) => handleHsvChange('s', v)}
                    />
                    <NumberInput
                        class="flex-grow"
                        label="Value"
                        hideLabel={true}
                        min={0}
                        max={100}
                        value={props.color.v}
                        onChange={(v) => handleHsvChange('v', v)}
                    />
                </Match>
            </Switch>
        </div>
    );
}
