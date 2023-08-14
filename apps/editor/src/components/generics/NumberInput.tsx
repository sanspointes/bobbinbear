import { TextField as KTextField } from "@kobalte/core";
import { createMemo, splitProps } from "solid-js";

export type NumberInputProps =
  & Omit<KTextField.TextFieldRootProps, "value" | "onChange">
  & {
    value: number;
    onChange: (value: number) => void;

    precision?: number;
    label: string;
    description?: string;
  };
export function NumberInput(props: NumberInputProps) {
  const [remainingProps, rootProps] = splitProps(props, [
    "label",
    "value",
    "onChange",
  ]);
  const strValue = createMemo(() => {
    return remainingProps.value.toFixed(props.precision ?? 2);
  });
  const onChange = (value: string) => {
    props.onChange(Number.parseFloat(value));
  };
  return (
    <KTextField.Root
      {...rootProps}
      value={strValue()}
      onChange={onChange}
      class="flex gap-2 items-center"
      classList={{
        [props.class ?? '']: props.class !== undefined
      }}
    >
      <KTextField.Label class="text-orange-50">{remainingProps.label}</KTextField.Label>
      <KTextField.Input type="number" class="bg-white rounded-md w-full p-2 box-border text-orange-900" />
    </KTextField.Root>
  );
}
