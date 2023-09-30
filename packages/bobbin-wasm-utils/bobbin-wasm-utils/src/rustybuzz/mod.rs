use std::collections::HashMap;

use rustybuzz::{Face, shape, UnicodeBuffer};
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct BBFaceHandle {
    face_id: u16,
}

#[derive(Error, Debug)]
pub enum BBTypeError {
    #[error("missing face with id {0:?}.")]
    MissingFace(u16),
}

#[wasm_bindgen]
struct BBGlyphShape {
    gid: u32,
    cluster: u32,
    x_advance: i32,
    y_advance: i32,
    x_offset: i32,
    y_offset: i32,
}

#[wasm_bindgen]
struct BBLineShape {
    entries: Vec<BBGlyphShape>
}

struct BBFaceInternal {
    source: Vec<u8>,
}

#[wasm_bindgen]
/// BBType, entry point for the font shaping and drawing api.
struct BBType {
    face_offset: u16,
    faces: HashMap<u16, BBFaceInternal>,
}

#[wasm_bindgen]
impl BBType {
    #[wasm_bindgen(constructor)]
    /// Creates a new BBType for storing and performing operations on fonts.
    pub fn new() -> Self {
        Self {
            face_offset: 0,
            faces: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    /// Adds a face to the internal store and returns a handle for operations on that face.
    ///
    /// * `face_data`: 
    pub fn add_face(&mut self, face_data: &[u8]) -> Option<BBFaceHandle> {
        let data = Vec::from(face_data);
        let face = Face::from_slice(&data, 0).to_owned();
        if let Some(face) = face {
            let face_id = self.face_offset;
            self.faces.insert(face_id, face);

            self.face_offset += 1;
            return Some(BBFaceHandle { face_id });
        }
        None
    }

    #[wasm_bindgen]
    /// Clears a face from the internal store.
    ///
    /// * `face_handle`: 
    pub fn clear_face(&mut self, face_handle: BBFaceHandle) -> Result<(), JsError> {
        let value = self.faces.remove(&face_handle.face_id);

        if let Some(v) = value {
            Ok(())
        } else {
            Err(BBTypeError::MissingFace(face_handle.face_id).into())
        }
    }

    fn get_face(&self, face_handle: &BBFaceHandle) -> Option<BBFaceInternal> {
        self.faces.get(&face_handle.face_id)
    }

    #[wasm_bindgen]
    pub fn shape_line(&mut self, face_handle: BBFaceHandle, text: &str) -> Result<BBLineShape, JsError> {
        let face = self.get_face(&face_handle);

        if let None = face {
            return Err(BBTypeError::MissingFace(face_handle.face_id).into())
        }
        let face = face.unwrap();

        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(text);
        let result = shape(&face, &[], buffer);

        let positions = result.glyph_positions();
        let infos = result.glyph_infos();

        let glyph_shapes = Vec::<BBGlyphShape>::with_capacity(positions.len());
        // iterate over the shaped glyphs
        let glyph_shapes: Vec<BBGlyphShape> = positions.iter().zip(infos).map(|(position, info)| {
            let gid = info.glyph_id;
            let cluster = info.cluster;
            let x_advance = position.x_advance;
            let y_advance = position.y_advance;
            let x_offset = position.x_offset;
            let y_offset = position.y_offset;
            BBGlyphShape {
                gid,
                cluster,
                x_advance,
                y_advance,
                x_offset,
                y_offset,
            }
        }).collect();

        Ok(BBLineShape { entries: glyph_shapes })
    }
}
