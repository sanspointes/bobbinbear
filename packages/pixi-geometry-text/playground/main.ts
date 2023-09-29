import { Application } from '@pixi/app';
import { FontHandle, GeometryText, initTesselator } from '../src';
import { Viewport } from 'pixi-viewport';
import { EventSystem } from '@pixi/events';

import TestFont from './Roboto-Regular.ttf';

const app = new Application({ background: '#1099bb', resizeTo: window });
globalThis.__PIXI_APP__ = app;

document.body.appendChild(app.view as unknown as HTMLCanvasElement);

async function main() {
    await initTesselator();

    const fontResult = await FontHandle.fromUrl(TestFont);
    if (!fontResult.ok) throw fontResult.err;

    const font = fontResult.value;
    console.log(font);
    const cgid = font.font.gid_by_code_point('c');
    const v = font.font.names();
    console.log(cgid, v);
    const shape = font.getStringShape('hello man');
    console.log(shape);
    const a = font.getCharGeometry('p');
    console.log({ a });

    const events = new EventSystem(app.renderer);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    events.domElement = app.renderer.view as any;

    const viewport = new Viewport({ events: app.renderer.events });
    viewport.drag().decelerate().wheel();
    app.stage.addChild(viewport);

    const text = new GeometryText(font);
    text.position.set(100, 100);
    // text.scale.set(0.05, 0.05);
    text.interactive = true;
    let value = `Test string`;
    text.value = value;

    text.interactive = true;
    text.onpointermove = (e) => {
        const localPos = e.getLocalPosition(text);
        console.log(text.hitTestCharIndex(localPos.x, localPos.y));
    };

    window.addEventListener('keydown', (e) => {
        console.log(e.key);
        if (e.key.length === 1) {
            value += e.key;
            text.value = value;
        } else if (e.key === 'Backspace') {
            value = value.slice(0, value.length - 1);
            text.value = value;
        }
    });

    viewport.addChild(text);
}
main();

// Assets.load('https://pixijs.com/assets/bitmap-font/desyrel.xml').then(() => {
//     const text = new SDFText();
//     text.text = 'My text please';
//
//     text.x = 50;
//     text.y = 200;
//
//     text.sync(() => {
//         app.stage.addChild(text);
//     });
// });
