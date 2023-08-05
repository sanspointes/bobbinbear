import { ExtraPropHandler } from "@bearbroidery/constructables/dist/elements";
import { Container } from "@pixi/display";
import { Point } from "@pixi/core";
import { SolixiState } from "./state";

const PositionHandler: ExtraPropHandler<SolixiState, typeof Container, Point | [number, number]> = (_1, _2, object: Container, value) => {
  if (Array.isArray(value)) {
    object.position.set(value[0], value[1]);
  } else {
    object.position.copyFrom(value)
  }
}
const PositionXHandler: ExtraPropHandler<SolixiState, typeof Container, number> = (_1, _2, object, value) => {
  object.position.x = value;
}
const PositionYHandler: ExtraPropHandler<SolixiState, typeof Container, number> = (_1, _2, object, value) => {
  object.position.y = value;
}

export const HasPositionFragment = {
  ['position']: PositionHandler,
  ['position-x']: PositionXHandler,
  ['position-y']: PositionYHandler,
}

const ScaleHandler: ExtraPropHandler<SolixiState, typeof Container, Point | [number, number]> = (_1, _2, object, value) => {
  if (Array.isArray(value)) {
    object.scale.set(value[0], value[1]);
  } else {
    object.scale.copyFrom(value)
  }
}
const ScaleXHandler: ExtraPropHandler<SolixiState, typeof Container, number> = (_1, _2, object, value) => {
  object.scale.x = value;
}
const ScaleYHandler: ExtraPropHandler<SolixiState, typeof Container, number> = (_1, _2, object, value) => {
  object.scale.y = value;
}

export const HasScaleFragment = {
  ['scale']: ScaleHandler,
  ['scale-x']: ScaleXHandler,
  ['scale-y']: ScaleYHandler,
}

const RotationHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object, value: number) => {
  object.rotation = value;
};
export const HasRotationFragment = {
  ['rotation']: RotationHandler,
}

const VisibilityHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object, value: boolean) => {
  object.visible = value;
};
export const HasVisibilityFragment = {
  ['visible']: VisibilityHandler,
}

// const OnWheelHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object, value: (event: FederatedWheelEvent) => void) => {
//   object.on('wheel', value)
//   return () => object.off('wheel', value);
// };
// const OnPointerOverHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//   object.on('pointerover', value)
//   return () => object.off('pointerover', value);
// }
// const OnPointerOutHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//   object.on('pointerout', value)
//   return () => object.off('pointerout', value);
// }
// const OnPointerEnterHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//   object.on('pointerenter', value)
//   return () => object.off('pointerenter', value);
// }
// const OnPointerLeaveHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//   object.on('pointerleave', value)
//   return () => object.off('pointerleave', value);
// }
// const OnRightClickHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//   object.on('rightclick', value)
//   return () => object.off('rightclick', value);
// }
// const OnRightDownHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//   object.on('rightdown', value)
//   return () => object.off('rightdown', value);
// }
// const OnRightUpHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//   object.on('rightup', value)
//   return () => object.off('rightup', value);
// }
// const OnClickHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//   object.on('click', value)
//   return () => object.off('click', value);
// }
// const OnPointerDownHandler: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//   object.on('pointerdown', value)
//   return () => object.off('pointerdown', value);
// }
// const OnPointerU: ExtraPropHandler<SolixiState, typeof Container> = (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//   object.on('pointerdown', value)
//   return () => object.off('pointerdown', value);
// }
//
// export const HasInteractivityFragment = {
//   onWheel: OnWheelHandler,
//   onPointerOver: OnPointerOverHandler,
//   onPointerOut: OnPointerOutHandler,
//
//   onPointerEnter: OnPointerEnterHandler,
//   onPointerLeave: OnPointerLeaveHandler,
//
//   onRightClick: OnRightClickHandler,
//   onRightDown: OnRightDownHandler,
//   onRightUp: OnRightUpHandler,
//
//   onClick: OnClickHandler,
//   onMouseDown: (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//     object.on('mousedown', value)
//     return () => object.off('mousedown', value);
//   },
//   onMouseUp: (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//     object.on('mouseup', value)
//     return () => object.off('mouseup', value);
//   },
//   onMouseMove: (_1, _2, object: Container, value: (event: FederatedPointerEvent) => void) => {
//     object.on('mousemove', value)
//     return () => object.off('mousemove', value);
//   },
// }
