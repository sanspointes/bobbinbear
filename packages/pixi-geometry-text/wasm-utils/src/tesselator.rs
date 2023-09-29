use lyon::math::Point;
use lyon::tessellation::geometry_builder::simple_builder;
use lyon::tessellation::{FillOptions, FillTessellator, VertexBuffers};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const M_STEP: usize = 2;
const L_STEP: usize = 2;
const Q_STEP: usize = 4;
const C_STEP: usize = 6;
const Z_STEP: usize = 0;
const X_STEP: usize = 0;

pub fn tesselate_font_path(cmds: &Vec<char>, crds: &Vec<f32>, options: &FillOptions) -> VertexBuffers<Point, u16> {
    let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();
    let mut geometry_builder = simple_builder(&mut geometry);

    let mut tesselator = FillTessellator::new();

    let mut builder = tesselator.builder(&options, &mut geometry_builder);

    let mut path_active = false;
    let mut crdi = 0;
    for cmd in cmds {
        match cmd {
            'M' => {
                let crd_end = crdi + M_STEP;
                if path_active == true {
                    builder.end(false);
                };

                if let [x, y] = &crds[crdi..crd_end] {
                    let p = Point::new(*x, *y);
                    builder.begin(p);
                    path_active = true;
                } else {
                    panic!("{}", "Could not get range {crdi}->{crd_end} of {crds:?}");
                }
                crdi += M_STEP;
            }
            'L' => {
                let crd_end = crdi + L_STEP;
                if let [x, y] = &crds[crdi..crd_end] {
                    let p = Point::new(*x, *y);
                    builder.line_to(p);
                } else {
                    panic!("{}", "Could not get range {crdi}->{crd_end} of {crds:?}");
                }
                crdi += L_STEP;
            }
            'Q' => {
                let crd_end = crdi + Q_STEP;
                if let [c0x, c0y, x, y] = &crds[crdi..crd_end] {
                    let c0 = Point::new(*c0x, *c0y);
                    let p = Point::new(*x, *y);
                    builder.quadratic_bezier_to(c0, p);
                } else {
                    panic!("{}", "Could not get range {crdi}->{crd_end} of {crds:?}");
                }
                crdi += Q_STEP;
            }
            'C' => {
                let crd_end = crdi + C_STEP;
                if let [c0x, c0y, c1x, c1y, x, y] = &crds[crdi..crd_end] {
                    let c0 = Point::new(*c0x, *c0y);
                    let c1 = Point::new(*c1x, *c1y);
                    let p = Point::new(*x, *y);
                    builder.cubic_bezier_to(c0, c1, p);
                } else {
                    panic!("{}", "Could not get range {crdi}->{crd_end} of {crds:?}");
                }
                crdi += C_STEP;
            }
            'Z' => {
                // TODO
                builder.end(true);
                path_active = false;
                crdi += Z_STEP;
            }
            'X' => {
                builder.end(true);
                path_active = false;
                crdi += X_STEP;
            }
            _ => {
                // TODO:
                panic!("Unknown path command.");
            }
        }
    }

    builder.build().unwrap();

    return geometry;
}
