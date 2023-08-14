import { For, Show } from "solid-js";
import { Collapsible as KCollapsible } from "@kobalte/core";
import { TbChevronDown } from "solid-icons/tb";
import clsx from 'clsx';

import { Command, MultiCommand } from "../store/commands";
import { Popover } from "./generics/Popover";
import { Button } from "./generics/Button";
import { BaseSceneObject } from "../types/scene";

export type CommandStackProps = {
  stack: Command[];
  class?: string;
};
export function CommandStack(props: CommandStackProps) {
  return (
    <Popover
      trigger={<Button variant="secondary" inverted={true}>Command Stack ({props.stack.length})</Button>}
      title="Command Stack"
      class={clsx("overflow-y-scroll pr-2 w-[600px] max-h-[400px]", props.class)}
    >
      <For each={props.stack}>
        {(command) => <CommandStackRow command={command} />}
      </For>
    </Popover>
  );
}

type CommandStackRowProps = {
  command: Command;
};
function CommandStackRow(props: CommandStackRowProps) {
  return (
    <KCollapsible.Root class="w-full">
      <div class="flex gap-2 items-center p-2 pr-0 border-b border-orange-500 border-solid">
        <KCollapsible.Trigger>
          <Button size="small">
            <TbChevronDown class="ml-auto transition-transform transform kb-expanded:rotate-180" />
          </Button>
        </KCollapsible.Trigger>
        {props.command.name} -{" "}
        {props.command.updatable ? "updatable" : "not updatable"} -{" "}
        {props.command.final ? "final" : "not final"}
        <Show when={props.command.error}>
        </Show>
      </div>

      <KCollapsible.Content class="pl-6">
        <Show
          when={props.command.type === "MultiCommand"}
          fallback={
            <pre class="text-xs bg-orange-200">{JSON.stringify(props.command, undefined, 2)}</pre>
          }
        >
          <For each={(props.command as MultiCommand<BaseSceneObject>).commands}>
            {(command) => <CommandStackRow command={command} />}
          </For>
        </Show>
      </KCollapsible.Content>
    </KCollapsible.Root>
  );
}
