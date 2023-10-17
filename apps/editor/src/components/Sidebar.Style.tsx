import { createMemo, Show, useContext } from 'solid-js';
import { AccordionItem } from './generics/Accordian';
import { AppContext } from '../store';
import { Command, SetSceneObjectFieldCommand } from '../store/commands';
import { ColorPicker } from './generics/ColorPicker';
import { NumberInput } from './generics/NumberInput';
import { Select } from './generics/Select';
import {
    EmbHasFill,
    EmbObject,
    EmbVector,
    FillOptions,
    LineOptions,
} from '../emb-objects';
import { HslColor } from '../utils/color';

const LineCapText: Record<LINE_CAP, string> = {
    [LINE_CAP.BUTT]: 'Butt',
    [LINE_CAP.ROUND]: 'Round',
    [LINE_CAP.SQUARE]: 'Square',
} as const;

enum Alignment {
    Inside = 'Inside',
    Middle = 'Middle',
    Outside = 'Outside',
}
const AlignmentValue: Record<Alignment, number> = {
    [Alignment.Inside]: 0,
    [Alignment.Middle]: 0.5,
    [Alignment.Outside]: 1,
};

type SidebarStyleProps = {
    object: EmbObject & Partial<EmbHasFill>;
};
export function SidebarStyle(props: SidebarStyleProps) {
    const { dispatch } = useContext(AppContext);

    const updateFillStyle = (model: Partial<FillOptions>) => {
        const fill = { ...props.object.fill!, ...model };
        const cmd = new SetSceneObjectFieldCommand<EmbObject & EmbHasFill>(
            props.object.id,
            'fill',
            fill,
        );
        dispatch('scene:do-command', cmd as Command);
    };

    const updateStrokeStyle = (model: Partial<LineOptions>) => {
        const preStroke = (props.object as EmbVector).line;
        if (!preStroke) {
            throw new Error(
                "SidebarStyle: Can't update style on scene object without stroke field",
            );
        }
        const stroke = { ...preStroke, ...model };
        console.log(model, stroke);
        const cmd = new SetSceneObjectFieldCommand(
            props.object.id,
            // @ts-expect-error ; SetSceneObjectFieldCommand not typed to GraphicSceneObject
            'line',
            stroke,
        );
        dispatch('scene:do-command', cmd);
    };

    const onFillColorChange = (color: HslColor) => {
        updateFillStyle({
            color,
        });
    };

    const alignmentAsEnum = createMemo<Alignment | undefined>(() => {
        const stroke = (props.object as EmbVector).line;
        if (!stroke?.alignment) return undefined;
        if (stroke.alignment < 0.25) return Alignment.Inside;
        if (stroke.alignment > 0.75) return Alignment.Outside;
        return Alignment.Middle;
    });

    return (
        <AccordionItem
            value="style"
            header="Style"
            innerClass="grid grid-cols-2 gap-4"
        >
            <Show when={props.object.fill}>
                {(fillStyle) => (
                    <ColorPicker
                        class="col-span-2"
                        label="Fill"
                        color={fillStyle().color}
                        onChange={onFillColorChange}
                    />
                )}
            </Show>
            <Show when={(props.object as EmbVector).line}>
                {(lineStyle) => (
                    <>
                        <ColorPicker
                            label="Stroke"
                            class="col-span-2"
                            color={lineStyle().color}
                            onChange={(v) => updateStrokeStyle({ color: v })}
                        />
                        <NumberInput
                            label="Width"
                            class="col-span-2"
                            value={lineStyle().width ?? 1}
                            onChange={(v) => updateStrokeStyle({ width: v })}
                        />
                        <Select<LINE_CAP>
                            class="col-span-2"
                            multiple={false}
                            value={lineStyle().cap}
                            options={[
                                LINE_CAP.BUTT,
                                LINE_CAP.ROUND,
                                LINE_CAP.SQUARE,
                            ]}
                            onChange={(v) => updateStrokeStyle({ cap: v })}
                        >
                            {(item) => LineCapText[item]}
                        </Select>
                        <Select<Alignment>
                            class="col-span-2"
                            multiple={false}
                            value={alignmentAsEnum() ?? Alignment.Inside}
                            options={[
                                Alignment.Inside,
                                Alignment.Middle,
                                Alignment.Outside,
                            ]}
                            onChange={(v) =>
                                updateStrokeStyle({
                                    alignment: AlignmentValue[v],
                                })
                            }
                        >
                            {(item) => item}
                        </Select>
                    </>
                )}
            </Show>
        </AccordionItem>
    );
}
