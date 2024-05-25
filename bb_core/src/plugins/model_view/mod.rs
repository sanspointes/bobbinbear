//! ModelView plugin is based off moonshine_view but modified to allow customizing the schedules it runs within
//! and removes the dependency on moonshine_core::load

use std::any::TypeId;

use bevy::{
    app::prelude::*,
    ecs::{prelude::*, schedule::ScheduleLabel},
    hierarchy::prelude::*,
    log::warn,
    utils::{HashMap, HashSet},
};

use moonshine_core::prelude::*;

/// Extension trait used to register views using an [`App`].
#[allow(dead_code)]
pub trait RegisterView {
    /// Registers a view for a given [`Kind`].
    fn register_view<T: Kind, V: BuildView<T>>(
        &mut self,
        spawn_schedule: impl ScheduleLabel,
        despawn_schedule: impl ScheduleLabel,
    ) -> &mut Self;

    /// Registers a given [`Kind`] as viewable.
    fn register_viewable<T: BuildView>(
        &mut self,
        spawn_schedule: impl ScheduleLabel,
        despawn_schedule: impl ScheduleLabel,
    ) -> &mut Self {
        self.register_view::<T, T>(spawn_schedule, despawn_schedule)
    }
}

impl RegisterView for App {
    fn register_view<T: Kind, V: BuildView<T>>(
        &mut self,
        spawn_schedule: impl ScheduleLabel,
        despawn_schedule: impl ScheduleLabel,
    ) -> &mut Self {
        warn!("Registering view in schedule {spawn_schedule:?} {despawn_schedule:?}");
        self.add_systems(spawn_schedule, spawn::<T, V>);
        let mut viewables = self.world.get_resource_or_insert_with(Viewables::default);
        if !viewables.is_viewable_kind::<T>() {
            viewables.add_kind::<T>();
            self.add_systems(despawn_schedule, despawn::<T, V>);
        }
        self
    }
}

/// Trait used to spawn a [`View`] [`Entity`] for an [`Instance`] of [`Kind`] `T`.
pub trait BuildView<T: Kind = Self>: Kind {
    /// Called when a new [`Instance`] of [`Kind`] `T` is spawned without a [`View`].
    ///
    /// Remember to register this type using [`RegisterView`] for this to happen.
    fn build(_world: &World, _object: Object<T>, view: &mut ViewBuilder<T>);

    /// Called when the view is destroyed
    fn on_before_destroy(_world: &World, _model: Instance<T>, view: Instance<View<T>>, commands: &mut Commands);
}

/// Used to build a [`View`] [`Entity`] for a given [`Instance`] of [`Kind`] `T`.
///
/// See [`BuildView`] for more information.
pub struct ViewBuilder<'a, T: Kind>(InstanceCommands<'a, View<T>>);

#[allow(dead_code)]
impl<'a, T: Kind> ViewBuilder<'a, T> {
    /// Returns the [`View`] [`Instance`].
    pub fn instance(&self) -> Instance<View<T>> {
        self.0.instance()
    }

    /// Returns the [`View`] [`Entity`].
    pub fn entity(&self) -> Entity {
        self.0.entity()
    }

    /// Inserts a new [`Bundle`] into the [`View`] [`Entity`].
    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.0.insert(bundle);
        self
    }

    /// Adds some children to the [`View`] [`Entity`].
    pub fn insert_children<F: FnOnce(&mut ChildBuilder)>(&mut self, f: F) -> &mut Self {
        self.0.with_children(|view| f(view));
        self
    }

    /// Returns the [`InstanceCommands`] for the [`View`] [`Entity`].
    ///
    /// # Usage
    ///
    /// This is useful for more advanced operations on the view entity, or for modifying the world beyond
    /// the view entity hierarchy.
    ///
    /// These types of operations should be avoided as for most cases, building the view should be
    /// purely "additive" through adding components (see [`ViewBuilder::insert`]) or spawning
    /// children (see [`ViewBuilder::insert_children`]).
    pub fn commands(&mut self) -> &mut InstanceCommands<'a, View<T>> {
        &mut self.0
    }
}

/// [`Component`] of an [`Entity`] associated with a [`View`].
#[derive(Component)]
pub struct Model<T: Kind> {
    view: Instance<View<T>>,
}

impl<T: Kind> Model<T> {
    fn new(view: Instance<View<T>>) -> Self {
        Self { view }
    }
}

impl<T: Kind> Model<T> {
    /// Returns the [`View`] [`Instance`] associated with this [`Model`].
    pub fn view(&self) -> Instance<View<T>> {
        self.view
    }
}

/// [`Component`] of an [`Entity`] associated with a [`Model`].
#[derive(Component)]
pub struct View<T: Kind> {
    model: Instance<T>,
}

#[allow(dead_code)]
impl<T: Kind> View<T> {
    /// Returns the [`Model`] [`Instance`] associated with this [`View`].
    pub fn model(&self) -> Instance<T> {
        self.model
    }

    #[deprecated(note = "use `model` instead")]
    pub fn target(&self) -> Instance<T> {
        self.model()
    }
}

#[derive(Bundle)]
struct ViewBundle<T: Kind> {
    view: View<T>,
    unload: Unload,
}

impl<T: Kind> ViewBundle<T> {
    pub fn new(model: impl Into<Instance<T>>) -> Self {
        let model = model.into();
        Self {
            view: View { model },
            unload: Unload,
        }
    }
}

impl<T: Kind> KindBundle for ViewBundle<T> {
    type Kind = View<T>;
}

/// A [`Resource`] which contains a mapping of all viewable entities to their views.
///
/// # Usage
///
/// Typically, you want to access models or views using [`Model`] and [`View`] components.
/// However, in some cases it may be needed to access **all** views for a given model.
/// This [`Resource`] provides an interface for this specific purpose.
#[derive(Resource, Default)]
pub struct Viewables {
    models: HashMap<Entity, HashSet<Entity>>,
    kinds: HashMap<TypeId, HashSet<Entity>>,
}

#[allow(dead_code)]
impl Viewables {
    pub fn contains(&self, entity: Entity) -> bool {
        self.models.contains_key(&entity)
    }

    /// Iterates over all viewed [`Model`] entities.
    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.models.keys().copied()
    }

    pub fn is_viewable_kind<T: Kind>(&self) -> bool {
        self.kinds.contains_key(&TypeId::of::<T>())
    }

    /// Iterates over all views for a given [`Model`] [`Entity`].
    pub fn views(&self, entity: Entity) -> impl Iterator<Item = Entity> + '_ {
        self.models
            .get(&entity)
            .into_iter()
            .flat_map(|views| views.iter().copied())
    }

    fn add_kind<T: Kind>(&mut self) {
        self.kinds.insert(TypeId::of::<T>(), HashSet::default());
    }

    fn add<T: Kind>(&mut self, entity: Entity, view: Instance<View<T>>) {
        self.models.entry(entity).or_default().insert(view.entity());
        self.kinds
            .get_mut(&TypeId::of::<T>())
            .expect("kind must be registered as viewable")
            .insert(entity);
    }

    fn remove<T: Kind>(&mut self, entity: Entity, view: Instance<View<T>>) {
        let views = self.models.get_mut(&entity).unwrap();
        views.remove(&view.entity());
        if views.is_empty() {
            self.models.remove(&entity);
        }
        let kinds = self.kinds.get_mut(&TypeId::of::<T>()).unwrap();
        kinds.remove(&view.entity());
    }
}

fn spawn<T: Kind, S: BuildView<T>>(
    objects: Objects<T, (Without<Model<T>>, S::Filter)>,
    world: &World,
    mut commands: Commands,
) {
    for object in objects.iter() {
        let view = commands.spawn_instance(ViewBundle::new(object));
        let mut view = ViewBuilder(view);
        S::build(world, object, &mut view);
        let view = view.instance();
        let entity = object.entity();
        commands.add(move |world: &mut World| {
            world.resource_mut::<Viewables>().add(entity, view);
        });
        commands.entity(entity).insert(Model::new(view));
        warn!("{view:?} spawned for {entity:?}");
    }
}

fn despawn<T: Kind, S: BuildView<T>>(
    world: &World,
    views: Query<InstanceRef<View<T>>>,
    query: Query<(), T::Filter>,
    mut commands: Commands,
) {
    for view in views.iter() {
        let model = view.model();
        let view = view.instance();
        if query.get(model.entity()).is_err() {
            S::on_before_destroy(world, model, view, &mut commands);
            if let Some(mut entity) = commands.get_entity(model.entity()) {
                entity.remove::<Model<T>>();
            }
            commands.entity(view.entity()).despawn_recursive();
            commands.add(move |world: &mut World| {
                world
                    .resource_mut::<Viewables>()
                    .remove(model.entity(), view);
            });
            warn!("{view:?} despawned for {model:?}");
        }
    }
}

/// Despawns the current [`View`] associated with this [`Model`] and rebuilds a new one.
///
/// # Example
/// ```
/// # use bevy::prelude::*;
/// # use moonshine_core::prelude::*;
/// # use moonshine_view::prelude::*;
///
/// #[derive(Component)]
/// enum Shape {
///     Square,
///     Circle,
/// }
///
/// impl BuildView for Shape {
///     fn build(world: &World, object: Object<Self>, view: &mut ViewBuilder<Self>) {
///         let shape = world.get::<Shape>(object.entity());
///         // ...
///     }
/// }
///
/// fn rebuild_shape_views(query: Query<InstanceRef<Model<Shape>>>, mut commands: Commands) {
///     for model in query.iter() {
///         moonshine_view::rebuild(model, &mut commands);
///     }
/// }
/// ```
#[allow(dead_code)]
pub fn rebuild<T: Kind>(model: InstanceRef<Model<T>>, commands: &mut Commands) {
    let entity = model.entity();
    let view = model.view();
    commands.entity(view.entity()).despawn_recursive();
    commands.add(move |world: &mut World| {
        world.resource_mut::<Viewables>().remove(entity, view);
    });
    commands.entity(entity).remove::<Model<T>>();
}
