
use super::{NodeType, NodeTypeDyn};

pub mod math;
use math::*;

pub mod terrain;
use terrain::*;

pub mod heightmap;
use heightmap::*;

mod biome;
use biome::*;

pub mod vector;
use vector::*;

pub mod trig;
use trig::*;

pub mod noise;
use noise::*;

pub struct NodeKind {
    pub label: &'static str,
    pub make: fn() -> Box<dyn NodeTypeDyn> 
}

const fn make_node_kind<T: NodeType + 'static>() -> NodeKind {
    NodeKind {
        label: T::LABEL,
        make: || Box::new(T::make()) 
    }
}

pub const NODE_TYPES: &[(&'static str, &[NodeKind])] = &[
    ("Terrain", &[
        make_node_kind::<TerrainOutput>(),
        make_node_kind::<HeightmapTerrain>(),
        make_node_kind::<FractalNoiseTerrain>(),
        make_node_kind::<BlobCaveTerrain>(),
        make_node_kind::<SpaghettiCaveTerrain>(),
        make_node_kind::<InvertTerrain>(),
        make_node_kind::<ErodeTerrain>(),
        make_node_kind::<TerrainUnion>(),
        make_node_kind::<TerrainIntersection>(),
        make_node_kind::<TerrainToSDF>(),
        make_node_kind::<SDFToTerrain>(),
    ]),
    ("Heightmap", &[
        make_node_kind::<NoiseHeightmap>(),
        make_node_kind::<RidgeHeightmap>()
    ]),
    ("Math", &[
        make_node_kind::<Add>(),
        make_node_kind::<Subtract>(),
        make_node_kind::<Multiply>(),
        make_node_kind::<Divide>(),
        make_node_kind::<Power>(),
        make_node_kind::<Log>(),
        make_node_kind::<Min>(),
        make_node_kind::<Max>(),
        make_node_kind::<GreaterThan>(),
        make_node_kind::<LessThan>(),
        make_node_kind::<Equal>(),
        make_node_kind::<Floor>(),
        make_node_kind::<Round>(),
        make_node_kind::<Ceil>(),
        make_node_kind::<Frac>(),
        make_node_kind::<Abs>(),
        make_node_kind::<Sign>(),
        make_node_kind::<Clamp>(),
        make_node_kind::<Lerp>(),
        make_node_kind::<MapRange>(),
    ]),
    ("Vector Math", &[
        make_node_kind::<CombineXYZ>(),
        make_node_kind::<SeparateXYZ>(),
        make_node_kind::<VectorAdd>(),
        make_node_kind::<VectorSubtract>(),
        make_node_kind::<VectorScale>(),
        make_node_kind::<VectorMultiply>(),
        make_node_kind::<DotProduct>(),
        make_node_kind::<CrossProduct>(),
        make_node_kind::<Length>(),
        make_node_kind::<Distance>(),
        make_node_kind::<Normalize>(),
        make_node_kind::<Position>(),
    ]),
    ("Noise", &[
        make_node_kind::<Noise3D>(),
        make_node_kind::<Noise2D>(),
    ]),
    ("Trig", &[
        make_node_kind::<Sin>(),
        make_node_kind::<Cos>(),
        make_node_kind::<Tan>(),
        make_node_kind::<Asin>(),
        make_node_kind::<Acos>(),
        make_node_kind::<Atan>(),
    ]),
    ("Biome", &[
        make_node_kind::<BiomeParameter>(),
        make_node_kind::<BiomeWeight>()
    ])
];
