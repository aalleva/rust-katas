use std::collections::HashMap;
use std::io::Write;

fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    let file_content = std::fs::read_to_string(file_name)?;
    Ok(file_content)
}

fn clean_text(text: &str) -> String {
    text
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

fn process_content(content: &str, frequency_map: &mut HashMap<String, u32>) {
    
    for word in clean_text(content).split_whitespace() {
        *frequency_map.entry(word.to_string()).or_insert(0) += 1;
    }

}

fn write_output(frequency_map: HashMap<String, u32>, file_name: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(file_name)?;
    let mut sorted_pairs: Vec<_> = frequency_map.iter().collect();
    sorted_pairs.sort_by(|a, b| b.1.cmp(a.1));

    for (word, frequency) in sorted_pairs {
        let line = format!("{}: {}\n", word, frequency);
        file.write_all(line.as_bytes())?;
   
    }

    Ok(())
}

fn main() {
    
    match read_file("input.txt") {
        Ok(content) => {  
            let mut frequency_map: HashMap<String, u32> = HashMap::new();
            process_content(&content, &mut frequency_map);

            if let Err(e) = write_output(frequency_map, "output.txt") {
                eprintln!("Error writing to file: {}", e);
            }
        },
        Err(error) => eprintln!("Error reading file: {}", error),
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text() {
        let input = "Hello, World!";
        let expected = "hello world";
        assert_eq!(clean_text(input), expected);
    }

    #[test]
    fn test_process_content() {
        let content = "Rust is great. Rust is fast.";
        let mut frequency_map = HashMap::new();
        process_content(content, &mut frequency_map);
        assert_eq!(frequency_map.get("rust"), Some(&2));
        assert_eq!(frequency_map.get("is"), Some(&2));
        assert_eq!(frequency_map.get("great"), Some(&1));
        assert_eq!(frequency_map.get("fast"), Some(&1));
    }
}
