import { Show } from 'solid-js';
import { useBobbinBear } from '../../hooks/useBobbinBear';
import { Name } from './Name';
import { Position } from './Position';

export function Inspector() {
    const { document } = useBobbinBear();
    const { selectedObject } = document;
    return (
        <div class="p-4 flex flex-col gap-4">
            <Show when={selectedObject()}>
                {(obj) => (
                    <>
                        <Name uid={obj().uid} name={obj().name} />
                        <Position uid={obj().uid} position={obj().position} />
                    </>
                )}
            </Show>
        </div>
    );
}
