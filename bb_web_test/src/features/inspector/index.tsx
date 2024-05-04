import { Show, createMemo } from 'solid-js';
import { useBobbinBear } from '../../hooks/useBobbinBear';
import { Name } from './Name';
import { Position } from './Position';

export function Inspector() {
    const { document } = useBobbinBear();
    const { selectedObject } = document;
    const title = createMemo(() => {
        const obj = selectedObject();
        if (obj) {
            return obj.ty;
        } else {
            return 'Inspector';
        }
    })
    return (
        <div class="flex flex-col gap-4 p-4">
            <h1>{title()}</h1>
            <Show when={selectedObject()}>
                {(obj) => (
                    <>
                        <Name uid={obj().uid} name={obj().name} />
                        <Position uid={obj().uid} position={obj().position} />
                    </>
                )}
            </Show>
            <pre>{JSON.stringify(selectedObject(), null, 2)}</pre>
        </div>
    );
}
