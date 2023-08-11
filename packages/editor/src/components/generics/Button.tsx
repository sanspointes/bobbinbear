import { Button as KButton } from "@kobalte/core";
import { mergeProps } from "solid-js";

export type ButtonProps = KButton.ButtonRootProps & {
  variant?: "default" | "highlighted" ;
};
const DEFAULT_PROPS: ButtonProps = {
  variant: "default",
};
export const Button = (p: ButtonProps) => {
  const props: ButtonProps = mergeProps(DEFAULT_PROPS, p);

  return (
    <KButton.Root
      class="p-4 rounded-md text-yellow-900"
      classList={{
        [props.class ?? ""]: props.class !== undefined,
        "bg-yellow-200 hover:bg-yellow-100":
          props.variant === "default",
        "bg-yellow-100 hover:bg-yellow-50 shadow shadow-yellow-500": props.variant === "highlighted",
      }}
      {...props}
    />
  );
};
