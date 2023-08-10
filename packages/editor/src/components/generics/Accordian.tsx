import { splitProps } from "solid-js";
import { Accordion as KAccordion } from "@kobalte/core";
import { TbChevronRight } from "solid-icons/tb";
import { TbChevronDown } from "solid-icons/tb";

type AccordionProps = KAccordion.AccordionRootProps;
export function Accordion(props: AccordionProps) {
  return (
    <KAccordion.Root {...props}>
    </KAccordion.Root>
  );
}

type AccordionItemProps = KAccordion.AccordionItemProps & {
  header: string;
  innerClass?: string;
};
export function AccordionItem(props: AccordionItemProps) {
  const [remainingProps, itemProps] = splitProps(props, [
    "children",
    "header",
    "innerClass",
    "class",
  ]);
  return (
    <KAccordion.Item
      {...itemProps}
      class="border-t first-of-type:border-t-0 last-of-type:border-b border-yellow-500 border-solid"
      classList={{
        [remainingProps.class ?? ""]: !!remainingProps.class,
      }}
    >
      <KAccordion.Header class="py-2">
        <KAccordion.Trigger class="flex justify-between items-center w-full font-bold">
          {remainingProps.header}{" "}
          <TbChevronDown class="w-6 h-6 transform transition-transform kb-expanded:rotate-180" />
        </KAccordion.Trigger>
      </KAccordion.Header>
      <KAccordion.Content
        class="pb-4"
        classList={{
          [remainingProps.innerClass ?? ""]: !!remainingProps.innerClass,
        }}
      >
        {remainingProps.children}
      </KAccordion.Content>
    </KAccordion.Item>
  );
}
