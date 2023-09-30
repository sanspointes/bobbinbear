export const blockEvent = (e: Event) => {
    e.stopPropagation();
};

export type SelectionChangeEvent = {
    selectionStart: number | null;
    selectionEnd: number | null;
    selectionDirection: 'forward' | 'backward' | 'none' | null;
};
