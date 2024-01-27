use bevy::{
    ecs::query::{QueryEntityError, ROQueryItem, WorldQuery},
    math::{vec2, vec3},
    prelude::*,
    sprite::{ColorMaterial, Mesh2dHandle},
};

use crate::utils::mesh::{add_vertex_colors_mesh, combine_meshes};

#[derive(Resource, Default, Clone)]
/// TODO: Move this to a more general location
///
/// * `material`:
/// * `control_node`:
/// * `endpoint_node`:
pub struct CachedMeshes {
    pub material: Option<Handle<ColorMaterial>>,
    pub control_node: Option<Mesh2dHandle>,
    pub endpoint_node: Option<Mesh2dHandle>,
}

pub fn sys_setup_cached_meshes(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut resource: ResMut<CachedMeshes>,
) {
    resource.material = Some(materials.add(ColorMaterial::default()));
    // Builds the control node mesh (square)
    {
        let mut control_node_m1 = Mesh::from(shape::Quad::new(vec2(3., 3.)));
        add_vertex_colors_mesh(&mut control_node_m1, Color::WHITE);
        let mut control_node_m2 = Mesh::from(shape::Quad::new(vec2(5., 5.)));
        add_vertex_colors_mesh(&mut control_node_m2, Color::BLUE);

        let to_combine = [control_node_m1, control_node_m2];
        let transforms = [
            Transform::default(),
            Transform {
                translation: vec3(0., 0., 1.),
                ..Default::default()
            },
        ];

        let combined = combine_meshes(&to_combine, &transforms, true, false, true, true);
        println!("Generating control node mesh {combined:?}");
        let handle = meshes.add(combined);
        resource.control_node = Some(handle.into());
    }

    // Builds the control node mesh (square)
    {
        let mut control_node_m1 = Mesh::from(shape::Circle::new(3.));
        add_vertex_colors_mesh(&mut control_node_m1, Color::WHITE);
        let mut control_node_m2 = Mesh::from(shape::Circle::new(5.));
        add_vertex_colors_mesh(&mut control_node_m2, Color::BLUE);

        let to_combine = [control_node_m1, control_node_m2];
        let transforms = [
            Transform::default(),
            Transform {
                translation: vec3(0., 0., 1.),
                ..Default::default()
            },
        ];

        let handle = meshes.add(combine_meshes(
            &to_combine,
            &transforms,
            true,
            false,
            true,
            true,
        ));
        resource.endpoint_node = Some(handle.into());
    }
}

pub trait WorldUtils {
    fn get_bundle<B: WorldQuery>(
        &mut self,
        entity: Entity,
    ) -> Result<ROQueryItem<B>, QueryEntityError>;
    fn bundle<B: WorldQuery>(&mut self, entity: Entity) -> ROQueryItem<B>;

    fn has_component<C: Component>(&self, entity: Entity) -> bool;
}

impl WorldUtils for World {
    fn get_bundle<B: WorldQuery>(
        &mut self,
        entity: Entity,
    ) -> Result<ROQueryItem<B>, QueryEntityError> {
        self.query::<B>().get(self, entity)
    }
    fn bundle<B: WorldQuery>(&mut self, entity: Entity) -> ROQueryItem<B> {
        self.get_bundle::<B>(entity).unwrap()
    }

    fn has_component<C: Component>(&self, entity: Entity) -> bool {
        self.get::<C>(entity).is_some()
    }
}
