import { Button as KButton, } from '@kobalte/core';
import { mergeProps } from 'solid-js';

export type ButtonProps = KButton.ButtonRootProps & {
  variant?: 'default'
}
const DEFAULT_PROPS: ButtonProps = {
  variant: 'default',
}
export const Button = (p: ButtonProps) => {
  const props: ButtonProps = mergeProps(p, DEFAULT_PROPS)

  return (
    <KButton.Root class='p-4 rounded-md' classList={{
      'bg-gray-50 text-gray-800': props.variant === 'default',
    }} {...props} />
  )
}
