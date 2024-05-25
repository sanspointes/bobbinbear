import { UndoRedoApi } from 'bb_core';
import {
    DropdownMenu,
    DropdownMenuTrigger,
    DropdownMenuPortal,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuSeparator,
} from '../../components/ui/dropdown-menu';

import { JSX } from 'solid-js';

type MainMenuProps = {
    children: JSX.Element,
}
export function MainMenu(props: MainMenuProps) {
    const undoRedoApi = new UndoRedoApi();
    return (
        <DropdownMenu>
            <DropdownMenuTrigger class="group">
                {props.children}
            </DropdownMenuTrigger>
            <DropdownMenuPortal>
                <DropdownMenuContent class="p-1 bg-orange-200 rounded-md shadow-2xl min-w-[250px] shadow-orange-500">
                    <DropdownMenuItem class="relative p-2 rounded-md hover:bg-orange-100" onClick={() => undoRedoApi.undo()}>
                        Undo{' '}
                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                            ⌘+Z
                        </div>
                    </DropdownMenuItem>
                    <DropdownMenuItem class="relative p-2 rounded-md hover:bg-orange-100" onClick={() => undoRedoApi.redo()}>
                        Redo{' '}
                        <div class="absolute right-2 top-1/2 -translate-y-1/2">
                            ⌘+R
                        </div>
                    </DropdownMenuItem>
                    <DropdownMenuSeparator class="w-full border-t-0 border-b border-solid border-b-orange-500" />
                    <DropdownMenuItem class="relative p-2 rounded-md hover:bg-orange-100">
                        About Bobbin Bear
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenuPortal>
        </DropdownMenu>
    );
}
