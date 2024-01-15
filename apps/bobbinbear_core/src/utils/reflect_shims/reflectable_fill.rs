use bevy::{
    ecs::reflect::ReflectComponent,
    prelude::{Color, Component},
    reflect::Reflect,
};
use bevy_prototype_lyon::prelude::{Fill, FillOptions, FillRule, Orientation};

#[derive(Reflect, Debug, Clone, Copy, Default)]
pub enum ReflectableOrientation {
    #[default]
    Vertical,
    Horizontal,
}
impl From<Orientation> for ReflectableOrientation {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Vertical => ReflectableOrientation::Vertical,
            Orientation::Horizontal => ReflectableOrientation::Horizontal,
        }
    }
}
impl From<ReflectableOrientation> for Orientation {
    fn from(value: ReflectableOrientation) -> Self {
        match value {
            ReflectableOrientation::Vertical => Orientation::Vertical,
            ReflectableOrientation::Horizontal => Orientation::Horizontal,
        }
    }
}

#[derive(Reflect, Debug, Clone, Copy, Default)]
pub enum ReflectableFillRule {
    #[default]
    EvenOdd,
    NonZero,
}
impl From<FillRule> for ReflectableFillRule {
    fn from(value: FillRule) -> Self {
        match value {
            FillRule::EvenOdd => ReflectableFillRule::EvenOdd,
            FillRule::NonZero => ReflectableFillRule::NonZero,
        }
    }
}
impl From<ReflectableFillRule> for FillRule {
    fn from(value: ReflectableFillRule) -> Self {
        match value {
            ReflectableFillRule::EvenOdd => FillRule::EvenOdd,
            ReflectableFillRule::NonZero => FillRule::NonZero,
        }
    }
}

#[derive(Reflect, Debug, Clone, Copy, Default)]
pub struct ReflectableFillOptions {
    tolerance: f32,
    fill_rule: ReflectableFillRule,
    sweep_orientation: ReflectableOrientation,
    handle_intersections: bool,
}
impl From<FillOptions> for ReflectableFillOptions {
    fn from(value: FillOptions) -> Self {
        Self {
            tolerance: value.tolerance,
            fill_rule: value.fill_rule.into(),
            sweep_orientation: value.sweep_orientation.into(),
            handle_intersections: value.handle_intersections,
        }
    }
}
impl From<ReflectableFillOptions> for FillOptions {
    fn from(value: ReflectableFillOptions) -> Self {
        Self::default()
            .with_tolerance(value.tolerance)
            .with_fill_rule(value.fill_rule.into())
            .with_sweep_orientation(value.sweep_orientation.into())
            .with_intersections(value.handle_intersections)
    }
}

#[derive(Component, Reflect, Default, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct ReflectableFill {
    color: Color,
    options: ReflectableFillOptions,
}

impl From<Fill> for ReflectableFill {
    fn from(value: Fill) -> Self {
        Self {
            color: value.color,
            options: value.options.into(),
        }
    }
}
impl From<ReflectableFill> for Fill {
    fn from(value: ReflectableFill) -> Self {
        Self {
            color: value.color,
            options: value.options.into(),
        }
    }
}
