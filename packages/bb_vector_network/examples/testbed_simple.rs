use bb_vector_network::*;
use comfy::*;

simple_game!("Simple Testbed", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig { ..config }
}

struct Test {
    name: String,
    executor: Box<dyn Fn()>,
}

pub struct GameState {
    tests: Vec<Test>,
    current_test: i32,
}

impl GameState {
    pub fn new(_c: &EngineContext) -> Self {
        Self {
            tests: vec![
                Test {
                    name: "Prong 1".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let (_, root_link) = bbvn.line(Vec2::new(5., 0.), Vec2::new(0., 0.));
                        bbvn.line_from(root_link.end_index(), Vec2::new(-5., -5.));
                        bbvn.line_from(root_link.end_index(), Vec2::new(0., -5.));
                        bbvn.line_from(root_link.end_index(), Vec2::new(-5., 5.));
                        bbvn.line_from(root_link.end_index(), Vec2::new(0., 5.));
                        bbvn.line_from(root_link.end_index(), mouse_world());

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                    }),
                },
                Test {
                    name: "Parallel 1".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let (_, root_link) = bbvn.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
                        bbvn.line_from(root_link.end_index(), Vec2::new(5., 0.));
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(5., 0.),
                            Vec2::new(5., 2.),
                            Vec2::new(5., 5.),
                        );
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(5., 0.),
                            Vec2::new(5., -2.),
                            Vec2::new(5., -5.),
                        );
                        bbvn.translate(Vec2::new(8., 0.));

                        let mut bbvn2 = BBVectorNetwork::new();

                        let (_, root_link) = bbvn2.line(Vec2::new(5., 0.), Vec2::new(0., 0.));
                        bbvn2.line_from(root_link.end_index(), Vec2::new(-5., 0.));
                        bbvn2.cubic_from(
                            root_link.end_index(),
                            Vec2::new(-5., 0.),
                            Vec2::new(-5., 2.),
                            Vec2::new(-5., 5.),
                        );
                        bbvn2.cubic_from(
                            root_link.end_index(),
                            Vec2::new(-5., 0.),
                            Vec2::new(-5., -2.),
                            Vec2::new(-5., -5.),
                        );
                        bbvn2.translate(Vec2::new(-8., 0.));

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                        debug_bbvn(&bbvn2, source);
                    }),
                },
                Test {
                    name: "Parallel 2".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let (_, root_link) = bbvn.line(Vec2::new(0., -5.), Vec2::new(0., 0.));
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., 5.),
                            Vec2::new(-2., 5.),
                            Vec2::new(-5., 5.),
                        );
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., 5.),
                            Vec2::new(-2., 3.),
                            Vec2::new(-5., 3.),
                        );
                        bbvn.translate(Vec2::new(-8., 0.));

                        let mut bbvn2 = BBVectorNetwork::new();

                        let (_, root_link) = bbvn2.line(Vec2::new(0., -5.), Vec2::new(0., 0.));
                        bbvn2.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., 5.),
                            Vec2::new(2., 5.),
                            Vec2::new(5., 5.),
                        );
                        bbvn2.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., 5.),
                            Vec2::new(2., 3.),
                            Vec2::new(5., 3.),
                        );
                        bbvn2.translate(Vec2::new(8., 0.));

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                        debug_bbvn(&bbvn2, source);
                    }),
                },
                Test {
                    name: "Parallel 3".to_string(),
                    executor: Box::new(|| {
                        let source = BBLinkIndex(0);

                        let mut bbvn = BBVectorNetwork::new();
                        let (_, root_link) = bbvn.line(Vec2::new(0., -3.), Vec2::new(0., 0.));
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., 3.),
                            Vec2::new(-1., 3.),
                            Vec2::new(-3., 3.),
                        );
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., -3.),
                            Vec2::new(-1., -3.),
                            Vec2::new(-3., -3.),
                        );
                        bbvn.translate(Vec2::new(-8., 0.));
                        debug_bbvn(&bbvn, source);

                        let mut bbvn = BBVectorNetwork::new();
                        let (_, root_link) = bbvn.line(Vec2::new(0., -3.), Vec2::new(0., 0.));
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., 3.),
                            Vec2::new(-1., 3.),
                            Vec2::new(-3., 3.),
                        );
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., -3.),
                            Vec2::new(1., -3.),
                            Vec2::new(3., -3.),
                        );
                        bbvn.translate(Vec2::new(-4., 0.));
                        debug_bbvn(&bbvn, source);

                        let mut bbvn = BBVectorNetwork::new();
                        let (_, root_link) = bbvn.line(Vec2::new(0., -3.), Vec2::new(0., 0.));
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., 3.),
                            Vec2::new(1., 3.),
                            Vec2::new(3., 3.),
                        );
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., -3.),
                            Vec2::new(-1., -3.),
                            Vec2::new(-3., -3.),
                        );
                        bbvn.translate(Vec2::new(4., 0.));
                        debug_bbvn(&bbvn, source);

                        let mut bbvn = BBVectorNetwork::new();
                        let (_, root_link) = bbvn.line(Vec2::new(0., -3.), Vec2::new(0., 0.));
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., 3.),
                            Vec2::new(1., 3.),
                            Vec2::new(3., 3.),
                        );
                        bbvn.cubic_from(
                            root_link.end_index(),
                            Vec2::new(0., -3.),
                            Vec2::new(1., -3.),
                            Vec2::new(3., -3.),
                        );
                        bbvn.translate(Vec2::new(8., 0.));
                        debug_bbvn(&bbvn, source);
                    }),
                },
                Test {
                    name: "Shape 1".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let (_, first_link) = bbvn.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
                        let (_, middle_link) =
                            bbvn.line_from(first_link.end_index(), Vec2::new(0., 5.));
                        let (_, last_link) =
                            bbvn.line_from(middle_link.end_index(), Vec2::new(-5., 5.));
                        bbvn.line_from_to(last_link.end_index(), first_link.start_index());
                        let time = comfy::get_time();
                        if time.sin() > -0.4 {
                            let (_, last_link) =
                                bbvn.line_from(first_link.end_index(), Vec2::new(5., 0.));
                            if time.sin() > 0.0 {
                                let (_, last_link) =
                                    bbvn.line_from(last_link.end_index(), Vec2::new(5., 5.));
                                if time.sin() > 0.4 {
                                    bbvn.line_from_to(last_link.end_index(), middle_link.end_index());
                                }
                            }
                        }

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                    }),
                },
                Test {
                    name: "Shape 2".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let (_, first_link) = bbvn.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
                        let (_, middle_link) =
                            bbvn.line_from(first_link.end_index(), Vec2::new(0., 5.));
                        let (_, last_link) =
                            bbvn.line_from(middle_link.end_index(), Vec2::new(-5., 5.));
                        bbvn.line_from_to(last_link.end_index(), first_link.start_index());
                        let (_, top_link) = bbvn.line_to(Vec2::new(5., 5.), middle_link.end_index());
                        let (_, bottom_link) = bbvn.line_to(Vec2::new(5., 0.), middle_link.start_index());

                        let time = comfy::get_time();
                        if time.sin() > 0.0 {
                            let _ = bbvn.line_from_to(bottom_link.start_index(), top_link.start_index());
                        }

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                    }),
                },
            ],
            current_test: 0,
        }
    }
}

fn debug_bbvn(bbvn: &BBVectorNetwork, source_link: BBLinkIndex) {
    #[cfg(feature = "debug_draw")]
    {
        bbvn.debug_draw();

        let first = *bbvn.link(source_link).unwrap();
        let next_links = first.next_links(bbvn);

        comfy::draw_circle(first.end_pos(bbvn), 0.1, WHITE, 11);

        let next = first.ccw_most_next_link(bbvn, &next_links[..]);
        let Some(next) = next else {
            comfy::draw_text("No next index", Vec2::ZERO, comfy::RED, TextAlign::Center);
            return;
        };
        let next = *bbvn.link(next).unwrap();
        next.debug_draw_with_color_and_z_index(bbvn, comfy::Color::rgb8(100, 255, 100), 11);
    }
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    if is_key_pressed(KeyCode::Left) {
        state.current_test = (state.current_test - 1) % state.tests.len() as i32;
    }
    if is_key_pressed(KeyCode::Right) {
        state.current_test = (state.current_test + 1) % state.tests.len() as i32;
    }

    let Some(test) = state.tests.get(state.current_test as usize) else {
        draw_text("Invalid Test", Vec2::new(0., 0.), WHITE, TextAlign::Center);
        return;
    };

    draw_text(
        &format!("<< {} >>", test.name),
        Vec2::new(0., -8.),
        WHITE,
        TextAlign::Center,
    );

    (test.executor)();
}

#[cfg(not(feature = "debug_draw"))]
fn main() {
    panic!("Testbeds require the `--features debug_draw` feature to be enabled.");
}
