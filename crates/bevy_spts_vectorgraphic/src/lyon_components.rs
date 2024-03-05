use bevy_ecs::component::Component;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum LineCap {
    /// The stroke for each sub-path does not extend beyond its two endpoints.
    /// A zero length sub-path will therefore not have any stroke.
    Butt,
    /// At the end of each sub-path, the shape representing the stroke will be
    /// extended by a rectangle with the same width as the stroke width and
    /// whose length is half of the stroke width. If a sub-path has zero length,
    /// then the resulting effect is that the stroke for that sub-path consists
    /// solely of a square with side length equal to the stroke width, centered
    /// at the sub-path's point.
    Square,
    /// At each end of each sub-path, the shape representing the stroke will be extended
    /// by a half circle with a radius equal to the stroke width.
    /// If a sub-path has zero length, then the resulting effect is that the stroke for
    /// that sub-path consists solely of a full circle centered at the sub-path's point.
    Round,
}

impl From<LineCap> for lyon_path::LineCap {
    fn from(value: LineCap) -> Self {
        match value {
            LineCap::Round => Self::Round,
            LineCap::Butt => Self::Butt,
            LineCap::Square => Self::Square,
        }
    }
}

/// Line join as defined by the SVG specification.
///
/// See: <https://svgwg.org/specs/strokes/#StrokeLinejoinProperty>
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum LineJoin {
    /// A sharp corner is to be used to join path segments.
    Miter,
    /// Same as a miter join, but if the miter limit is exceeded,
    /// the miter is clipped at a miter length equal to the miter limit value
    /// multiplied by the stroke width.
    MiterClip,
    /// A round corner is to be used to join path segments.
    Round,
    /// A beveled corner is to be used to join path segments.
    /// The bevel shape is a triangle that fills the area between the two stroked
    /// segments.
    Bevel,
}

impl From<LineJoin> for lyon_path::LineJoin {
    fn from(value: LineJoin) -> Self {
        match value {
            LineJoin::Miter => Self::Miter,
            LineJoin::MiterClip => Self::MiterClip,
            LineJoin::Round => Self::Round,
            LineJoin::Bevel => Self::Bevel,
        }
    }
}

/// Vertical or Horizontal.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl From<Orientation> for lyon_tessellation::Orientation {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Vertical => Self::Vertical,
            Orientation::Horizontal => Self::Horizontal,
        }
    }
}

/// An alias for `usize`.
pub type AttributeIndex = usize;

/// Parameters for the tessellator.
#[derive(Copy, Clone, Debug, PartialEq, Component)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct StrokeOptions {
    /// What cap to use at the start of each sub-path.
    ///
    /// Default value: `LineCap::Butt`.
    pub start_cap: LineCap,

    /// What cap to use at the end of each sub-path.
    ///
    /// Default value: `LineCap::Butt`.
    pub end_cap: LineCap,

    /// See the SVG specification.
    ///
    /// Default value: `LineJoin::Miter`.
    pub line_join: LineJoin,

    /// Line width
    ///
    /// Default value: `StrokeOptions::DEFAULT_LINE_WIDTH`.
    pub line_width: f32,

    /// Index of a custom attribute defining a per-vertex
    /// factor to modulate the line width.
    ///
    /// Default value: `None`.
    pub variable_line_width: Option<AttributeIndex>,

    /// See the SVG specification.
    ///
    /// Must be greater than or equal to 1.0.
    /// Default value: `StrokeOptions::DEFAULT_MITER_LIMIT`.
    pub miter_limit: f32,

    /// Maximum allowed distance to the path when building an approximation.
    ///
    /// See [Flattening and tolerance](index.html#flattening-and-tolerance).
    /// Default value: `StrokeOptions::DEFAULT_TOLERANCE`.
    pub tolerance: f32,
}

impl From<StrokeOptions> for lyon_tessellation::StrokeOptions {
    fn from(value: StrokeOptions) -> Self {
        let lyon = Self::default()
            .with_start_cap(value.start_cap.into())
            .with_end_cap(value.end_cap.into())
            .with_line_join(value.line_join.into())
            .with_line_width(value.line_width)
            .with_miter_limit(value.miter_limit)
            .with_tolerance(value.tolerance);

        if let Some(idx) = value.variable_line_width {
            lyon.with_variable_line_width(idx)
        } else {
            lyon
        }
    }
}

impl StrokeOptions {
    /// Minimum miter limit as defined by the SVG specification.
    ///
    /// See [StrokeMiterLimitProperty](https://svgwg.org/specs/strokes/#StrokeMiterlimitProperty)
    pub const MINIMUM_MITER_LIMIT: f32 = 1.0;
    /// Default miter limit as defined by the SVG specification.
    ///
    /// See [StrokeMiterLimitProperty](https://svgwg.org/specs/strokes/#StrokeMiterlimitProperty)
    pub const DEFAULT_MITER_LIMIT: f32 = 4.0;
    pub const DEFAULT_LINE_CAP: LineCap = LineCap::Butt;
    pub const DEFAULT_LINE_JOIN: LineJoin = LineJoin::Miter;
    pub const DEFAULT_LINE_WIDTH: f32 = 1.0;
    pub const DEFAULT_TOLERANCE: f32 = 0.1;

    pub const DEFAULT: Self = StrokeOptions {
        start_cap: Self::DEFAULT_LINE_CAP,
        end_cap: Self::DEFAULT_LINE_CAP,
        line_join: Self::DEFAULT_LINE_JOIN,
        line_width: Self::DEFAULT_LINE_WIDTH,
        variable_line_width: None,
        miter_limit: Self::DEFAULT_MITER_LIMIT,
        tolerance: Self::DEFAULT_TOLERANCE,
    };

    #[inline]
    pub fn tolerance(tolerance: f32) -> Self {
        Self::DEFAULT.with_tolerance(tolerance)
    }

    #[inline]
    pub const fn with_tolerance(mut self, tolerance: f32) -> Self {
        self.tolerance = tolerance;
        self
    }

    #[inline]
    pub const fn with_line_cap(mut self, cap: LineCap) -> Self {
        self.start_cap = cap;
        self.end_cap = cap;
        self
    }

    #[inline]
    pub const fn with_start_cap(mut self, cap: LineCap) -> Self {
        self.start_cap = cap;
        self
    }

    #[inline]
    pub const fn with_end_cap(mut self, cap: LineCap) -> Self {
        self.end_cap = cap;
        self
    }

    #[inline]
    pub const fn with_line_join(mut self, join: LineJoin) -> Self {
        self.line_join = join;
        self
    }

    #[inline]
    pub const fn with_line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    #[inline]
    pub fn with_miter_limit(mut self, limit: f32) -> Self {
        assert!(limit >= Self::MINIMUM_MITER_LIMIT);
        self.miter_limit = limit;
        self
    }

    #[inline]
    pub const fn with_variable_line_width(mut self, idx: AttributeIndex) -> Self {
        self.variable_line_width = Some(idx);
        self
    }
}

impl Default for StrokeOptions {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The fill rule defines how to determine what is inside and what is outside of the shape.
///
/// See the SVG specification.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum FillRule {
    EvenOdd,
    NonZero,
}

impl From<FillRule> for lyon_tessellation::FillRule {
    fn from(value: FillRule) -> Self {
        match value {
            FillRule::EvenOdd => Self::EvenOdd,
            FillRule::NonZero => Self::NonZero,
        }
    }
}


/// Parameters for the fill tessellator.
#[derive(Copy, Clone, Debug, PartialEq, Component)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct FillOptions {
    /// Maximum allowed distance to the path when building an approximation.
    ///
    /// See [Flattening and tolerance](index.html#flattening-and-tolerance).
    ///
    /// Default value: `FillOptions::DEFAULT_TOLERANCE`.
    pub tolerance: f32,

    /// Set the fill rule.
    ///
    /// See the [SVG specification](https://www.w3.org/TR/SVG/painting.html#FillRuleProperty).
    ///
    /// Default value: `EvenOdd`.
    pub fill_rule: FillRule,

    /// Whether to perform a vertical or horizontal traversal of the geometry.
    ///
    /// Default value: `Vertical`.
    pub sweep_orientation: Orientation,

    /// A fast path to avoid some expensive operations if the path is known to
    /// not have any self-intersections.
    ///
    /// Do not set this to `false` if the path may have intersecting edges else
    /// the tessellator may panic or produce incorrect results. In doubt, do not
    /// change the default value.
    ///
    /// Default value: `true`.
    pub handle_intersections: bool,
}

impl From<FillOptions> for lyon_tessellation::FillOptions {
    fn from(value: FillOptions) -> Self {
        Self::default()
            .with_tolerance(value.tolerance)
            .with_fill_rule(value.fill_rule.into())
            .with_sweep_orientation(value.sweep_orientation.into())
            .with_intersections(value.handle_intersections)
    }
}

impl FillOptions {
    /// Default flattening tolerance.
    pub const DEFAULT_TOLERANCE: f32 = 0.1;
    /// Default Fill rule.
    pub const DEFAULT_FILL_RULE: FillRule = FillRule::EvenOdd;
    /// Default orientation.
    pub const DEFAULT_SWEEP_ORIENTATION: Orientation = Orientation::Vertical;

    pub const DEFAULT: Self = FillOptions {
        tolerance: Self::DEFAULT_TOLERANCE,
        fill_rule: Self::DEFAULT_FILL_RULE,
        sweep_orientation: Self::DEFAULT_SWEEP_ORIENTATION,
        handle_intersections: true,
    };

    #[inline]
    pub fn even_odd() -> Self {
        Self::DEFAULT
    }

    #[inline]
    pub fn tolerance(tolerance: f32) -> Self {
        Self::DEFAULT.with_tolerance(tolerance)
    }

    #[inline]
    pub fn non_zero() -> Self {
        let mut options = Self::DEFAULT;
        options.fill_rule = FillRule::NonZero;
        options
    }

    #[inline]
    pub const fn with_tolerance(mut self, tolerance: f32) -> Self {
        self.tolerance = tolerance;
        self
    }

    #[inline]
    pub const fn with_fill_rule(mut self, rule: FillRule) -> Self {
        self.fill_rule = rule;
        self
    }

    #[inline]
    pub const fn with_sweep_orientation(mut self, orientation: Orientation) -> Self {
        self.sweep_orientation = orientation;
        self
    }

    #[inline]
    pub const fn with_intersections(mut self, intersections: bool) -> Self {
        self.handle_intersections = intersections;
        self
    }
}

impl Default for FillOptions {
    fn default() -> Self {
        Self::DEFAULT
    }
}
