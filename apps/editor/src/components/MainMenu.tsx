import { DropdownMenu } from "@kobalte/core";
import { RiUserFacesBearSmileFill } from 'solid-icons/ri'

export function MainMenu() {
  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        <RiUserFacesBearSmileFill class="fill-orange-100 w-12 h-12 -mr-1" />
      </DropdownMenu.Trigger>
      <DropdownMenu.Portal>
        <DropdownMenu.Content class="bg-orange-200 min-w-[250px] p-1 rounded-md shadow-2xl shadow-orange-500">
          <DropdownMenu.Item class="relative hover:bg-orange-100 p-2 rounded-md">
            None of these do anything yet...
          </DropdownMenu.Item>
          <DropdownMenu.Item class="relative hover:bg-orange-100 p-2 rounded-md">
            New <div class="absolute right-2 top-1/2 -translate-y-1/2">⌘+N</div>
          </DropdownMenu.Item>
          <DropdownMenu.Item class="relative hover:bg-orange-100 p-2 rounded-md">
            Undo <div class="absolute right-2 top-1/2 -translate-y-1/2">⌘+Z</div>
          </DropdownMenu.Item>
          <DropdownMenu.Item class="relative hover:bg-orange-100 p-2 rounded-md">
            Redo <div class="absolute right-2 top-1/2 -translate-y-1/2">⌘+R</div>
          </DropdownMenu.Item>
          <DropdownMenu.Separator class="w-full border-t-0 border-b-orange-500 border-b border-solid" />
          <DropdownMenu.Item class="relative hover:bg-orange-100 p-2 rounded-md">About Bobbin Bear</DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  )
}
