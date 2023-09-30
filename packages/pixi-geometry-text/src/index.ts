export * from './FontParser';
export * from './GeometryText';
export * from './input/GeometryTextInput';
import __wbg_init from '@bearbroidery/bobbin-wasm-utils';

export type InitInput =
    | RequestInfo
    | URL
    | Response
    | BufferSource
    | WebAssembly.Module;
export default function init(module: InitInput | Promise<InitInput>) {
    __wbg_init(module);
    console.log('Attempting to init wasm');
}
