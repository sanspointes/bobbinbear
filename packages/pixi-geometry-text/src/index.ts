export * from './FontParser';
export * from './GraphicsText';
import init from '@bearbroidery/bobbin-wasm-utils';

export async function initTesselator() {
    await init();
}
