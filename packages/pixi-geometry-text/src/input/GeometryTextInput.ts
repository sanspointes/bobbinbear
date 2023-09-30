import { Container, IDestroyOptions } from '@pixi/display';
import { FontHandle, GeometryText } from '..';
import {
    // Required for input types
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    type EventSystem,
    FederatedEventHandler,
    FederatedPointerEvent,
} from '@pixi/events';
import { SelectionChangeEvent, VirtualInput } from '.';
import { CaretView } from './Caret';
import { Point } from '@pixi/core';

type GeometryTextInputOptions = {
    ltr: boolean;
    multiline: boolean;
};

export class GeometryTextInput extends Container {
    private geometryText: GeometryText;
    private input: VirtualInput;
    private caret: CaretView;

    constructor(
        handle: FontHandle,
        private options: GeometryTextInputOptions,
    ) {
        super();
        this.geometryText = new GeometryText(handle, options.ltr);
        this.addChild(this.geometryText);
        this.geometryText.interactive = true;

        this.interactive = true;
        this.caret = new CaretView({
            alpha: 1,
            color: 0xffffff,
            rangeAlpha: 1,
            rangeColor: 0xffffff,
        });
        this.addChild(this.caret);

        this.input = new VirtualInput({
            multiline: this.options.multiline,
            onChange: (value) => {
                this.geometryText.value = value;
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
        return this.geometryText.value;
    }

    set value(value: string) {
        this.geometryText.value = value;
        this.input.value = value;
    }

    isEditing = false;
    // TODO: Break up to make data flow clearer
    private startEditing() {
        if (this.isEditing) return;

        this.isEditing = true;
        this.input.focus();
        this.caret.visible = true;
    }

    private stopEditing() {
        if (!this.isEditing) return;
        this.isEditing = false;
        this.input.blur();
        this.caret.visible = false;
    }

    onpointertap: FederatedEventHandler<FederatedPointerEvent> | null = (e) => {
        this.startEditing();

        const localPos = e.getLocalPosition(this);
        const v = this.geometryText.hitTestCharIndex(localPos.x, localPos.y);
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
        const v = this.geometryText.hitTestCharIndex(localPos.x, localPos.y);
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
        if (ev.selectionStart === null) {
            return;
        }
        const handle = this.geometryText.handle;

        const caretHeight = handle.ascender - handle.descender;

        const c = this.geometryText.getCharAtIndex(ev.selectionStart);
        const pos = new Point(c.position.x, c.position.y - 300);
        this.caret.updatePosition(pos, caretHeight);
    }

    destroy(options?: boolean | IDestroyOptions | undefined): void {
        this.geometryText.destroy(options);
        this.input.destroy();
    }
}
