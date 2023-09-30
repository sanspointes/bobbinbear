import { IDestroyOptions } from '@pixi/display';
import { FontHandle, GeometryText } from '..';
import {
    // Required for input types
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    type EventSystem,
    FederatedEventHandler,
    FederatedPointerEvent,
} from '@pixi/events';
import { SelectionChangeEvent, VirtualInput } from '.';

type GeometryTextInputOptions = {
    ltr: boolean;
    multiline: boolean;
};

export class GeometryTextInput extends GeometryText {
    private input: VirtualInput;

    constructor(
        handle: FontHandle,
        private options: GeometryTextInputOptions,
    ) {
        super(handle, options.ltr);

        this.interactive = true;

        this.input = new VirtualInput({
            multiline: this.options.multiline,
            onChange: (value) => {
                super.value = value;
            },
            onFocus: () => {
                console.log('Input focus');
            },
            onBlur: () => {
                this.stopEditing();
                console.log('Input blur');
            },
            onSelectionChange: (event) => {
                this.handleSelectionChange(event);
            },
        });
    }

    get value(): string {
        return super.value;
    }

    set value(value: string) {
        super.value = value;
        this.input.value = value;
    }

    isEditing = false;
    // TODO: Break up to make data flow clearer
    private startEditing() {
        if (this.isEditing) return;

        this.isEditing = true;
        this.input.focus();
    }

    private stopEditing() {
        if (!this.isEditing) return;
        this.isEditing = false;
        this.input.blur();
    }

    onpointertap: FederatedEventHandler<FederatedPointerEvent> | null = (e) => {
        this.startEditing();

        const localPos = e.getLocalPosition(this);
        const v = this.hitTestCharIndex(localPos.x, localPos.y);
        this.input.focus();
        if (v !== undefined) {
            this.input.setSelectionRange(v);
            this.handleSelectionChange({
                selectionStart: v,
                selectionEnd: v,
                selectionDirection: null,
            });
        }
    };
    onclick: FederatedEventHandler<FederatedPointerEvent> | null = (e) => {
        this.startEditing();

        const localPos = e.getLocalPosition(this);
        const v = this.hitTestCharIndex(localPos.x, localPos.y);
        this.input.focus();
        if (v !== undefined) {
            this.input.setSelectionRange(v);
            this.handleSelectionChange({
                selectionStart: v,
                selectionEnd: v,
                selectionDirection: null,
            });
        }
    };

    private handleSelectionChange(ev: SelectionChangeEvent) {
        console.log(ev);
    }

    destroy(options?: boolean | IDestroyOptions | undefined): void {
        super.destroy();
        this.input.destroy();
    }
}
