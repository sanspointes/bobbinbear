pub mod tesselator;

use js_sys::{Float32Array, Uint16Array};
use lyon::{lyon_tessellation::VertexBuffers, geom::euclid::{Point2D, UnknownUnit}};
use wasm_bindgen::prelude::*;

pub enum BBVectorCommand {
    M { x: f32, y: f32 },
    L { x: f32, y: f32 },
    Q { c0x: f32, c0y: f32, x: f32, y: f32},
    C { c0x: f32, c0y: f32, c1x: f32, c1y: f32, x: f32, y: f32},
    Z,
}

/// TODO: Combine internal/external as memory safety rules means we need to clone the array on
/// access anyway.

/// Represents a shape stored as a vector.
///
/// * `commands`: 
pub struct BBVector {
    pub commands: Vec<BBVectorCommand>,
}

/// Internal (rust) representation of a 2d geometry
///
/// * `positions`: 
/// * `indices`: 
pub struct BBGeometryInternal {
    positions: Vec<f32>,
    indices: Vec<u16>,
}

#[wasm_bindgen(inspectable)]
/// External (js) representation of a 2d geometry
///
/// * `positions`: 
/// * `indices`: 
pub struct BBGeometry {
    positions: Float32Array,
    indices: Uint16Array,
}

impl BBGeometryInternal {
    pub fn translate(&mut self, x: f32, y: f32) {
        for ( i, val ) in self.positions.iter_mut().enumerate() {
            if i % 2 == 0 {
                *val += x;
            } else {
                *val += y;
            }
        }
    }
}

#[wasm_bindgen]
impl BBGeometry {
    #[wasm_bindgen(getter = positions)]
    pub fn positions_as_float32array(&self) -> Float32Array {
        self.positions.clone()
    }
    #[wasm_bindgen(getter = indices)]
    pub fn indices_as_uint16array(&self) -> Uint16Array {
        self.indices.clone()
    }
}

impl Into<BBGeometry> for BBGeometryInternal {
    fn into(self) -> BBGeometry {
        let positions = {
            let boxed_slice = self.positions.into_boxed_slice();
            let ptr = boxed_slice.as_ptr();
            let length = boxed_slice.len();
            std::mem::forget(boxed_slice); // Do not deallocate the memory
            unsafe { js_sys::Float32Array::view(&std::slice::from_raw_parts(ptr, length)) }
        };
        let indices = {
            let boxed_slice = self.indices.into_boxed_slice();
            let ptr = boxed_slice.as_ptr();
            let length = boxed_slice.len();
            std::mem::forget(boxed_slice); // Do not deallocate the memory
            unsafe { js_sys::Uint16Array::view(&std::slice::from_raw_parts(ptr, length)) }
        };

        BBGeometry { positions, indices }
    }
}

impl From<VertexBuffers<Point2D<f32, UnknownUnit>, u16>> for BBGeometryInternal {
    fn from(value: VertexBuffers<Point2D<f32, UnknownUnit>, u16>) -> Self {

        let vertices = {
            let mut vertices: Vec<f32> = Vec::new();
            vertices.resize(value.vertices.len() * 2, 0.0);
            for (i, point) in value.vertices.iter().enumerate() {
                vertices[i * 2] = point.x;
                vertices[i * 2 + 1] = point.y;
            }
            vertices
        };

        Self {
            positions: vertices,
            indices: value.indices,
        }
    }
}
