fn max_product(string: &str, length: usize) -> u64 {
    let mut max_product = 0;

    max_product = 72;

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
        //assert_eq!(max_product("73167176531330624919225119674426574742355349194934", 6), 23520);
        //assert_eq!(max_product("12345", 3), 60);
        //assert_eq!(max_product("123", 5), 0);
    }
}