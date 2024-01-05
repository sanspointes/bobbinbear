use comfy::*;

pub const DRAG_THRESHOLD: f32 = 0.02;

#[derive(Default)]
pub struct InputHelper {
    mouse_pos: Vec2,
    mouse_press_pos: Option<Vec2>,
    is_dragging: bool,
}

#[derive(Debug)]
pub enum InputEvent {
    MouseMove {
        position: Vec2,
    },
    MouseClick {
        position: Vec2,
    },
    DragStart {
        position: Vec2,
    },
    DragMove {
        start_position: Vec2,
        position: Vec2,
    },
    DragEnd {
        start_position: Vec2,
        position: Vec2,
    }
}

impl InputHelper {
    pub fn compute_mouse_events(&mut self) -> Vec<InputEvent> {
        let new_mouse_pos = mouse_world();
        // if  {
        //     self.mouse_press_pos = Some(mouse_world());
        // } else if is_mouse_button_released(MouseButton::Left) {
        //     self.mouse_press_pos = None;
        // }

        let moved = self.mouse_pos != new_mouse_pos;

        let lm_pressed = is_mouse_button_pressed(MouseButton::Left);
        let lm_released = is_mouse_button_released(MouseButton::Left);

        let mut events = vec![];

        if lm_pressed {
            self.mouse_press_pos = Some(mouse_world());
        }

        match (self.mouse_press_pos, moved, self.is_dragging) {
            // Moved while down but not dragging
            (Some(p), true, false) => {
                if p.distance(mouse_world()) > DRAG_THRESHOLD {
                    self.is_dragging = true;
                    events.push(InputEvent::DragStart { position: p })
                }
            }
            (Some(p), true, true) => {
                events.push(InputEvent::DragMove { start_position: p, position: mouse_world() });
            }
            (None, true, false) => {
                events.push(InputEvent::MouseMove { position: mouse_world() });
            }
            _ => (),
        }

        if self.mouse_press_pos.is_some() && lm_released {
            let p = self.mouse_press_pos.unwrap();
            if self.is_dragging {
                events.push(InputEvent::DragEnd { start_position: p, position: mouse_world() });
            } else {
                events.push(InputEvent::MouseClick { position: p })
            }
            self.mouse_press_pos = None;
            self.is_dragging = false;
        }

        events
    }
}
