pub fn mean(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None
    }
    
    let sum : f64 = numbers.iter().sum();
    Some (sum / numbers.len() as f64 )
}

pub fn median(numbers: &mut [f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None
    }

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = numbers.len() / 2;

    Some (if numbers.len() % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2.0
    } else {
        numbers[mid]
    })

}