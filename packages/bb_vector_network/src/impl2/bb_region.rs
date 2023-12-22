use crate::BBEdgeIndex;

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct BBCycle {
    pub edges: Vec<BBEdgeIndex>,
    pub children: Vec<BBCycle>,
}

impl BBCycle {
    pub fn new(edges: Vec<BBEdgeIndex>) -> Self {
        Self {
            edges,
            children: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum BBWindingRule {
    Default,
    NonZero,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct BBRegion {
    winding_rule: BBWindingRule,
    pub cycles: Vec<BBCycle>
}

impl BBRegion {
    pub fn new(cycles: Vec<BBCycle>) -> Self {
        Self {
            winding_rule: BBWindingRule::Default,
            cycles,
        }
    }
}
