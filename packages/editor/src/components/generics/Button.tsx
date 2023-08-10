import { Button as KButton } from "@kobalte/core";
import { mergeProps } from "solid-js";

export type ButtonProps = KButton.ButtonRootProps & {
  variant?: "default";
};
const DEFAULT_PROPS: ButtonProps = {
  variant: "default",
};
export const Button = (p: ButtonProps) => {
  const props: ButtonProps = mergeProps(p, DEFAULT_PROPS);

  return (
    <KButton.Root
      class="p-4 rounded-md"
      classList={{
        [props.class ?? '']: props.class !== undefined,
        "bg-gray-100 hover:bg-gray-200 text-gray-800":
          props.variant === "default",
 
      }}
      {...props}
    />
  );
};
