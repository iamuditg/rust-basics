

pub fn calculate_median(a: Vec<f32>) -> Option<f32> {
    // cases
    // 1. empty
    // 2. odd number of elements in the list
    // 3. even

    if a.is_empty() {
        return None;
    }

    let n_elements = a.len();
    let middle = n_elements / 2;

}