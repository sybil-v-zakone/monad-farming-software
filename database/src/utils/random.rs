use rand::{Rng, distr::uniform::SampleUniform};

pub fn random_in_range<T>(range: [T; 2]) -> T
where
    T: SampleUniform + PartialOrd + Copy,
{
    let start = range[0];
    let end = range[1];

    let inclusive_range = if start <= end { start..=end } else { end..=start };

    rand::rng().random_range(inclusive_range)
}
