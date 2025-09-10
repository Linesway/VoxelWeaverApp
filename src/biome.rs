
use std::{collections::HashMap, path::PathBuf};

use serde_json::json;

pub const BIOME_DEPTH_LIMIT: i32 = 99999;

pub struct Biome {
    pub name: String,
    pub frequency: f32,
    pub texture: PathBuf,
    pub color: [f32; 3],
    pub min_depth: i32,
    pub max_depth: i32,
    pub params: HashMap<String, f32>,
}

impl Biome {

    pub fn frequency(&self) -> f32 {
        self.frequency.max(0.01)
    }

}

pub struct Biomes {
    pub biome_params: Vec<String>,
    pub biomes: Vec<Biome>,
    pub biome_size: f32,
    pub biome_blending: f32
}

impl Biomes {

    pub fn new() -> Self {
        Self {
            biome_params: vec![],
            biomes: vec![Biome {
                name: "Biome".to_owned(),
                frequency: 1.0,
                texture: PathBuf::new(),
                params: HashMap::new(),
                min_depth: -BIOME_DEPTH_LIMIT,
                max_depth: BIOME_DEPTH_LIMIT,
                color: [1.0, 0.0, 0.0]
            }],
            biome_size: 500.0,
            biome_blending: 0.25
        }
    }

    pub fn add_parameter(&mut self, name: String) {
        if name.is_empty() {
            return;
        }
        self.biome_params.push(name);
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "params": self.biome_params,
            "size": self.biome_size,
            "blending": self.biome_blending,
            "biomes": serde_json::Value::Array(self.biomes.iter().map(|biome| json!({
                "name": biome.name,
                "frequency": biome.frequency,
                "params": biome.params,
                "color": biome.color,
                "texture": biome.texture,
                "min_depth": biome.min_depth,
                "max_depth": biome.max_depth,
            })).collect())
        })
    }

    pub fn from_json(data: &serde_json::Value) -> Option<Self> {
        let data = data.as_object()?;
        let params: Vec<_> = data.get("params")?.as_array()?.iter().filter_map(|param| param.as_str()).map(|param| param.to_owned()).collect();
        let biome_size = data.get("size").map(|val| val.as_f64()).flatten().unwrap_or(500.0) as f32;
        let biome_blending = data.get("blending").map(|val| val.as_f64()).flatten().unwrap_or(0.25) as f32;
        
        let mut biomes = vec![];
        for biome_data in data.get("biomes")?.as_array()? {
            let biome_data = biome_data.as_object()?;
            let biome_params = biome_data.get("params")?.as_object()?;
            let mut color = [1.0, 0.0, 0.0];
            if let Some(color_data) = biome_data.get("color") {
                let color_data = color_data.as_array()?;
                color[0] = color_data.get(0)?.as_f64()? as f32;
                color[1] = color_data.get(1)?.as_f64()? as f32;
                color[2] = color_data.get(2)?.as_f64()? as f32;
            }
            let texture = biome_data.get("texture").map(|texture| texture.as_str()).flatten().map(|str| str.into()).unwrap_or(PathBuf::new());
            biomes.push(Biome {
                name: biome_data.get("name")?.as_str()?.to_owned(),
                frequency: biome_data.get("frequency").map(|freq| freq.as_f64()).flatten().unwrap_or(1.0) as f32,
                color, 
                texture,
                min_depth: biome_data.get("min_depth").map(|min_depth| min_depth.as_i64()).flatten().unwrap_or(-BIOME_DEPTH_LIMIT as i64) as i32, 
                max_depth: biome_data.get("max_depth").map(|max_depth| max_depth.as_i64()).flatten().unwrap_or(BIOME_DEPTH_LIMIT as i64) as i32,
                params: params.iter().map(|param| (
                    param.clone(), 
                    biome_params.get(param).map(|val| val.as_f64()).flatten().unwrap_or(0.0) as f32
                )).collect(),
            });
        }

        Some(Self {
            biome_params: params,
            biomes,
            biome_size,
            biome_blending
        })
    }

}
