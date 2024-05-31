use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

use bevy::{
    ecs::{
        schedule::{NodeId, Schedule},
        world::World,
    },
    utils::{
        hashbrown::HashMap,
        petgraph::{graphmap::GraphMap, Directed, Direction},
        smallvec::SmallVec,
    },
};

#[allow(non_snake_case)]
pub mod definitions {
    use serde::{Deserialize, Serialize};
    use tsify::Tsify;
    use wasm_bindgen::prelude::*;

    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub enum ScheduleLabel {
        First,
        PreUpdate,
        Update,
        PostUpdate,
        Last,
    }

    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub enum ScheduleGraphDirection {
        TopBottom,
        LeftRight,
    }

    #[derive(Tsify, Serialize, Deserialize)]
    #[tsify(into_wasm_abi, from_wasm_abi)]
    pub struct ScheduleGraphSettings {
        pub graph_direction: ScheduleGraphDirection,
        pub collapse_single_system_sets: bool,
        pub ambiguity_enable: bool,
        pub ambiguity_enable_on_world: bool,
        pub prettify_system_names: bool,
    }
}

impl Display for definitions::ScheduleGraphDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            definitions::ScheduleGraphDirection::TopBottom => write!(f, "TB"),
            definitions::ScheduleGraphDirection::LeftRight => write!(f, "LR"),
        }
    }
}

#[allow(dead_code)]
pub struct ScheduleGrapher<'a> {
    schedule: &'a Schedule,
    world: &'a World,
    settings: &'a definitions::ScheduleGraphSettings,

    node_id_remap: HashMap<NodeId, NodeId>,

    top_hierarchy: SmallVec<[GraphNode; 4]>,
    top_dependency: SmallVec<[GraphNode; 4]>,
    nodes: HashMap<NodeId, GraphNode>,
}

impl<'a> ScheduleGrapher<'a> {
    pub fn new(
        schedule: &'a Schedule,
        world: &'a World,
        settings: &'a definitions::ScheduleGraphSettings,
    ) -> Self {
        let graph = schedule.graph();
        let hierarchy = graph.hierarchy();
        let hierarchy_graph = hierarchy.graph();
        let dependency = graph.dependency();

        let mut node_id_remap: HashMap<NodeId, NodeId> = HashMap::new();

        // Build remap from NodeID -> NodeId to remove SystemTypeSets from the graph.
        if settings.collapse_single_system_sets {
            for (id, sys_set, _) in graph.system_sets() {
                if sys_set.system_type().is_some() {
                    let mut children = hierarchy_graph
                        .neighbors_directed(id, bevy::utils::petgraph::Direction::Outgoing);
                    if let Some(system_node_id) = children.next() {
                        node_id_remap.insert(id, system_node_id);
                    }
                }
            }
        }

        let mut graph_map: HashMap<NodeId, GraphNode> = HashMap::new();

        for (id, _, _) in graph.system_sets() {
            if node_id_remap.get(&id).is_some() {
                continue;
            }
            let entry = graph_map.entry(id);
            entry.insert(GraphNode(id));
        }

        for (id, _) in schedule.systems().unwrap() {
            let entry = graph_map.entry(id);
            entry.insert(GraphNode(id));
        }

        let top_hierarchy: SmallVec<[GraphNode; 4]> = hierarchy
            .cached_topsort()
            .iter()
            .map(|id| {
                let id = *node_id_remap.get(id).unwrap_or(id);
                GraphNode(id)
            })
            .collect();

        let top_dependency: SmallVec<[GraphNode; 4]> = hierarchy
            .cached_topsort()
            .iter()
            .map(|id| {
                let id = *node_id_remap.get(id).unwrap_or(id);
                GraphNode(id)
            })
            .collect();

        Self {
            schedule,
            world,
            settings,
            node_id_remap,
            top_hierarchy,
            top_dependency,
            nodes: graph_map,
        }
    }
}

impl<'a> Display for ScheduleGrapher<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "graph {}", self.settings.graph_direction)?;

        for top in self.top_hierarchy.iter() {
            top.format_definition(self, f)?;
        }

        for node in self.nodes.values() {
            node.format_relations(self, f)?;
        }

        Ok(())
    }
}

pub struct GraphNode(NodeId);

impl From<NodeId> for GraphNode {
    fn from(value: NodeId) -> Self {
        Self(value)
    }
}

impl GraphNode {
    /// Gets the neighbours of a graph with a given direction but remaps the NodeIds against node_id_remap
    fn neighbors_directed<'s>(
        &'s self,
        cx: &'s ScheduleGrapher,
        graph_map: &'s GraphMap<NodeId, (), Directed>,
        direction: Direction,
    ) -> impl Iterator<Item = GraphNode> + 's {
        graph_map
            .neighbors_directed(self.0, direction)
            .map(|n| GraphNode(*cx.node_id_remap.get(&n).unwrap_or(&n)))
    }

    pub fn hierarchy_children<'s>(
        &'s self,
        cx: &'s ScheduleGrapher,
    ) -> impl Iterator<Item = GraphNode> + 's {
        use bevy::utils::petgraph::Direction::Outgoing;

        self.neighbors_directed(cx, cx.schedule.graph().hierarchy().graph(), Outgoing)
    }

    pub fn dependency_incoming<'s>(
        &'s self,
        cx: &'s ScheduleGrapher,
    ) -> impl Iterator<Item = GraphNode> + 's {
        use bevy::utils::petgraph::Direction::Incoming;

        self.neighbors_directed(cx, cx.schedule.graph().dependency().graph(), Incoming)
    }

    pub fn dependency_outgoing<'s>(
        &'s self,
        cx: &'s ScheduleGrapher,
    ) -> impl Iterator<Item = GraphNode> + 's {
        use bevy::utils::petgraph::Direction::Outgoing;

        self.neighbors_directed(cx, cx.schedule.graph().dependency().graph(), Outgoing)
    }

    pub fn graph_id(&self) -> Cow<str> {
        match self.0 {
            NodeId::Set(id) => format!("set_{id}").into(),
            NodeId::System(id) => format!("system_{id}").into(),
        }
    }

    pub fn graph_fullname(&self, cx: &ScheduleGrapher) -> Cow<str> {
        match self.0 {
            NodeId::System(_) => {
                if let Ok(mut systems) = cx.schedule.systems() {
                    if let Some((_, sys)) = systems.find(|(n, _)| *n == self.0) {
                        return sys.name();
                    }
                }
                std::borrow::Cow::Borrowed("unknown_system")
            }
            NodeId::Set(_) => format!("{:?}", cx.schedule.graph().set_at(self.0)).into(),
        }
    }

    pub fn graph_name(&self, cx: &ScheduleGrapher) -> Cow<str> {
        let name = self.graph_fullname(cx);

        if cx.settings.prettify_system_names {
            escape_html_punctuation(&pretty_type_name::pretty_type_name_str(&name)).into()
        } else {
            escape_html_punctuation(&name).into()
        }
    }

    pub fn format_definition(&self, cx: &ScheduleGrapher, f: &mut Formatter) -> std::fmt::Result {
        match self.0 {
            NodeId::Set(_) => {
                writeln!(f, "subgraph {}[\"{}\"]", self.graph_id(), self.graph_name(cx))?;
                writeln!(f, "direction {}", cx.settings.graph_direction)?;
                for child in self.hierarchy_children(cx) {
                    child.format_definition(cx, f)?;
                }
                writeln!(f, "end")?;
                Ok(())
            }
            NodeId::System(_) => {
                writeln!(f, "{}[\"{}\"]", self.graph_id(), self.graph_name(cx))
            }
        }
    }

    pub fn format_relations(&self, cx: &ScheduleGrapher, f: &mut Formatter) -> std::fmt::Result {
        for incoming in self.dependency_incoming(cx) {
            writeln!(f, "{} --> {}", incoming.graph_id(), self.graph_id())?;
        }
        for outgoing in self.dependency_outgoing(cx) {
            writeln!(f, "{} --> {}", self.graph_id(), outgoing.graph_id())?;
        }
        Ok(())
    }
}

fn escape_html_punctuation(input: &str) -> String {
    let mut result = String::new();
    
    for ch in input.chars() {
        match ch {
            ':' => result.push_str("&#58;"),
            '\\' => result.push_str("&#92;"),
            '/' => result.push_str("&#47;"),
            '<' => result.push_str("&#60;"),
            '>' => result.push_str("&#62;"),
            '[' => result.push_str("&#91;"),
            ']' => result.push_str("&#93;"),
            '(' => result.push_str("&#40;"),
            ')' => result.push_str("&#41;"),
            _ => result.push(ch),
        }
    }
    
    result
}
