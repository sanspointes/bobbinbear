use bevy::{prelude::*, math::Vec3Swizzles};
#[derive(Component, Default)]
pub struct DocumentTag;

#[derive(Component, Default)]
pub struct ActiveDocumentTag;

#[derive(Component, Default, Debug, Clone, Reflect)]
pub enum SelectableTag {
    #[default]
    Default,
    Locked,
}

#[derive(Component, Default)]
pub struct MovableTag;
#[derive(Component, Default)]
pub struct ResizableTag;

#[derive(Component, Default, Debug, Reflect)]
pub enum Bounded {
    #[default]
    NeedsCalculate,
    Calculated { min: Vec2, max: Vec2 },
}

impl Bounded {
    pub fn from_vec3s(vec3s: &Vec<Vec3>) -> Self {
        let mut min = Vec2::new(f32::MAX, f32::MAX);
        let mut max = Vec2::new(f32::MIN, f32::MIN);
        for v in vec3s {
            min = min.min(v.xy());
            max = max.max(v.xy());
        }
        Self::Calculated { min, max }
    }

    pub fn from_vertices(vertices: &Vec<[f32; 3]>) -> Self {
        let mut min_x = f32::MAX; 
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN; 
        let mut max_y = f32::MIN;
        for [x, y, _] in vertices {
            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }
        Self::Calculated { min: Vec2::new(min_x, min_y), max: Vec2::new(max_x, max_y) }
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }
    pub fn width(&self) -> f32 {
        match self {
            Self::NeedsCalculate => 0.,
            Self::Calculated { min, max } => max.x - min.x,
        }
    }
    pub fn height(&self) -> f32 {
        match self {
            Self::NeedsCalculate => 0.,
            Self::Calculated { min, max } => max.y - min.y,
        }
    }

    pub fn min(&self) -> Option<&Vec2> {
        match self {
            Self::NeedsCalculate => None,
            Self::Calculated { min, max } => Some(&min),
        }
    }
    pub fn max(&self) -> Option<&Vec2> {
        match self {
            Self::NeedsCalculate => None,
            Self::Calculated { min, max } => Some(&max),
        }
    }
}

#[derive(Component, Default, Debug, Clone, PartialEq, Reflect)]
pub enum HoveredState {
    #[default]
    Unhovered,
    Hovered,
}
#[derive(Component, Default, Debug, Clone, PartialEq, Reflect)]
pub enum SelectedState {
    #[default]
    Unselected,
    Selected,
}

#[derive(Component, Default)]
pub struct NeedsBoundsUpdate;

#[derive(Component, Default)]
pub struct NeedsDelete;
