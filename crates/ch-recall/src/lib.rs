use ch_errors::Result;
use ch_ruvector::AnalogueIndex;
use ch_types::Analogue;
pub fn recall(index: &AnalogueIndex, vector: &[f32]) -> Result<Vec<Analogue>> {
    index.search(vector, 3)
}
