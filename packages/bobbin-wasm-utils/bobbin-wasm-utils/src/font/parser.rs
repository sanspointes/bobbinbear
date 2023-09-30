use js_sys::{Array, Uint8Array, Float32Array, Uint16Array};
use lyon::lyon_tessellation::{FillOptions, Orientation};
use lyon::path::FillRule;
use owned_ttf_parser::{AsFaceRef, Face, GlyphId, OwnedFace, Rect};
use rustybuzz::{UnicodeBuffer, shape};
use wasm_bindgen::prelude::*;

use crate::font::vector_builder::BBFaceVectorBuilder;
use crate::vector::BBVector;
use crate::vector::{tesselator::tesselate_bb_vector_fill, BBGeometry};

#[wasm_bindgen]
pub struct BBFace {
    face: OwnedFace,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum BBOrientation {
    Vertical = 0,
    Horizontal = 1,
}

impl From<BBOrientation> for Orientation {
    fn from(value: BBOrientation) -> Self {
        match value {
            BBOrientation::Vertical => Orientation::Vertical,
            BBOrientation::Horizontal => Orientation::Horizontal,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum BBFillRule {
    EvenOdd = 0,
    NonZero = 1,
}

impl From<BBFillRule> for FillRule {
    fn from(value: BBFillRule) -> Self {
        match value {
            BBFillRule::EvenOdd => FillRule::EvenOdd,
            BBFillRule::NonZero => FillRule::NonZero,
        }
    }
}

#[wasm_bindgen]
pub struct BBFillOptions {
    tolerance: f32,
    fill_rule: BBFillRule,
    handle_intersections: bool,
    sweep_orientation: BBOrientation,
}
#[wasm_bindgen]
impl BBFillOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tolerance: 1.,
            fill_rule: BBFillRule::NonZero,
            handle_intersections: false,
            sweep_orientation: BBOrientation::Vertical,
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_tolerance(&mut self, tolerance: f32) {
        self.tolerance = tolerance;
    }
    #[wasm_bindgen(getter)]
    pub fn tolerance(&mut self) -> f32 {
        self.tolerance
    }
    #[wasm_bindgen(setter)]
    pub fn set_fill_rule(&mut self, fill_rule: BBFillRule) {
        self.fill_rule = fill_rule;
    }
    #[wasm_bindgen(getter)]
    pub fn fill_rule(&mut self) -> BBFillRule {
        self.fill_rule
    }
    #[wasm_bindgen(setter)]
    pub fn set_intersections(&mut self, intersections: bool) {
        self.handle_intersections = intersections;
    }
    #[wasm_bindgen(getter)]
    pub fn intersections(&mut self) -> bool {
        self.handle_intersections
    }
    #[wasm_bindgen(setter)]
    pub fn set_sweep_orientation(&mut self, sweep_orientation: BBOrientation) {
        self.sweep_orientation = sweep_orientation;
    }
    #[wasm_bindgen(getter)]
    pub fn sweep_orientation(&mut self) -> BBOrientation {
        self.sweep_orientation
    }
}

impl From<BBFillOptions> for FillOptions {
    fn from(value: BBFillOptions) -> Self {
        FillOptions::tolerance(value.tolerance)
            .with_fill_rule(value.fill_rule.into())
            .with_intersections(value.handle_intersections)
            .with_sweep_orientation(value.sweep_orientation.into())
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct BBRect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[wasm_bindgen]
impl BBRect {
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f32 {
        self.x
    }
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f32 {
        self.y
    }
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f32 {
        self.width
    }
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f32 {
        self.height
    }
}

impl From<Rect> for BBRect {
    fn from(value: Rect) -> Self {
        BBRect {
            x: value.x_min as f32,
            y: value.y_min as f32,
            width: value.x_max as f32 - value.x_min as f32,
            height: value.y_max as f32 - value.y_min as f32,
        }
    }
}

#[wasm_bindgen]
pub struct BBToGeometryReturn{ geometry: BBGeometry, bounds: Option<BBRect> }
#[wasm_bindgen]
impl BBToGeometryReturn {
    #[wasm_bindgen(getter)]
    pub fn vertices(&self) -> Float32Array {
        self.geometry.positions_as_float32array()
    }
    #[wasm_bindgen(getter)]
    pub fn indices(&self) -> Uint16Array {
        self.geometry.indices_as_uint16array()
    }
    #[wasm_bindgen(getter)]
    pub fn bounds(&self) -> Option<BBRect> {
        self.bounds.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct BBGlyphShape {
    gid: u32,
    cluster: u32,
    x_advance: i32,
    y_advance: i32,
    x_offset: i32,
    y_offset: i32,
}
#[wasm_bindgen]
impl BBGlyphShape {
    #[wasm_bindgen(getter)]
    pub fn gid(&self) -> u32 {
        self.gid
    }
    #[wasm_bindgen(getter)]
    pub fn cluster(&self) -> u32 {
        self.cluster
    }
    #[wasm_bindgen(getter)]
    pub fn x_advance(&self) -> i32 {
        self.x_advance
    }
    #[wasm_bindgen(getter)]
    pub fn y_advance(&self) -> i32 {
        self.y_advance
    }
    #[wasm_bindgen(getter)]
    pub fn x_offset(&self) -> i32 {
        self.x_offset
    }
    #[wasm_bindgen(getter)]
    pub fn y_offset(&self) -> i32 {
        self.y_offset
    }
}


#[wasm_bindgen]
pub struct BBLineShape {
    glyphs: Vec<BBGlyphShape>
}

#[wasm_bindgen] 
impl BBLineShape {
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        return self.glyphs.len();
    }
    #[wasm_bindgen]
    pub fn glyph_at(&self, index: usize) -> Option<BBGlyphShape> {
        return self.glyphs.get(index).cloned();
    }
}


#[wasm_bindgen]
impl BBFace {
    #[wasm_bindgen]
    pub fn from_buffer(font_data: Uint8Array) -> BBFace {
        // Create a Vec<u8> with the same length as Uint8Array.
        let length = font_data.length() as usize;
        let mut rs_font_data = vec![0; length];
        // Copy the data to the Vec<u8>.
        font_data.copy_to(&mut rs_font_data);
        let face = OwnedFace::from_vec(rs_font_data, 0).unwrap();

        BBFace { face }
    }

    #[wasm_bindgen]
    pub fn names(&self) -> Array {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        let names = face_ref.names();
        let result = Array::new_with_length(names.len().into());
        for name in face_ref.names() {
            if let Some(string) = name.to_string() {
                result.push(&JsValue::from_str(&string));
            } else {
                result.push(&JsValue::from_str("Unknown"));
            }
        }
        return result;
    }

    #[wasm_bindgen]
    pub fn is_regular(&self) -> bool {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.is_regular();
    }

    #[wasm_bindgen(getter)]
    pub fn ascender(&self) -> i16 {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.ascender();
    }
    #[wasm_bindgen(getter)]
    pub fn x_height(&self) -> Option<i16> {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.x_height();
    }
    #[wasm_bindgen(getter)]
    pub fn descender(&self) -> i16 {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.descender();
    }
    #[wasm_bindgen(getter)]
    pub fn capital_height(&self) -> Option<i16> {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.capital_height();
    }

    #[wasm_bindgen(getter)]
    pub fn number_of_glyphs(&self) -> u16 {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.number_of_glyphs();
    }
    #[wasm_bindgen]
    pub fn gid_by_code_point(&self, code_point: &str) -> Option<u16> {
        let face_ref: &Face<'_> = self.face.as_face_ref();

        let gid = code_point
            .chars()
            .next()
            .map(|cp_char| face_ref.glyph_index(cp_char))
            .flatten();

        return gid.map(|x| x.0);
    }
    #[wasm_bindgen]
    pub fn gid_by_name(&self, name: &str) -> Option<u16> {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.glyph_index_by_name(name).map(|x| x.0);
    }
    #[wasm_bindgen]
    pub fn x_advance_by_gid(&self, gid: u16) -> Option<u16> {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.glyph_hor_advance(GlyphId(gid));
    }
    #[wasm_bindgen]
    pub fn y_advance_by_gid(&self, gid: u16) -> Option<u16> {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.glyph_ver_advance(GlyphId(gid));
    }
    #[wasm_bindgen]
    pub fn x_side_bearing_by_gid(&self, gid: u16) -> Option<i16> {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.glyph_hor_side_bearing(GlyphId(gid));
    }
    #[wasm_bindgen]
    pub fn y_side_bearing_by_gid(&self, gid: u16) -> Option<i16> {
        let face_ref: &Face<'_> = self.face.as_face_ref();
        return face_ref.glyph_ver_side_bearing(GlyphId(gid));
    }

    #[wasm_bindgen]
    // TODO: Seperate, make more modular 
    pub fn gid_to_fill_geometry(
        &self,
        gid: u16,
        options: BBFillOptions,
    ) -> Result<BBToGeometryReturn, JsError> {

        let (bounds, vector) = self.bb_vector_by_gid(gid);
        match tesselate_bb_vector_fill(vector, &options.into()) {
            Ok(mut geometry) => {
                // let g = geometry.borrow_mut();
                geometry.translate(0.0, self.capital_height().unwrap_or(0) as f32);
                let ret = BBToGeometryReturn { 
                    geometry: geometry.into(),
                    bounds: bounds.map(|b| b.into())
                 };
                Ok(ret)
            },
            Err(err) => {
                let msg = format!("TessellationError: {}", err.to_string());
                Err(JsError::new(&msg))
            }
        }
    }

    #[wasm_bindgen]
    pub fn shape_text(&self, text: &str) -> BBLineShape {
        let face_ref = self.face.as_face_ref();
        let face = rustybuzz::Face::from_face(face_ref.clone());

        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(text);

        let result = shape(&face, &[], buffer);

        let positions = result.glyph_positions();
        let infos = result.glyph_infos();

        // iterate over the shaped glyphs
        let glyph_shapes: Vec<BBGlyphShape> = positions.iter().zip(infos).map(|(position, info)| {
            let gid = info.glyph_id;
            let cluster = info.cluster;
            let x_advance = position.x_advance;
            let y_advance = position.y_advance;
            let x_offset = position.x_offset;
            let y_offset = position.y_offset;
            let glyph_shape = BBGlyphShape {
                gid,
                cluster,
                x_advance,
                y_advance,
                x_offset,
                y_offset,
            };

            glyph_shape
        }).collect();


        BBLineShape { glyphs: glyph_shapes }
    }
}

impl BBFace {
    pub fn bb_vector_by_gid(&self, gid: u16) -> (Option<BBRect>, BBVector) {
        let face_ref: &Face<'_> = self.face.as_face_ref();

        let mut builder = BBFaceVectorBuilder::new();
        let glyph_id = GlyphId(gid);
        let maybe_bounds = face_ref.outline_glyph(glyph_id, &mut builder);

        let bounds: Option<BBRect> = maybe_bounds.map(|b| b.into());
        (bounds, builder.build())
    }
}
