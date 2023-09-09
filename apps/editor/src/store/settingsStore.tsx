import { generateStore } from '.';

export enum ColorInputStrategy {
    Hex = '#',
    Rgb = 'rgb',
    Hsv = 'hsv',
}
export const AllColorInputStrategies = [
    ColorInputStrategy.Hex,
    ColorInputStrategy.Hsv,
    ColorInputStrategy.Rgb,
];

export type SettingsMessage = {
    'settings:set-color-input-strategy': ColorInputStrategy;
};
export type SettingsModel = {
    colorInputStrategy: ColorInputStrategy;
};

export function createSettingsStore() {
    const model: SettingsModel = {
        colorInputStrategy: ColorInputStrategy.Hex,
    };

    const result = generateStore<SettingsModel, SettingsMessage>(model, {
        'settings:set-color-input-strategy': (_store, set, strategy) => {
            console.debug(
                `SettingsModel: Setting ColorInputStrategy to ${strategy}`,
            );
            set('colorInputStrategy', strategy);
        },
    });

    return result;
}
