export * from './FontParser';
export * from './GeometryText';
export * from './input/GeometryTextInput';
import init from '@bearbroidery/bobbin-wasm-utils';

export async function initTesselator() {
    await init();
}
