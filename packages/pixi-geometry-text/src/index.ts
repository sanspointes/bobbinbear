export * from './FontParser';
export * from './GraphicsText';
import init from 'bobbin-wasm-utils';

export async function initTesselator() {
    await init();
}
