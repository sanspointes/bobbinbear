import { Button as KButton } from "@kobalte/core";
import { mergeProps } from "solid-js";
import { tv } from "tailwind-variants";
import clsx from 'clsx';


const button = tv({
  base: "rounded-md",
  variants: {
    variant: {
      default: 'bg-yellow-200 hover:bg-yellow-100 text-yellow-800 font-bold',
      secondary: 'bg-yellow-200 bg-opacity-0 border-yellow-600 border-solid border text-yellow-800 hover:bg-opacity-20'
    },
    inverted: {
      true: 'bg-yellow-50',
    },
    highlighted: {
      true: 'bg-yellow-100 hover:bg-yellow-50 shadow shadow-yellow-500',
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
      class: 'bg-yellow-800 hover:bg-yellow-700 text-yellow-200'
    },
    {
      variant: 'secondary',
      inverted: true,
      class: 'bg-transparent border-yellow-200 border-solid border text-yellow-200 bg-yellow-200',
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
