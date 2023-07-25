// @ts-ignore
// import {
//   ContinuousEventPriority,
//   DiscreteEventPriority,
//   DefaultEventPriority,
// } from "react-reconciler/constants";
import type { StoreApi as UseStore } from "zustand/vanilla";
import type { Instance } from "./renderer";
import type { RootState } from "./store";

export interface Intersection extends THREE.Intersection {
  eventObject: THREE.Object3D;
}

export interface IntersectionEvent<TSourceEvent> extends Intersection {
  intersections: Intersection[];
  stopped: boolean;
  unprojectedPoint: THREE.Vector3;
  ray: THREE.Ray;
  camera: Camera;
  stopPropagation: () => void;
  /**
   * @deprecated in favour of nativeEvent. Please use that instead.
   */
  sourceEvent: TSourceEvent;
  nativeEvent: TSourceEvent;
  delta: number;
  spaceX: number;
  spaceY: number;
}

export type Camera = THREE.OrthographicCamera | THREE.PerspectiveCamera;
export type ThreeEvent<TEvent> = IntersectionEvent<TEvent>;
export type DomEvent = PointerEvent | MouseEvent | WheelEvent;

export type Events = {
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
};

export type EventHandlers = {
  onClick?: (event: ThreeEvent<MouseEvent>) => void;
  onContextMenu?: (event: ThreeEvent<MouseEvent>) => void;
  onDoubleClick?: (event: ThreeEvent<MouseEvent>) => void;
  onPointerUp?: (event: ThreeEvent<PointerEvent>) => void;
  onPointerDown?: (event: ThreeEvent<PointerEvent>) => void;
  onPointerOver?: (event: ThreeEvent<PointerEvent>) => void;
  onPointerOut?: (event: ThreeEvent<PointerEvent>) => void;
  onPointerEnter?: (event: ThreeEvent<PointerEvent>) => void;
  onPointerLeave?: (event: ThreeEvent<PointerEvent>) => void;
  onPointerMove?: (event: ThreeEvent<PointerEvent>) => void;
  onPointerMissed?: (event: MouseEvent) => void;
  onPointerCancel?: (event: ThreeEvent<PointerEvent>) => void;
  onWheel?: (event: ThreeEvent<WheelEvent>) => void;
};

export interface EventManager<TTarget> {
  connected: TTarget | boolean;
  handlers?: Events;
  connect?: (target: TTarget) => void;
  disconnect?: () => void;
}

export interface PointerCaptureTarget {
  intersection: Intersection;
  target: Element;
}

function makeId(event: Intersection) {
  return (
    (event.eventObject || event.object).uuid +
    "/" +
    event.index +
    event.instanceId
  );
}

// https://github.com/facebook/react/tree/main/packages/react-reconciler#getcurrenteventpriority
// Gives React a clue as to how import the current interaction is
// export function getEventPriority() {
//   let name = window?.event?.type;
//   switch (name) {
//     case "click":
//     case "contextmenu":
//     case "dblclick":
//     case "pointercancel":
//     case "pointerdown":
//     case "pointerup":
//       return DiscreteEventPriority;
//     case "pointermove":
//     case "pointerout":
//     case "pointerover":
//     case "pointerenter":
//     case "pointerleave":
//     case "wheel":
//       return ContinuousEventPriority;
//     default:
//       return DefaultEventPriority;
//   }
// }

/**
 * Release pointer captures.
 * This is called by releasePointerCapture in the API, and when an object is removed.
 */
function releaseInternalPointerCapture(
  capturedMap: Map<number, Map<THREE.Object3D, PointerCaptureTarget>>,
  obj: THREE.Object3D,
  captures: Map<THREE.Object3D, PointerCaptureTarget>,
  pointerId: number
): void {
  const captureData: PointerCaptureTarget | undefined = captures.get(obj);
  if (captureData) {
    captures.delete(obj);
    // If this was the last capturing object for this pointer
    if (captures.size === 0) {
      capturedMap.delete(pointerId);
      captureData.target.releasePointerCapture(pointerId);
    }
  }
}

export function removeInteractivity(
  store: UseStore<RootState>,
  object: THREE.Object3D
) {
  const { internal } = store.getState();
  // Removes every trace of an object from the data store
  internal.interaction = internal.interaction.filter((o) => o !== object);
  internal.initialHits = internal.initialHits.filter((o) => o !== object);
  internal.hovered.forEach((value, key) => {
    if (value.eventObject === object || value.object === object) {
      internal.hovered.delete(key);
    }
  });
  internal.capturedMap.forEach((captures, pointerId) => {
    releaseInternalPointerCapture(
      internal.capturedMap,
      object,
      captures,
      pointerId
    );
  });
}

export function createEvents(store: UseStore<RootState>) {
  /** Calculates delta */
  function calculateDistance(event: DomEvent) {
    const { internal } = store.getState();
    const dx = event.offsetX - internal.initialClick[0];
    const dy = event.offsetY - internal.initialClick[1];
    return Math.round(Math.sqrt(dx * dx + dy * dy));
  }

  /** Returns true if an instance has a valid pointer-event registered, this excludes scroll, clicks etc */
  function filterPointerEvents(objects: THREE.Object3D[]) {
    return objects.filter((obj) =>
      ["Move", "Over", "Enter", "Out", "Leave"].some(
        (name) =>
          (obj as unknown as Instance).__r3f?.handlers[
            ("onPointer" + name) as keyof EventHandlers
          ]
      )
    );
  }


  const handlePointer = (name: string) => {
    // Deal with cancelation
    switch (name) {
      case "onPointerLeave":
      case "onPointerCancel":
        return () => cancelPointer([]);
      case "onLostPointerCapture":
        return (event: DomEvent) => {
          const { internal } = store.getState();
          if (
            "pointerId" in event &&
            !internal.capturedMap.has(event.pointerId)
          ) {
            // If the object event interface had onLostPointerCapture, we'd call it here on every
            // object that's getting removed.
            internal.capturedMap.delete(event.pointerId);
            cancelPointer([]);
          }
        };
    }

    // Any other pointer goes here ...
    return (event: DomEvent) => {
      const { onPointerMissed, internal } = store.getState();

      prepareRay(event);
      internal.lastEvent.current = event;

      // Get fresh intersects
      const isPointerMove = name === "onPointerMove";
      const isClickEvent =
        name === "onClick" ||
        name === "onContextMenu" ||
        name === "onDoubleClick";
      const filter = isPointerMove ? filterPointerEvents : undefined;
      const hits = patchIntersects(intersect(filter), event);
      const delta = isClickEvent ? calculateDistance(event) : 0;

      // Save initial coordinates on pointer-down
      if (name === "onPointerDown") {
        internal.initialClick = [event.offsetX, event.offsetY];
        internal.initialHits = hits.map((hit) => hit.eventObject);
      }

      // If a click yields no results, pass it back to the user as a miss
      // Missed events have to come first in order to establish user-land side-effect clean up
      if (isClickEvent && !hits.length) {
        if (delta <= 2) {
          pointerMissed(event, internal.interaction);
          if (onPointerMissed) onPointerMissed(event);
        }
      }
      // Take care of unhover
      if (isPointerMove) cancelPointer(hits);

      handleIntersects(hits, event, delta, (data: ThreeEvent<DomEvent>) => {
        const eventObject = data.eventObject;
        const instance = (eventObject as unknown as Instance).__r3f;
        const handlers = instance?.handlers;
        // Check presence of handlers
        if (!instance?.eventCount) return;

        if (isPointerMove) {
          // Move event ...
          if (
            handlers.onPointerOver ||
            handlers.onPointerEnter ||
            handlers.onPointerOut ||
            handlers.onPointerLeave
          ) {
            // When enter or out is present take care of hover-state
            const id = makeId(data);
            const hoveredItem = internal.hovered.get(id);
            if (!hoveredItem) {
              // If the object wasn't previously hovered, book it and call its handler
              internal.hovered.set(id, data);
              handlers.onPointerOver?.(data as ThreeEvent<PointerEvent>);
              handlers.onPointerEnter?.(data as ThreeEvent<PointerEvent>);
            } else if (hoveredItem.stopped) {
              // If the object was previously hovered and stopped, we shouldn't allow other items to proceed
              data.stopPropagation();
            }
          }
          // Call mouse move
          handlers.onPointerMove?.(data as ThreeEvent<PointerEvent>);
        } else {
          // All other events ...
          const handler = handlers[name as keyof EventHandlers] as (
            event: ThreeEvent<PointerEvent>
          ) => void;
          if (handler) {
            // Forward all events back to their respective handlers with the exception of click events,
            // which must use the initial target
            if (!isClickEvent || internal.initialHits.includes(eventObject)) {
              // Missed events have to come first
              pointerMissed(
                event,
                internal.interaction.filter(
                  (object) => !internal.initialHits.includes(object)
                )
              );
              // Now call the handler
              handler(data as ThreeEvent<PointerEvent>);
            }
          } else {
            // Trigger onPointerMissed on all elements that have pointer over/out handlers, but not click and weren't hit
            if (isClickEvent && internal.initialHits.includes(eventObject)) {
              pointerMissed(
                event,
                internal.interaction.filter(
                  (object) => !internal.initialHits.includes(object)
                )
              );
            }
          }
        }
      });
    };
  };

  function pointerMissed(event: MouseEvent, objects: THREE.Object3D[]) {
    objects.forEach((object: THREE.Object3D) =>
      (object as unknown as Instance).__r3f?.handlers.onPointerMissed?.(event)
    );
  }

  return { handlePointer };
}
