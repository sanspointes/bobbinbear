import { Uuid } from '@/utils/uuid';
import { EmbBase, EmbHasDimensions } from '../shared';

export type FontWeight = 100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900;
export type GoogleFontVariant =
    | '100'
    | '100italic'
    | '200'
    | '200italic'
    | '300'
    | '300italic'
    | 'regular'
    | 'italic'
    | '500'
    | '500italic'
    | '600'
    | '600italic'
    | '700'
    | '700italic'
    | '900'
    | '900italic';

export type FontFaceDescription = {
    fontFamily: string;
    weight: FontWeight;
    italic: boolean;
};

export function fontFaceDescToGFVariant(
    ffDescription: FontFaceDescription,
): GoogleFontVariant {
    if (ffDescription.weight === 400) {
        if (ffDescription.italic) return 'italic';
        else return 'regular';
    } else {
        return (ffDescription.weight +
            (ffDescription.italic ? 'italic' : '')) as GoogleFontVariant;
    }
}

export type EmbText = EmbBase &
    EmbHasDimensions & {
        id: Uuid<EmbText>;
        type: 'text';

        value: string;

        fontFace: FontFaceDescription;
    };
