# Kata 8: Struct for a Book Catalog

## Description
Create a `struct` that represents a book in a catalog. Each book should have a title, author, year of publication, and a field indicating if it is available for loan. Then, implement methods to modify the availability of the book and to get a description as a formatted string.

## Instructions

1. **Define the `Book` struct**:
   - **Fields**:
     - `title`: `String`
     - `author`: `String`
     - `year`: `u32`
     - `available`: `bool` (indicates if the book is available for loan)

2. **Implement the methods**:
   - **`is_available`**: Returns `true` if the book is available for loan.
   - **`borrow`**: Changes `available` to `false` if the book is available and returns `Some("Borrowed successfully")`. If not available, returns `None`.
   - **`return_book`**: Changes `available` to `true`.
   - **`description`**: Returns a formatted `String` describing the book, such as: `"Title: The Rust Book, Author: Steve Klabnik, Year: 2019, Available: Yes"`

## Example Usage
```rust
let mut book = Book::new("The Rust Book", "Steve Klabnik", 2019);
assert_eq!(book.is_available(), true);
assert_eq!(book.borrow(), Some("Borrowed successfully".to_string()));
assert_eq!(book.is_available(), false);
book.return_book();
assert_eq!(book.is_available(), true);
assert_eq!(book.description(), "Title: The Rust Book, Author: Steve Klabnik, Year: 2019, Available: Yes".to_string());
