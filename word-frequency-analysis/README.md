# Kata 11: File Handling and Word Frequency Analysis

## 📌 Description
Implement a Rust program that reads text from a file, processes it, and counts the frequency of each word using a `HashMap`. The results will be written to an output file.

## 📜 Instructions

1. **Read from a file (`input.txt`)**  
   - Read the contents of `input.txt` (ensure it's in the root of your project).
   - Remove punctuation and convert all text to lowercase.
   - Store words and their frequencies in a `HashMap`.

2. **Process the text**  
   - Count the occurrences of each word.
   - Sort the words by frequency (most frequent first).

3. **Write the results to `output.txt`**  
   - Each line should contain: `"word: count"`.
   - The words should be sorted by frequency in descending order.

## 💡 Tips
- Use `std::fs::read_to_string` to read file contents.
- Use `.to_lowercase()` to normalize words.
- Use `HashMap<String, u32>` to store word frequencies.
- Use `.split_whitespace()` to tokenize words.
- Use `Vec::sort_by` to sort word frequencies before writing to the file.
