import { DocTreeContext, createDocTreeContext } from './createDocTreeState';
import { DocTreeHead } from './DocTreeHead';
import { DocTreeList } from './DocTreeList';

export function DocTree() {
    // eslint-disable-next-line solid/reactivity
    const ctx = createDocTreeContext();

    return (
        <DocTreeContext.Provider value={ctx}>
            <div>
                <DocTreeHead />
                <DocTreeList />
            </div>
        </DocTreeContext.Provider>
    );
}
