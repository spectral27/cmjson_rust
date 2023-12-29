use crate::distribution::Distribution;

pub fn get_new_id(distributions: &Vec<Distribution>) -> i32 {
    for i in 0..distributions.len() {
        let current_id = i + 1;

        let distribution = distributions.iter()
            .find(|distribution| distribution.id() == current_id as i32);

        if distribution.is_none() {
            return current_id as i32;
        }
    }

    (distributions.len() + 1) as i32
}
