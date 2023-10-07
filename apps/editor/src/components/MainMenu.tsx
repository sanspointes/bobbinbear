import { DropdownMenu } from '@kobalte/core';
import LogoImage from '../assets/logo.svg';
import LogoWinkImage from '../assets/logo_wink.svg';

export function MainMenu() {
    return (
        <DropdownMenu.Root>
            <DropdownMenu.Trigger class="group">
                <img
                    src={LogoImage}
                    class="block w-14 h-auto group-hover:hidden drop-shadow-md shadow-orange-900"
                />
                <img
                    src={LogoWinkImage}
                    class="hidden w-14 h-auto group-hover:block drop-shadow-md shadow-orange-900"
                />
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
                <DropdownMenu.Content class="p-1 bg-orange-200 rounded-md shadow-2xl min-w-[250px] shadow-orange-500">
                    <DropdownMenu.Item class="relative p-2 rounded-md hover:bg-orange-100">
                        None of these do anything yet...
                    </DropdownMenu.Item>
                    <DropdownMenu.Item class="relative p-2 rounded-md hover:bg-orange-100">
                        New{' '}
                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                            ⌘+N
                        </div>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item class="relative p-2 rounded-md hover:bg-orange-100">
                        Undo{' '}
                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                            ⌘+Z
                        </div>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item class="relative p-2 rounded-md hover:bg-orange-100">
                        Redo{' '}
                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                            ⌘+R
                        </div>
                    </DropdownMenu.Item>
                    <DropdownMenu.Separator class="w-full border-t-0 border-b border-solid border-b-orange-500" />
                    <DropdownMenu.Item class="relative p-2 rounded-md hover:bg-orange-100">
                        About Bobbin Bear
                    </DropdownMenu.Item>
                </DropdownMenu.Content>
            </DropdownMenu.Portal>
        </DropdownMenu.Root>
    );
}
