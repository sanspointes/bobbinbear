use bevy::{
    core::Name,
    ecs::{component::Component, event::Events, reflect::ReflectComponent, system::Commands, world::World},
    hierarchy::BuildWorldChildren,
    log::warn,
    reflect::Reflect,
};
use bevy_spts_uid::{Uid, UidRegistry};
use moonshine_core::{kind::Instance, object::Object};

use crate::{
    ecs::{InternalObject, ObjectBundle, ObjectType, ProxiedObjectBundle},
    materials::UiElementMaterialCache,
    meshes::BobbinMeshesResource,
    plugins::{
        effect::Effect,
        model_view::{BuildView, View, ViewBuilder},
        viewport::BobbinViewportResource,
    },
};

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
/// View/Model marker struct for view_model plugin.  When this is present it will generates views
/// for the Object::VectorEndpoint entity.
pub struct VectorEndpointVM;

impl BuildView<VectorEndpointVM> for VectorEndpointVM {
    fn build(
        world: &World,
        object: Object<VectorEndpointVM>,
        view: &mut ViewBuilder<VectorEndpointVM>,
    ) {
        warn!(
            "Building view for VectorEndpointModel {:?}",
            object.entity()
        );
        let material = world.resource::<UiElementMaterialCache>().default.clone();
        let mesh = world
            .resource::<BobbinMeshesResource>()
            .endpoint_mesh()
            .clone();
        let endpoint_uid = world.resource::<UidRegistry>().uid(object.entity());
        let uid = Uid::default();
        view.insert((
            Name::from("VectorEndpoint (View)"),
            ObjectBundle::new(ObjectType::VectorEndpoint).with_z_position(10.),
            ProxiedObjectBundle::new(endpoint_uid),
            InternalObject,
            material,
            mesh,
            uid,
        ));
        let viewport_entity = world.resource::<BobbinViewportResource>().viewport_entity();
        let view_entity = view.entity();

        view.commands().commands().add(move |world: &mut World| {
            world.entity_mut(view_entity).set_parent(viewport_entity);
        });
        view.commands().commands().add(move |world: &mut World| {
            world
                .resource_mut::<Events<Effect>>()
                .send(Effect::EntitiesSpawned(vec![uid]));
        });
        view.commands().commands().add(move |world: &mut World| {
            world.resource_mut::<UidRegistry>().register(uid, view_entity);
        });
    }

    fn on_before_destroy(
        world: &World,
        _model: Instance<VectorEndpointVM>,
        view: Instance<View<VectorEndpointVM>>,
        commands: &mut Commands,
    ) {
        let view_uid = *world.get::<Uid>(view.entity()).unwrap();
        commands.add(move |world: &mut World| {
            world
                .resource_mut::<Events<Effect>>()
                .send(Effect::EntitiesDespawned(vec![view_uid]));
        });
        commands.add(move |world: &mut World| {
            world.resource_mut::<UidRegistry>().unregister(view_uid);
        });
    }
}
