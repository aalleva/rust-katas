struct Book {
    title: String,
    author: String,
    year: u32,
    available: bool,
}

impl Book {

    fn new(title: &str, author: &str, year: u32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            available: true,
            year: year,
        }
    }

    fn is_available(&self) -> bool {
        self.available
    }

    fn borrow(&mut self) -> Option<String> {

        if self.is_available() {
            self.available = false;
            return Some("Borrowed successfully".to_string());
        } else {
            return None;
        }
    }

    fn return_book(&mut self) {
        self.available = true;
    }

    fn description(&self) -> String {
        let mut available = "No";

        if self.available {
            available = "Yes";
        }

        format!("Title: {}, Author: {}, Year: {}, Available: {}", self.title, self.author, self.year, available)
    }

}

fn main() {
    let mut book = Book::new("The Rust Book", "Steve Klabnik", 2019);
    println!("{}", book.description());
    book.borrow();
    println!("{}", book.description());
    book.return_book();
    println!("{}", book.description());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_struct() {
        let mut book = Book::new("The Rust Book", "Steve Klabnik", 2019);
        assert_eq!(book.is_available(), true);
        assert_eq!(book.borrow(), Some("Borrowed successfully".to_string()));
        assert_eq!(book.is_available(), false);
        book.return_book();
        assert_eq!(book.is_available(), true);
        assert_eq!(book.description(), "Title: The Rust Book, Author: Steve Klabnik, Year: 2019, Available: Yes".to_string());
    }
}