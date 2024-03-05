// use bevy_app::App;
// use bevy_ecs::{entity::Entity, query::With, world::World};
// use bevy_hierarchy::BuildWorldChildren;
// use bevy_math::{vec2, Vec3};
// use bevy_spts_vector_graphic::prelude::*;
// use bevy_transform::{components::Transform, TransformBundle};
//
// #[test]
// pub fn it_detects_changes_when_vector_graphic_spawned() {
//     let mut app = App::new();
//
//     let world = &mut app.world;
//
//     build_endpoints(world);
//
//     let sys_id = world.register_system(sys_check_vector_graphic_children_changed);
//     let result = world.run_system(sys_id).unwrap();
//
//     let vg = world
//         .query_filtered::<Entity, With<VectorGraphic>>()
//         .single(world);
//     assert_eq!(result.len(), 1);
//     assert_eq!(vg, *result.first().unwrap());
//
//     app.update();
//
//     let world = &mut app.world;
//     let result = world.run_system(sys_id).unwrap();
//
//     assert_eq!(result.len(), 0);
// }
//
// #[test]
// pub fn it_detects_changes_when_vector_graphic_endpoint_moved() {
//     let mut app = App::new();
//
//     let world = &mut app.world;
//
//     let (vg, _) = build_box(world);
//
//     let sys_id = world.register_system(sys_check_vector_graphic_children_changed);
//     world.run_system(sys_id).unwrap();
//
//     app.update();
//     let world = &mut app.world;
//
//     let mut q_endpoints = world.query_filtered::<&mut Transform, With<Endpoint>>();
//     let mut first = q_endpoints.iter_mut(world).next().unwrap();
//     first.translation.x = 10.;
//
//     let result = world.run_system(sys_id).unwrap();
//
//     assert_eq!(result.len(), 1);
//     assert_eq!(vg, *result.first().unwrap());
// }
//
// #[test]
// pub fn it_detects_changes_when_vector_graphic_edge_added() {
//     let mut app = App::new();
//     let world = &mut app.world;
//     let (vg, (p0, p1, p2, p3)) = build_endpoints(world);
//     world.spawn_edge(EdgeVariant::Line, p0, p1);
//     world.spawn_edge(EdgeVariant::Line, p1, p2);
//
//     let sys_id = world.register_system(sys_check_vector_graphic_children_changed);
//     world.run_system(sys_id).unwrap();
//     app.update();
//     let world = &mut app.world;
//
//     world.spawn_edge(EdgeVariant::Line, p2, p3);
//
//     let result = world.run_system(sys_id).unwrap();
//
//     assert_eq!(result.len(), 1);
//     assert_eq!(vg, *result.first().unwrap());
// }
//
// #[test]
// pub fn it_detects_changes_when_vector_graphic_edge_moved() {
//     let mut app = App::new();
//     let world = &mut app.world;
//     let (vg, (p0, p1, p2, p3)) = build_endpoints(world);
//     world.spawn_edge(EdgeVariant::Quadratic { ctrl1: vec2(25., 50.) }, p0, p1);
//     world.spawn_edge(EdgeVariant::Quadratic { ctrl1: vec2(75., 50.) }, p1, p2);
//     world.spawn_edge(EdgeVariant::Quadratic { ctrl1: vec2(75., 50.) }, p2, p3);
//
//     let sys_id = world.register_system(sys_check_vector_graphic_children_changed);
//     world.run_system(sys_id).unwrap();
//     app.update();
//     let world = &mut app.world;
//
//     let mut edge = world.query::<&mut EdgeVariant>().iter_mut(world).next().unwrap();
//     match *edge {
//         EdgeVariant::Quadratic { ref mut ctrl1 } => {
//             *ctrl1 += vec2(10., 10.);
//         }
//         _ => panic!("Impossible. Only quadratics in this test case.")
//     }
//
//     let result = world.run_system(sys_id).unwrap();
//
//     assert_eq!(result.len(), 1);
//     assert_eq!(vg, *result.first().unwrap());
// }
//
// #[test]
// pub fn it_detects_changes_when_vector_graphic_edge_removed() {
//     let mut app = App::new();
//     let world = &mut app.world;
//     let (vg, (p0, p1, p2, p3)) = build_endpoints(world);
//     world.spawn_edge(EdgeVariant::Line, p0, p1);
//     let edge_id = world.spawn_edge(EdgeVariant::Line, p1, p2).id();
//
//     let sys_id = world.register_system(sys_check_vector_graphic_children_changed);
//     world.run_system(sys_id).unwrap();
//     app.update();
//     let world = &mut app.world;
//
//     world.despawn_edge(edge_id);
//
//     let result = world.run_system(sys_id).unwrap();
//
//     assert_eq!(result.len(), 1);
//     assert_eq!(vg, *result.first().unwrap());
// }
