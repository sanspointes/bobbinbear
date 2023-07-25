import { ComponentProps, JSX, createComputed, mergeProps, onMount, splitProps } from 'solid-js'
import * as display from '@pixi/display';
import * as mesh from '@pixi/mesh';
import * as core from '@pixi/core';

import { createRoot, extend } from '../core/index'
// import { createPointerEvents } from './events'

import type { DomEvent } from '../core/events'
import type { RenderProps } from '../core/index'

export interface CanvasProps extends Omit<RenderProps, 'size'>, ComponentProps<'div'> {
  children: JSX.Element
  /** Canvas fallback content, similar to img's alt prop */
  fallback?: JSX.Element
  /**
   * Options to pass to useMeasure.
   * @see https://github.com/pmndrs/react-use-measure#api
   */
  resize?: any //ResizeOptions
  /** The target where events are being subscribed to, default: the div that wraps canvas */
  eventSource?: HTMLElement
  /** The event prefix that is cast into canvas pointer x/y events, default: "offset" */
  eventPrefix?: 'offset' | 'client' | 'page' | 'layer' | 'screen'

  style?: JSX.CSSProperties
}

export interface Props extends CanvasProps {}
/**
 * A DOM canvas which accepts threejs elements as children.
 * @see https://docs.pmnd.rs/react-three-fiber/api/canvas
 */
export function Canvas(props: Props) {
  const [_, rest] = splitProps(props, [
    'children',
    'fallback',
    'resize',
    'style',
    'gl',
    'events',
    'eventSource',
    'eventPrefix',
    'frameloop',
    'dpr',
    'performance',
    'raycaster',
    'onPointerMissed',
    'onCreated',
    'scene',
  ])

  // Create a known catalogue of Threejs-native elements
  // This will include the entire THREE namespace by default, users can extend
  // their own elements by using the createRoot API instead
  createComputed(() => extend({
    Container: display.Container,
    Mesh: mesh.Mesh,
    MeshMaterial: mesh.MeshMaterial,
    MeshGeometry: mesh.MeshGeometry,
    Shader: core.Shader,
  }), [])

  const [other, threeProps] = splitProps(
    mergeProps(
      props,
    ),
    ['children'],
  )

  // const [containerRef, containerRect] = useMeasure({ scroll: true, debounce: { scroll: 50, resize: 0 }, ...resize })
  let containerRef: HTMLDivElement = null!,
    canvasRef: HTMLCanvasElement = null!,
    divRef: HTMLDivElement = null!

  onMount(() => {
    let size = canvasRef.parentElement?.getBoundingClientRect() ?? {
      width: 0,
      height: 0,
      top: 0,
      left: 0,
    }

    let root = createRoot<HTMLCanvasElement>(canvasRef)
    root.configure({
      ...threeProps,
      size,
      // Pass mutable reference to onPointerMissed so it's free to update
      onPointerMissed: (...args) => props.onPointerMissed?.(...args),
      onCreated: (state) => {
        // Call onCreated callback
        props.onCreated?.(state)
      },
    })

    root.render(props)
  })

  // When the event source is not this div, we need to set pointer-events to none
  // Or else the canvas will block events from reaching the event source
  const pointerEvents = props.eventSource ? 'none' : 'auto'

  return (
    <div
      ref={divRef}
      style={{
        position: 'relative',
        width: '100%',
        height: '100%',
        overflow: 'hidden',
        'pointer-events': pointerEvents,
        ...props.style,
      }}
      {...rest}>
      <div ref={containerRef} style={{ width: '100%', height: '100%' }}>
        <canvas ref={canvasRef} style={{ display: 'block' }}>
          {props.fallback}
        </canvas>
      </div>
    </div>
  )
}
