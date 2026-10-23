use ch_types::EvidencePoint;
pub const DIMENSION: usize = 8;
pub fn embed(points: &[EvidencePoint], target: f64) -> Vec<f32> {
    let values: Vec<f64> = points.iter().map(|p| p.observed).collect();
    let mean = values.iter().sum::<f64>() / values.len().max(1) as f64;
    let min = values.iter().copied().fold(1.0, f64::min);
    let max = values.iter().copied().fold(0.0, f64::max);
    let last = values.last().copied().unwrap_or_default();
    let first = values.first().copied().unwrap_or_default();
    vec![
        mean as f32,
        min as f32,
        max as f32,
        last as f32,
        (last - first) as f32,
        target as f32,
        points.len() as f32 / 512.0,
        (mean - target) as f32,
    ]
}
