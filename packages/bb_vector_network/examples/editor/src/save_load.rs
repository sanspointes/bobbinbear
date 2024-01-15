use bb_vector_network::prelude::*;

#[derive(Default)]
pub struct SaveLoad {
    saved: Option<String>,
}

impl SaveLoad {

    pub fn save(&mut self, graph: &BBGraph) {
        match serde_json::to_string(graph) {
            Ok(result) => {
                self.saved = Some(result);
            }
            Err(reason) => {
                println!("SaveLoad: Save error: {reason:?}");
            }
        }
    }
    pub fn try_load(&self) -> Result<BBGraph, String> {
        let Some(ref saved) = self.saved else {
            return Err("No saved graph.".to_string());
        };

        serde_json::from_str(&saved).map_err(|reason| format!("Error: {reason:?}").to_string())
    }
}
