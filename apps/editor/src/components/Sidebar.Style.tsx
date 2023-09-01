import { createMemo, JSX, Show, useContext } from "solid-js";
import {
  EmbBase,
  EmbVector,
  EmbHasFill,
} from "../emb-objects/shared";
import { AccordionItem } from "./generics/Accordian";
import { AppContext } from "../store";
import { SetSceneObjectFieldCommand } from "../store/commands";
import { ColorPicker } from "./generics/ColorPicker";
import { IFillStyleOptions, ILineStyleOptions, LINE_CAP } from "@pixi/graphics";
import { NumberInput } from "./generics/NumberInput";
import { Select } from "./generics/Select";
import { Uuid } from "../utils/uuid";

const LineCapText: Record<LINE_CAP, string> = {
  [LINE_CAP.BUTT]: "Butt",
  [LINE_CAP.ROUND]: "Round",
  [LINE_CAP.SQUARE]: "Square",
} as const;

enum Alignment {
  Inside = "Inside",
  Middle = "Middle",
  Outside = "Outside",
}
const AlignmentValue: Record<Alignment, number> = {
  [Alignment.Inside]: 0,
  [Alignment.Middle]: 0.5,
  [Alignment.Outside]: 1,
};

type SidebarStyleProps = {
  object: EmbBase & EmbHasFill;
};
export function SidebarStyle(props: SidebarStyleProps) {
  const { dispatch } = useContext(AppContext);

  const updateFillStyle = (model: Partial<IFillStyleOptions>) => {
    const fill = { ...props.object.fill, ...model };
    const cmd = new SetSceneObjectFieldCommand<
      EmbBase & EmbHasFill
    >(props.object.id as unknown as Uuid<EmbBase & EmbHasFill>, "fill", fill);
    dispatch("scene:do-command", cmd);
  };

  const updateStrokeStyle = (model: Partial<ILineStyleOptions>) => {
    const preStroke = (props.object as EmbVector).stroke;
    if (!preStroke) {
      throw new Error(
        "SidebarStyle: Can't update style on scene object without stroke field",
      );
    }
    const stroke = { ...preStroke, ...model };
    const cmd = new SetSceneObjectFieldCommand(
      props.object.id,
      // @ts-expect-error ; SetSceneObjectFieldCommand not typed to GraphicSceneObject
      "stroke",
      stroke,
    );
    dispatch("scene:do-command", cmd);
  };

  const onFillColorChange = (color: number | undefined) => {
    updateFillStyle({
      color,
    });
  };

  const alignmentAsEnum = createMemo<Alignment | undefined>(() => {
    const stroke = (props.object as EmbVector).stroke;
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
        {(fill) => (
          <ColorPicker
            class="col-span-2"
            label="Fill"
            colorValue={fill().color}
            onChange={onFillColorChange}
          />
        )}
      </Show>
      <Show when={(props.object as EmbVector).stroke}>
        {(stroke) => (
          <>
            <ColorPicker
              label="Stroke"
              colorValue={stroke().color}
              onChange={(v) => updateStrokeStyle({ color: v })}
            />
            <NumberInput
              label="Width"
              value={stroke().width ?? 1}
              onChange={(v) => updateStrokeStyle({ width: v })}
            />
            <Select<LINE_CAP>
              class="col-span-2"
              multiple={false}
              value={stroke().cap}
              options={[LINE_CAP.BUTT, LINE_CAP.ROUND, LINE_CAP.SQUARE]}
              onChange={(v) => updateStrokeStyle({ cap: v })}
            >
              {(item) => LineCapText[item]}
            </Select>
            <Select<Alignment>
              class="col-span-2"
              multiple={false}
              value={alignmentAsEnum() ?? Alignment.Inside}
              options={[Alignment.Inside, Alignment.Middle, Alignment.Outside]}
              onChange={(v) =>
                updateStrokeStyle({ alignment: AlignmentValue[v] })}
            >
              {(item) => item}
            </Select>
          </>
        )}
      </Show>
    </AccordionItem>
  );
}
