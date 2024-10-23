
fn sum_digits(i_number: i32) -> i32 {
    
    let mut number = i_number;
    let mut digits_acc = 0;
    
    while number > 0 || digits_acc > 9 {
        
        if number == 0 {
            number = digits_acc;
            digits_acc = 0;
        }

        digits_acc += number % 10;
        number /= 10;
        
    }
    
    digits_acc
}

fn main() {
    let i_number = 942;
    println!("Sum of digits of {} is {}", i_number, sum_digits(i_number));
}

// Write the scaffold of unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_digits() {
        assert_eq!(sum_digits(942), 6);
        assert_eq!(sum_digits(123), 6);
        assert_eq!(sum_digits(9999), 9);
        assert_eq!(sum_digits(5), 5);
    }
}