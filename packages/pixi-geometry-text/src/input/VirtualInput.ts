import { SelectionChangeEvent, blockEvent } from './shared';

type VirtualInputOptions = {
    multiline: boolean;
    onChange: (value: string) => void;
    onFocus: () => void;
    onBlur: () => void;
    onSelectionChange: (ev: SelectionChangeEvent) => void;
};
/**
 * Invisible real input element to provide native controls for the pixi input element.
 */
export class VirtualInput {
    domInput: HTMLInputElement | HTMLTextAreaElement;
    constructor(private options: VirtualInputOptions) {
        const domTag = options.multiline ? 'textarea' : 'input';
        this.domInput = document.createElement(domTag);

        // Bind event listeners
        this.domInput.addEventListener('focus', () => this.options.onFocus());
        this.domInput.addEventListener('blur', () => this.options.onBlur());
        this.domInput.addEventListener('keydown', blockEvent);
        this.domInput.addEventListener('keyup', blockEvent);
        this.domInput.addEventListener('keypress', blockEvent);
        this.domInput.addEventListener('input', (e) => {
            e.stopPropagation();
            e.preventDefault();
            this.options.onChange(this.value);
        });
        this.domInput.addEventListener('selectionchange', () => {
            this.options.onSelectionChange({
                selectionStart: this.domInput.selectionStart,
                selectionEnd: this.domInput.selectionEnd,
                selectionDirection: this.domInput.selectionDirection,
            });
        });

        document.body.append(this.domInput);
    }

    get value(): string {
        return this.domInput.value;
    }

    set value(value: string) {
        this.domInput.value = value;
    }

    setSelectionRange(start: number, end = start) {
        this.domInput.selectionStart = start;
        this.domInput.selectionEnd = end;
    }

    focus() {
        this.domInput.focus();
        this.options.onFocus();
    }

    blur() {
        this.domInput.blur();
        this.options.onBlur();
    }

    destroy() {
        this.domInput.remove();
    }
}
