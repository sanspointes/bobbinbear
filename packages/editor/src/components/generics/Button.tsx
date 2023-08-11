import { Button as KButton } from "@kobalte/core";
import { mergeProps } from "solid-js";

export type ButtonProps = KButton.ButtonRootProps & {
  variant?: "default" | "highlighted";
  size?: "small" | "medium" | "large";
};
const DEFAULT_PROPS: ButtonProps = {
  variant: "default",
  size: 'medium',
};
export const Button = (p: ButtonProps) => {
  const props: ButtonProps = mergeProps(DEFAULT_PROPS, p);

  return (
    <KButton.Root
      class="text-yellow-900 rounded-md"
      classList={{
        [props.class ?? ""]: props.class !== undefined,
        "bg-yellow-200 hover:bg-yellow-100": props.variant === "default",
        "bg-yellow-100 hover:bg-yellow-50 shadow shadow-yellow-500":
          props.variant === "highlighted",
        "p-2": props.size === 'small',
        "p-4": props.size === 'medium',
        "p-6": props.size === 'large',
      }}
      {...props}
    />
  );
};
