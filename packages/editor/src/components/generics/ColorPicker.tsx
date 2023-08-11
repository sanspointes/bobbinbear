import { TextField as KTextField } from "@kobalte/core";
import { JSX, createMemo, splitProps } from "solid-js";

export type ColorPickerProps =
  & Omit<KTextField.TextFieldRootProps, "value" | "onChange">
  & {
    label: string,
    colorValue: number|undefined;
    onChange: (value: number|undefined) => void;
  };
export function ColorPicker(props: ColorPickerProps) {
  const [remainingProps, rootProps] = splitProps(props, [
    "label",
    "colorValue",
    "onChange",
  ]);
  const strValue = createMemo(() => {
    const v = remainingProps.colorValue ? remainingProps.colorValue.toString(16) : 'FFFFFF';
    console.log(v);
    // const a = remainingProps.opacityValue ? remainingProps.opacityValue.toString(16) : 'FF';
    return '#' + v;
  });
  const onChange: JSX.ChangeEventHandlerUnion<HTMLInputElement, Event> = (e) => {
    const value = e.target.value;
    console.log(value);
    props.onChange(Number.parseInt(value.replace('#', ''), 16));
  };
  return (
    <KTextField.Root
      {...rootProps}
      class="flex gap-2 items-center"
      classList={{
        [props.class ?? '']: props.class !== undefined
      }}
    >
      <KTextField.Label>{remainingProps.label}</KTextField.Label>
      <KTextField.Input type="color" value={strValue()} onChange={onChange} class="bg-white rounded-md w-full box-border h-8" />
      
    </KTextField.Root>
  );
}
