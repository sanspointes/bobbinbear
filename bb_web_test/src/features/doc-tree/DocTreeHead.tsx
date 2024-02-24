import { Button } from '../../components/button';
import { useDocTreeContext } from './createDocTreeState';

export function DocTreeHead() {
    const [_, { refresh }] = useDocTreeContext();

    return (
        <div class="flex gap-2 items-center">
            <Button onClick={refresh}>Refresh</Button>
        </div>
    );
}
