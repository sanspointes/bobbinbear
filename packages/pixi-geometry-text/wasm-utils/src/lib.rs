mod tesselator;
mod utils;
mod font;
mod vector;

use std::convert::TryInto;

use js_sys::{Array, Float32Array, Uint16Array};
use lyon::lyon_tessellation::FillOptions;
use tesselator::tesselate_font_path;
use wasm_bindgen::prelude::*;

fn js_value_to_char(js_value: JsValue) -> Option<char> {
    if let Some(js_string) = js_value.as_string() {
        // Convert the JavaScript string to a Rust string.
        let rust_string = js_string;

        // Ensure the Rust string consists of exactly one Unicode scalar value.
        if rust_string.chars().count() == 1 {
            return rust_string.chars().next();
        }
    }
    None
}

#[allow(dead_code)]
#[wasm_bindgen]
pub struct TesselationResult {
    _vertices: Float32Array,
    _indices: Uint16Array,
}

#[wasm_bindgen]
impl TesselationResult {
    #[wasm_bindgen(getter)]
    pub fn vertices(&self) -> Float32Array {
        return self._vertices.clone();
    }
    #[wasm_bindgen(getter)]
    pub fn indices(&self) -> Uint16Array {
        return self._indices.clone();
    }
}

#[wasm_bindgen]
pub enum FillRule {
    EvenOdd = 0,
    NonZero = 1,
}

impl Into<lyon::path::FillRule> for FillRule {
    fn into(self) -> lyon::path::FillRule {
        match self {
            FillRule::EvenOdd => lyon::path::FillRule::EvenOdd,
            FillRule::NonZero => lyon::path::FillRule::NonZero,
        }
    }
}

#[wasm_bindgen]
pub fn tesselate_font(
    cmds: &Array,
    crds: &Array,
    tolerance: f32,
    fill_rule: FillRule,
) -> TesselationResult {
    console_error_panic_hook::set_once();
    let mut cmds_vec: Vec<char> = Vec::with_capacity(cmds.length().try_into().unwrap());
    for cmd_val in cmds.iter() {
        if let Some(char) = js_value_to_char(cmd_val) {
            cmds_vec.push(char);
        }
    }

    let mut crds_vec: Vec<f32> = Vec::with_capacity(crds.length().try_into().unwrap());
    for crd_val in crds.iter() {
        if let Some(number) = crd_val.as_f64() {
            crds_vec.push(number as f32);
        }
    }

    let options = FillOptions::tolerance(tolerance).with_fill_rule(fill_rule.into());

    let geometry = tesselate_font_path(&cmds_vec, &crds_vec, &options);

    let vertices = {
        let verts_f32: Vec<f32> = geometry
            .vertices
            .iter()
            .flat_map(|p| vec![p.x, p.y])
            .collect();
        let boxed_slice = verts_f32.into_boxed_slice();
        let ptr = boxed_slice.as_ptr();
        let length = boxed_slice.len();
        std::mem::forget(boxed_slice); // Do not deallocate the memory
        unsafe { js_sys::Float32Array::view(&std::slice::from_raw_parts(ptr, length)) }
    };

    let indices = {
        let boxed_slice = geometry.indices.into_boxed_slice();
        let ptr = boxed_slice.as_ptr();
        let length = boxed_slice.len();
        std::mem::forget(boxed_slice); // Do not deallocate the memory
        unsafe { js_sys::Uint16Array::view(&std::slice::from_raw_parts(ptr, length)) }
    };

    TesselationResult {
        _vertices: vertices,
        _indices: indices,
    }
}
