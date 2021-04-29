#[derive(Debug, serde::Deserialize)]
pub struct PlantConfig {
    pub name: String,
    pub soil: RangeConfig,
    pub temperature: RangeConfig,
    pub hygro: RangeConfig<f32>,
    pub pump: PumpConfig,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct RangeConfig<N = u32> {
    pub min: N,
    pub max: N,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub struct PumpConfig {
    pub duration: u32,
    pub wait: u32,
}
