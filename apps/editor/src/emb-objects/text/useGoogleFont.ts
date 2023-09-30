import { MaybeAccessor, access } from '@solid-primitives/utils';
import { GoogleFontVariant } from './shared';
import { createQuery } from '@tanstack/solid-query';
import { arrayFirst } from '@/utils/array';

type UseGoogleFontResult = {
    kind: 'webfonts#webfontList';
    items: [
        {
            family: string;
            variants: GoogleFontVariant[];
            // subsets: [
            //     'cyrillic',
            //     'cyrillic-ext',
            //     'greek',
            //     'greek-ext',
            //     'latin',
            //     'latin-ext',
            //     'vietnamese',
            // ];
            version: string;
            lastModified: string;
            files: Record<GoogleFontVariant, string | undefined>;
            category: 'sans-serif' | 'serif' | 'monospace';
            kind: 'webfonts#webfont';
            // menu: 'http://fonts.gstatic.com/s/roboto/v30/KFOmCnqEu92Fr1Mu5GxP.ttf';
        },
    ];
};

export function useGoogleFont(fontFamily: MaybeAccessor<string>) {
    const result = createQuery(
        () => [access(fontFamily)],
        () =>
            fetch(
                `https://www.googleapis.com/webfonts/v1/webfonts?family=${access(
                    fontFamily,
                )}&key=${import.meta.env['VITE_GOOGLE_FONT_API_KEY']}`,
            )
                .then((r) => r.json() as Promise<UseGoogleFontResult>)
                .then((v) => arrayFirst(v.items)),
    );
    return result;
}
