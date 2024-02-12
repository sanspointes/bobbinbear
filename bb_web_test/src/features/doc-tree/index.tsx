import { Api } from 'bb_core';
import { createDocTreeState } from './createDocTreeState';
import { Button } from '../../components/button';

type DocTreeProps = {
    api: Api;
};
export function DocTree(props: DocTreeProps) {
    // eslint-disable-next-line solid/reactivity
    const [data, { refresh }] = createDocTreeState(props.api);

    return (
        <div>
            <Button onClick={refresh}>Refresh</Button>
            {JSON.stringify(data())}
        </div>
    );
}
