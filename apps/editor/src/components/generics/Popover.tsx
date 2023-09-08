import { Popover as KPopover } from "@kobalte/core";
import { TbX } from "solid-icons/tb";
import { JSX, splitProps } from "solid-js";

type PopoverProps = KPopover.PopoverRootProps & {
  title: string;
  trigger: JSX.Element;
  class?: string;
};
export function Popover(props: PopoverProps) {
  const [internalProps, remaining] = splitProps(props, [
    "title",
    "children",
    "trigger",
    "class",
  ]);

  return (
    <KPopover.Root {...remaining}>
      <KPopover.Trigger>
        {props.trigger}
      </KPopover.Trigger>
      <KPopover.Portal>
        <KPopover.Content
          class="overflow-hidden bg-orange-300 rounded-md min-w-[300px] z-[100]"
          classList={{
            [internalProps.class ?? ""]: !!internalProps.class,
          }}
        >
          <div class="flex justify-between p-2 w-full border-b border-orange-500 border-solid">
            <KPopover.Title>{internalProps.title}</KPopover.Title>
            <KPopover.CloseButton>
              <TbX />
            </KPopover.CloseButton>
          </div>
          <KPopover.Description>
            {props.children}
          </KPopover.Description>
        </KPopover.Content>
      </KPopover.Portal>
    </KPopover.Root>
  );
}
