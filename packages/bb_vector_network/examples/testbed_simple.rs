use bb_vector_network::*;
use comfy::*;

simple_game!("Simple Testbed", GameState, config, setup, update);

fn config(config: GameConfig) -> GameConfig {
    GameConfig {
        ..config
    }
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
                    name: "Determinate 1".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let endpoint = bbvn.line(Vec2::new(0., -5.), Vec2::new(0., 0.));
                        bbvn.line_from(endpoint, Vec2::new(-3., 3.));
                        bbvn.line_from(endpoint, Vec2::new(3., 3.));

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                    }),
                },
                Test {
                    name: "Prong 1".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let endpoint = bbvn.line(Vec2::new(5., 0.), Vec2::new(0., 0.));
                        bbvn.line_from(endpoint, mouse_world());
                        bbvn.line_from(endpoint, Vec2::new(-5., 0.));

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                    }),
                },
                Test {
                    name: "Prong 2".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let endpoint = bbvn.line(Vec2::new(-0., 5.), Vec2::new(0., 0.));
                        bbvn.line_from(endpoint, mouse_world());
                        bbvn.line_from(endpoint, Vec2::new(-5., -0.));

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                    }),
                },
                Test {
                    name: "Prong 3".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let endpoint = bbvn.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
                        bbvn.line_from(endpoint, mouse_world());
                        bbvn.line_from(endpoint, Vec2::new(0., 5.));

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                    }),
                },
                Test {
                    name: "Prong 4".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let endpoint = bbvn.line(Vec2::new(5., 0.), Vec2::new(0., 0.));
                        bbvn.line_from(endpoint, Vec2::new(-5., -5.));
                        bbvn.line_from(endpoint, Vec2::new(0., -5.));
                        bbvn.line_from(endpoint, Vec2::new(-5., 5.));
                        bbvn.line_from(endpoint, Vec2::new(0., 5.));
                        bbvn.line_from(endpoint, mouse_world());

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                    }),
                },
                Test {
                    name: "Parallel 1".to_string(),
                    executor: Box::new(|| {
                        let mut bbvn = BBVectorNetwork::new();

                        let endpoint = bbvn.line(Vec2::new(-5., 0.), Vec2::new(0., 0.));
                        bbvn.line_from(endpoint, Vec2::new(5., 0.));
                        bbvn.cubic_from(endpoint, Vec2::new(5., 0.), Vec2::new(5., 2.), Vec2::new(5., 5.));
                        bbvn.cubic_from(endpoint, Vec2::new(5., 0.), Vec2::new(5., -2.), Vec2::new(5., -5.));
                        bbvn.translate(Vec2::new(8., 0.));

                        let mut bbvn2 = BBVectorNetwork::new();

                        let endpoint = bbvn2.line(Vec2::new(5., 0.), Vec2::new(0., 0.));
                        bbvn2.line_from(endpoint, Vec2::new(-5., 0.));
                        bbvn2.cubic_from(endpoint, Vec2::new(-5., 0.), Vec2::new(-5., 2.), Vec2::new(-5., 5.));
                        bbvn2.cubic_from(endpoint, Vec2::new(-5., 0.), Vec2::new(-5., -2.), Vec2::new(-5., -5.));
                        bbvn2.translate(Vec2::new(-8., 0.));

                        let source = BBLinkIndex(0);
                        debug_bbvn(&bbvn, source);
                        debug_bbvn(&bbvn2, source);
                    }),
                },
            ],
            current_test: 0,
        }
    }
}

fn debug_bbvn(bbvn: &BBVectorNetwork, source_link: BBLinkIndex) {
    #[cfg(feature = "debug_draw")]
    bbvn.debug_draw();

    let first = *bbvn.link(source_link).unwrap();
    let next_links = first.next_links(bbvn);

    let next = first.ccw_most_next_link(bbvn, &next_links[..]);
    let Some(next) = next else {
        comfy::draw_text("No next index", Vec2::ZERO, comfy::RED, TextAlign::Center);
        return;
    };
    let next = *bbvn.link(next).unwrap();
    #[cfg(feature = "debug_draw")]
    next.debug_draw_with_color_and_z_index(bbvn, comfy::Color::rgb8(100, 255, 100), 11);
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {}

fn update(state: &mut GameState, _c: &mut EngineContext) {
    #[cfg(not(feature = "debug_draw"))]
    panic!("Testbeds require the `--features debug_draw` feature to be enabled.");

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
    //
    // draw_text(
    //     "Straight lines",
    //     Vec2::new(-10., 7.),
    //     WHITE,
    //     TextAlign::TopLeft,
    // );
    // {
    // }
    // {
    // }
    // {
    //     let mut bbvn = BBVectorNetwork::new();
    //
    //     let endpoint = bbvn.line(Vec2::new(-2., 0.), Vec2::new(0., 0.));
    //     bbvn.line_from(endpoint, mouse_world());
    //     bbvn.line_from(endpoint, Vec2::new(0., 2.));
    //
    //     bbvn.translate(Vec2::new(-9., -6.));
    //
    //     let source = BBLinkIndex(0);
    //     debug_bbvn(&bbvn, source);
    // }
    //
    // {
    //     let mut bbvn = BBVectorNetwork::new();
    //
    //     let endpoint = bbvn.line(Vec2::new(-2., 0.), Vec2::new(0., 0.));
    //     bbvn.cubic_from(
    //         endpoint,
    //         Vec2::new(2., 1.),
    //         Vec2::new(2., 2.),
    //         mouse_world(),
    //     );
    //     bbvn.line_from(endpoint, Vec2::new(2., 0.));
    //
    //     bbvn.translate(Vec2::new(-0., -4.));
    //
    //     let source = BBLinkIndex(0);
    //     debug_bbvn(&bbvn, source);
    // }
    // {
    //     let mut bbvn = BBVectorNetwork::new();
    //
    //     let endpoint = bbvn.line(Vec2::new(-2., 0.), Vec2::new(0., 0.));
    //     bbvn.cubic_from(
    //         endpoint,
    //         Vec2::new(2., -1.),
    //         Vec2::new(2., 2.),
    //         mouse_world(),
    //     );
    //     bbvn.line_from(endpoint, Vec2::new(2., 0.));
    //
    //     bbvn.translate(Vec2::new(-0., -0.));
    //
    //     let source = BBLinkIndex(0);
    //     debug_bbvn(&bbvn, source);
    // }
}
