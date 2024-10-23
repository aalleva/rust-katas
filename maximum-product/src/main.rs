fn max_product(string: &str, length: usize) -> u64 {
    let mut max_product = 0;

    if string.len() < length {
        return 0;
    }

    for i in 0..=string.len() - length {
        
        let slice = &string[i..i + length];
        let product = slice.chars().fold(1, |acc, c| acc * c.to_digit(10).unwrap_or(0) as u64);

        if product > max_product {
            max_product = product;
        }

    }

    max_product
}

fn main() {
    println!("{}", max_product("123456789", 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product() {
        assert_eq!(max_product("123456789", 2), 72);
        assert_eq!(max_product("73167176531330624919225119674426574742355349194934", 6), 23520);
        assert_eq!(max_product("12345", 3), 60);
        assert_eq!(max_product("123", 5), 0);
        assert_eq!(max_product("1020304050607089", 2), 72);
        assert_eq!(max_product("2,C,8,10", 2), 0);
    }
}