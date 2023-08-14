import { Button as KButton } from "@kobalte/core";
import { mergeProps } from "solid-js";
import { tv } from "tailwind-variants";
import clsx from 'clsx';


const button = tv({
  base: "rounded-md",
  variants: {
    variant: {
      default: 'bg-orange-200 hover:bg-orange-100 text-orange-800 font-bold',
      secondary: 'bg-orange-200 bg-opacity-0 border-orange-600 border-solid border text-orange-800 hover:bg-opacity-20'
    },
    inverted: {
      true: 'bg-orange-50',
    },
    highlighted: {
      true: 'bg-orange-100 hover:bg-orange-50 shadow shadow-orange-500',
    },
    size: {
      small: 'p-2',
      medium: 'p-4',
      large: 'p-6',
    },
  },
  compoundVariants: [ 
    {
      variant: 'default',
      inverted: true,
      class: 'bg-orange-800 hover:bg-orange-700 text-orange-200'
    },
    {
      variant: 'secondary',
      inverted: true,
      class: 'bg-transparent border-orange-200 border-solid border text-orange-200 bg-orange-200',
    }
  ]
});


export type ButtonProps = KButton.ButtonRootProps & {
  variant?: "default" | "secondary";
  size?: "small" | "medium" | "large";
  inverted?: boolean,
  highlighted?: boolean,
};
const DEFAULT_PROPS: ButtonProps = {
  variant: "default",
  size: 'medium',
  inverted: false,
  highlighted: false,
};
export const Button = (p: ButtonProps) => {
  const props: ButtonProps = mergeProps(DEFAULT_PROPS, p);

  return (
    <KButton.Root
      {...props}
      class={clsx(button({ variant: props.variant, inverted: props.inverted, size: props.size }), props.class)}
    />
  );
};
