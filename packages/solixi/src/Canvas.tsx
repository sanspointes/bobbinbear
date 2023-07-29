import { Application, Container, DisplayObject, IApplicationOptions } from "pixi.js"
import { ComponentProps, onMount, splitProps, JSX, createSignal } from "solid-js"
import { Solixi } from "."
import { SolixiState } from "./state"
import { SolixiRoot } from "@bearbroidery/constructables/src/renderer"
import { createRAF, targetFPS } from '@solid-primitives/raf';

type InternalCanvasProps = {
  app?: Omit<Partial<IApplicationOptions>, 'canvas'>,
  resolution?: number,
  children: JSX.Element | null,
  onCreated?: (state: SolixiState) => void,
  devtools?: boolean,
  frameloop?: 'ondemand'|'always',
}
type CanvasProps = ComponentProps<'div'> & InternalCanvasProps;

const INTERNAL_PROP_KEYS = ['app', 'resolution', 'devtools', 'frameloop'] as unknown as (keyof InternalCanvasProps[]);

export const Canvas = (props: CanvasProps) => {
  const [internalProps, divElementProps] = splitProps(props, INTERNAL_PROP_KEYS);
  const [childrenProps, _] = splitProps(props, ['children']);

  let wrapperEl: HTMLDivElement|undefined;
  let containerEl: HTMLDivElement|undefined;
  let canvasEl: HTMLCanvasElement|undefined;

  let solixiRoot: SolixiRoot<SolixiState, Container<DisplayObject>>|undefined = undefined;

  let canRender = true;
  const invalidate = () => {
    canRender = true;
  }
  const [running, start, stop] = createRAF((time) => {
    if (solixiRoot && canRender) {
      solixiRoot.state.app.render();
      if (!internalProps.frameloop || internalProps.frameloop === 'always') {
        invalidate();
      }
    }
  })


  start();

  onMount(() => {
    const defaultAppOptions: Partial<IApplicationOptions> = {
      view: canvasEl,
      resolution: internalProps.resolution,
    }
    const appOptions = {...(internalProps.app ?? {}), ...defaultAppOptions };
    const app = new Application(appOptions)

    // @ts-expect-error ; Pixi.js devtools
    if (internalProps.devtools) globalThis.__PIXI_APP__ = app;

    const state: SolixiState = {
      app, 
      stage: app.stage,
      ticker: app.ticker,
      invalidate,
    }

    const root = Solixi.createRoot<typeof app.stage>(app.stage, state);

    root.render(childrenProps);

    if (internalProps.onCreated) internalProps.onCreated(root.state);
    solixiRoot = root;
  })

  return (
    <div
      ref={wrapperEl}
      style={{
        position: 'relative',
        width: '100%',
        height: '100%',
        overflow: 'hidden',
      }}
      {...divElementProps}>
      <div ref={containerEl} style={{ width: '100%', height: '100%' }}>
        <canvas ref={canvasEl} style={{ display: 'block' }} />
      </div>
    </div>
  )
}
