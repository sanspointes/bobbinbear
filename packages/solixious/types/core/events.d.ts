import * as interaction from '@pixi/interaction';
import type { RootState } from './store';
import type { Properties } from './utils';
import { Container } from 'pixi.js';
export interface Intersection extends interaction.InteractionData {
    /** The event source (the object which registered the handler) */
    eventObject: Container;
}
export type ThreeEvent<TEvent> = interaction.InteractionEvent & Properties<TEvent>;
export type DomEvent = PointerEvent | MouseEvent | WheelEvent;
export interface Events {
    onClick: EventListener;
    onContextMenu: EventListener;
    onDoubleClick: EventListener;
    onWheel: EventListener;
    onPointerDown: EventListener;
    onPointerUp: EventListener;
    onPointerLeave: EventListener;
    onPointerMove: EventListener;
    onPointerCancel: EventListener;
    onLostPointerCapture: EventListener;
}
export interface EventHandlers {
    onClick?: (event: interaction.InteractionEvent) => void;
    onContextMenu?: (event: interaction.InteractionEvent) => void;
    onDoubleClick?: (event: interaction.InteractionEvent) => void;
    onPointerUp?: (event: interaction.InteractionEvent) => void;
    onPointerDown?: (event: interaction.InteractionEvent) => void;
    onPointerOver?: (event: interaction.InteractionEvent) => void;
    onPointerOut?: (event: interaction.InteractionEvent) => void;
    onPointerEnter?: (event: interaction.InteractionEvent) => void;
    onPointerLeave?: (event: interaction.InteractionEvent) => void;
    onPointerMove?: (event: interaction.InteractionEvent) => void;
    onPointerMissed?: (event: interaction.InteractionEvent) => void;
    onPointerCancel?: (event: interaction.InteractionEvent) => void;
    onWheel?: (event: interaction.InteractionEvent) => void;
}
export type FilterFunction = (items: THREE.Intersection[], state: RootState) => THREE.Intersection[];
export type ComputeFunction = (event: DomEvent, root: RootState, previous?: RootState) => void;
export interface EventManager<TTarget> {
    /** Determines if the event layer is active */
    enabled: boolean;
    /** Event layer priority, higher prioritized layers come first and may stop(-propagate) lower layer  */
    priority: number;
    /** The compute function needs to set up the raycaster and an xy- pointer  */
    compute?: ComputeFunction;
    /** The filter can re-order or re-structure the intersections  */
    filter?: FilterFunction;
    /** The target node the event layer is tied to */
    connected?: TTarget;
    /** All the pointer event handlers through which the host forwards native events */
    handlers?: Events;
    /** Allows re-connecting to another target */
    connect?: (target: TTarget) => void;
    /** Removes all existing events handlers from the target */
    disconnect?: () => void;
    /** Triggers a onPointerMove with the last known event. This can be useful to enable raycasting without
     *  explicit user interaction, for instance when the camera moves a hoverable object underneath the cursor.
     */
    update?: () => void;
}
export interface PointerCaptureTarget {
    intersection: Intersection;
    target: Element;
}
//# sourceMappingURL=events.d.ts.map