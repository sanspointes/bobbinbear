export * from './FontParser';
export * from './GraphicsText';
import init from 'tesselator';

export async function initTesselator() {
    await init();
}
