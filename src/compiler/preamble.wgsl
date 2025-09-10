
fn sdf(pos: vec3<f32>) -> TerrainOutput {
    var terrain_out: TerrainOutput;
    terrain_out.terrain.sdf = 1.0;
    let seed = 666;