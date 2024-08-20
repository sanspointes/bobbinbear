import { DocTreeHead } from './DocTreeHead';
import { DocTreeList } from './DocTreeList';

export function DocTree() {
    return (
        <div class='h-full overflow-y-scroll'>
            <DocTreeHead />
            <DocTreeList />
        </div>
    );
}
