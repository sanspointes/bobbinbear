import type { Component, JSX, ValidComponent } from "solid-js"
import { splitProps } from "solid-js"

import { PolymorphicProps } from "@kobalte/core/polymorphic"
import * as ProgressPrimitive from "@kobalte/core/progress"

import { Label } from "~/components/ui/label"

type ProgressRootProps = ProgressPrimitive.ProgressRootProps & { children?: JSX.Element }

const Progress = <T extends ValidComponent = "div">(
  props: PolymorphicProps<T, ProgressRootProps>
) => {
  const [local, others] = splitProps(props as ProgressRootProps, ["children"])
  return (
    <ProgressPrimitive.Root {...others}>
      {local.children}
      <ProgressPrimitive.Track class="relative h-4 w-full overflow-hidden rounded-full bg-secondary outline outline-2 outline-muted-foreground">
        <ProgressPrimitive.Fill class="h-full w-[var(--kb-progress-fill-width)] flex-1 bg-primary transition-all" />
      </ProgressPrimitive.Track>
    </ProgressPrimitive.Root>
  )
}

const ProgressLabel: Component<ProgressPrimitive.ProgressLabelProps> = (props) => {
  return <ProgressPrimitive.Label as={Label} {...props} />
}

const ProgressValueLabel: Component<ProgressPrimitive.ProgressValueLabelProps> = (props) => {
  return <ProgressPrimitive.ValueLabel as={Label} {...props} />
}

export { Progress, ProgressLabel, ProgressValueLabel }
