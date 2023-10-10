use bevy::prelude::*;
use bevy_mod_raycast::DefaultRaycastingPlugin;
use bevy_mod_raycast::RaycastMesh;
use bevy_prototype_lyon::prelude::*;
use bevy_mod_raycast::RaycastMethod;
use bevy_mod_raycast::RaycastSource;
use bevy_mod_raycast::RaycastSystem;

use crate::editor2::constants::BG_HIT_Z_INDEX;
use crate::editor2::msgs::editor_msg_system;

// use super::EditorSet;
#[derive(Clone, Reflect)]
pub struct RaycastSelectable;

#[derive(Component)]
pub struct MyCameraTag {
    /// The minimum scale for the camera
    ///
    /// The orthographic projection's scale will be clamped at this value when zooming in
    pub min_scale: f32,
    /// The maximum scale for the camera
    ///
    /// If present, the orthographic projection's scale will be clamped at
    /// this value when zooming out.
    pub max_scale: Option<f32>,
    /// The minimum x position of the camera window
    ///
    /// If present, the orthographic projection will be clamped to this boundary both
    /// when dragging the window, and zooming out.
    pub min_x: Option<f32>,
    /// The maximum x position of the camera window
    ///
    /// If present, the orthographic projection will be clamped to this boundary both
    /// when dragging the window, and zooming out.
    pub max_x: Option<f32>,
    /// The minimum y position of the camera window
    ///
    /// If present, the orthographic projection will be clamped to this boundary both
    /// when dragging the window, and zooming out.
    pub min_y: Option<f32>,
    /// The maximum y position of the camera window
    ///
    /// If present, the orthographic projection will be clamped to this boundary both
    /// when dragging the window, and zooming out.
    pub max_y: Option<f32>,
}

impl Default for MyCameraTag {
    fn default() -> Self {
        Self {
            min_scale: 0.00001,
            max_scale: None,
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        }
    }
}

#[derive(Resource, Default)]
pub struct CameraResource {}

#[derive(Event, Debug, Clone)]
pub enum CameraMessage {
    SetTranslate { pos: Vec3 },
    UpdateBounds { rect: Rect, padding: Vec2 },
}
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        println!("Setting up camera plugin.");
        app
            .add_event::<CameraMessage>()
            .insert_resource(CameraResource::default())
            .add_startup_system(setup_bg_hit_plane)
            .add_system(move_bg_hit_plane_system) // Moves bg hit plane to follow camera
            // Raycast cursor updates
            .add_plugin(DefaultRaycastingPlugin::<RaycastSelectable>::default())
            .add_systems(First,
                update_raycast_selectable_with_cursor
                    .before(RaycastSystem::BuildRays::<RaycastSelectable>),
            )
            .add_plugin(DefaultRaycastingPlugin::<RaycastRawInput>::default())
            .add_systems(First,
                update_raycast_raw_input_with_cursor
                    .before(RaycastSystem::BuildRays::<RaycastRawInput>),
            )
            .add_plugin(DefaultRaycastingPlugin::<RaycastTools>::default())
            .add_systems(First,
                update_raycast_tool_with_cursor
                    .before(RaycastSystem::BuildRays::<RaycastTools>),
            )
            .add_systems(Update, camera_message_system.after(editor_msg_system))
            .add_startup_system(camera_startup_system);
    }
}

// Update our `RaycastSource` with the current cursor position every frame.
fn update_raycast_selectable_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut raycast_selectable: Query<&mut RaycastSource<RaycastSelectable>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut raycast_selectable {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

fn update_raycast_raw_input_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut raycast_raw_input: Query<&mut RaycastSource<RaycastRawInput>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut raycast_raw_input {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

/// Sets up the entities required to view the document on startup.
pub fn camera_startup_system(mut commands: Commands) {
    commands
        .spawn(MyCameraTag::default())
        .insert(Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1., 1., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RaycastSource::<RaycastSelectable>::new())
        .insert(RaycastSource::<RaycastRawInput>::new())
    ;
}

pub fn camera_message_system(
    // mut res: ResMut<CameraResource>,
    mut ev_reader: EventReader<CameraMessage>,
    // mut frontend_writer: EventWriter<FrontendMessage>,
    mut camera: Query<(&mut MyCameraTag, &mut Transform)>,
    // document_entities: Query<(Entity, &CameraTag, &mut Visibility)>,
) {
    for msg in ev_reader.iter() {
        match msg {
            CameraMessage::SetTranslate { pos } => {
                let (_, mut transform) = camera.single_mut();
                transform.translation = *pos;
            }
            CameraMessage::UpdateBounds { rect, padding } => {
                let (mut my_cam, _) = camera.single_mut();
                my_cam.min_x = Some(rect.min.x - padding.x);
                my_cam.min_y = Some(rect.min.y - padding.y);
                my_cam.max_x = Some(rect.max.x + padding.x);
                my_cam.max_y = Some(rect.max.y + padding.y);
            }
        }
    }
}

// BG Hit plane, responsible for proving the background color +
// mapping mouse events to world coordinates

#[derive(Component)]
pub struct BgHitPlane;

#[derive(Debug, Clone, Reflect)]
pub struct RaycastRawInput;

pub fn setup_bg_hit_plane(mut commands: Commands) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(10000., 10000.),
        ..Default::default()
    };
    commands.spawn((
        Name::from("BgHitPlane"),
        BgHitPlane {},
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            transform: Transform {
                translation: Vec3::new(0., 0., BG_HIT_Z_INDEX),
                ..Default::default()
            },
            ..Default::default()
        },
        RaycastMesh::<RaycastRawInput>::default(),
        Fill::color(Color::rgb(0.2, 0.2, 0.2)),
    ));
}
pub fn move_bg_hit_plane_system(
    cam: Query<&Transform, (With<Camera2d>, Without<BgHitPlane>)>,
    mut bg_hit_plane: Query<&mut Transform, (With<BgHitPlane>, Without<Camera2d>)>) {
    if let (Ok(cam_transform), Ok(mut bg_hit_transform)) = (cam.get_single(), bg_hit_plane.get_single_mut()) {
        bg_hit_transform.translation.x = cam_transform.translation.x;
        bg_hit_transform.translation.y = cam_transform.translation.y;
    }
}



#[derive(Debug, Clone, Reflect)]
pub struct RaycastTools;
fn update_raycast_tool_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut raycast_selectable: Query<&mut RaycastSource<RaycastTools>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut raycast_selectable {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}
