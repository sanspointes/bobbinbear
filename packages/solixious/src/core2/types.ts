import * as core from '@pixi/core'
import * as app from '@pixi/app';
import * as interaction from '@pixi/interaction';
import * as display from '@pixi/display';
import * as ticker from '@pixi/ticker';

/*
 * UTILITY TYPES
 */
export type NonFunctionKeys<T> = { [K in keyof T]: T[K] extends Function ? never : K }[keyof T]
export type Overwrite<T, O> = Omit<T, NonFunctionKeys<O>> & O
export type Constructor = new (...args: any[]) => any
export type Args<T> = T extends Constructor ? ConstructorParameters<T> : any[]

/*
 * GENERAL TYPES
 */
export interface EventHandlers {
  onClick?: (event: interaction.InteractionEvent) => void
  onContextMenu?: (event: interaction.InteractionEvent) => void
  onDoubleClick?: (event: interaction.InteractionEvent) => void
  onPointerUp?: (event: interaction.InteractionEvent) => void
  onPointerDown?: (event: interaction.InteractionEvent) => void
  onPointerOver?: (event: interaction.InteractionEvent) => void
  onPointerOut?: (event: interaction.InteractionEvent) => void
  onPointerEnter?: (event: interaction.InteractionEvent) => void
  onPointerLeave?: (event: interaction.InteractionEvent) => void
  onPointerMove?: (event: interaction.InteractionEvent) => void
  onPointerMissed?: (event: interaction.InteractionEvent) => void
  onPointerCancel?: (event: interaction.InteractionEvent) => void
  onWheel?: (event: interaction.InteractionEvent) => void
}

/*
 * MATH TYPES
 */
export interface MathRepresentation {
  set(...args: number[]): any
}
export type Matrix = core.Matrix | Parameters<core.Matrix['set']> | Readonly<core.Matrix['set']>
export type MathType<T extends MathRepresentation | core.Color> = T extends core.Color
  ? ConstructorParameters<typeof core.Color> | core.Color | core.ColorSource
  : T extends MathRepresentation
  ? T | Parameters<T['set']> | number
  : T;

export type Point = ConstructorParameters<typeof core.Point> | core.Point;
export type Color = ConstructorParameters<typeof core.Color> | core.Color | core.ColorSource;

/*
 * PROP TYPES
 */
type WithMathProps<P> = { [K in keyof P]: P[K] extends MathRepresentation ? MathType<P[K]> : P[K] }

// Attach
export type AttachFnType<O = any> = (parent: any, self: O) => () => void
export type AttachType<O = any> = string | AttachFnType<O>

export interface NodeProps<T> {
  attach?: AttachType
  /** Constructor arguments */
  args?: Args<T>
  children?: JSX.Element
  ref?: ((value: T) => void) | T
  key?: string
  onUpdate?: (self: T) => void
}

type EventProps = Partial<EventHandlers>

export type SxiElementProps<T extends Constructor, P = InstanceType<T>> = Partial<
  Overwrite<WithMathProps<P>, NodeProps<P> & EventProps>
>

/*
 * STATE TYPES
 */


/**
 * Root state of the Pixi app
 */
export type SxiState = {
  // Pixi objects
  app: app.Application;
  stage: display.Container;
  ticker: ticker.Ticker;

  /** Whether or not this SxiState content is mounted to the page */
  active: boolean;
}


// INSTANCE TYPES
export type SxiInstance<T = unknown> = {
  solixi: SxiState;
  type: string;
  parent?: SxiInstance;
}
