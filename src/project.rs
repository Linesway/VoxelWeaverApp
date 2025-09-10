
use serde_json::json;

use crate::{biome::Biomes, graph::TerrainGraph};

pub struct Project {
    pub terrain_graph: TerrainGraph,
    pub biomes: Biomes
}

impl Project {

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "graph": self.terrain_graph.to_json(),
            "biomes": self.biomes.to_json()
        })
    }

    pub fn load_from_json(&mut self, data: serde_json::Value) {
        let Some(data) = data.as_object() else { return; };

        let Some(graph) = data.get("graph") else { return; };
        let Some(graph) = TerrainGraph::from_json(graph) else { return; };

        let biomes = if let Some(biome_data) = data.get("biomes") {
            Biomes::from_json(biome_data).unwrap_or(Biomes::new())
        } else {
            Biomes::new()
        };

        self.terrain_graph = graph;
        self.biomes = biomes;
    }

}
