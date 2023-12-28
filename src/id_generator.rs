use crate::distribution::Distribution;

pub fn get_new_id(distributions: &Vec<Distribution>) -> u32 {
    for i in 0..distributions.len() {
        let current_id = i + 1;

        let distribution = distributions.iter()
            .find(|distribution| distribution.id() == current_id as u32);

        if distribution.is_none() {
            return current_id as u32;
        }
    }

    (distributions.len() + 1) as u32
}
