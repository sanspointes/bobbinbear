import { DropdownMenu } from '@kobalte/core';
import { RiUserFacesBearSmileFill } from 'solid-icons/ri';
import LogoImage from '../assets/logo.svg';
import LogoWinkImage from '../assets/logo_wink.svg';

export function MainMenu() {
    return (
        <DropdownMenu.Root>
            <DropdownMenu.Trigger class='group'>
                <img src={LogoImage} class='w-14 h-auto block group-hover:hidden drop-shadow-md shadow-orange-900' />
                <img src={LogoWinkImage} class='w-14 h-auto hidden group-hover:block drop-shadow-md shadow-orange-900' />
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
                <DropdownMenu.Content class="bg-orange-200 min-w-[250px] p-1 rounded-md shadow-2xl shadow-orange-500">
                    <DropdownMenu.Item class="relative hover:bg-orange-100 p-2 rounded-md">
                        None of these do anything yet...
                    </DropdownMenu.Item>
                    <DropdownMenu.Item class="relative hover:bg-orange-100 p-2 rounded-md">
                        New{' '}
                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                            ⌘+N
                        </div>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item class="relative hover:bg-orange-100 p-2 rounded-md">
                        Undo{' '}
                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                            ⌘+Z
                        </div>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item class="relative hover:bg-orange-100 p-2 rounded-md">
                        Redo{' '}
                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                            ⌘+R
                        </div>
                    </DropdownMenu.Item>
                    <DropdownMenu.Separator class="w-full border-t-0 border-b-orange-500 border-b border-solid" />
                    <DropdownMenu.Item class="relative hover:bg-orange-100 p-2 rounded-md">
                        About Bobbin Bear
                    </DropdownMenu.Item>
                </DropdownMenu.Content>
            </DropdownMenu.Portal>
        </DropdownMenu.Root>
    );
}
