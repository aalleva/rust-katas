# Kata 10: Modular Library for Mathematical Utilities

## Description
Create a modular library in Rust that provides various mathematical utilities. Split the functions into different modules based on their purpose and use `pub` to expose them.

## Instructions

1. **Create a root module `math_utils`**:
   - This will serve as the entry point for mathematical utilities.

2. **Submodules**:
   - **`basic`**: Contains basic operations like addition, subtraction, multiplication, and division.
   - **`geometry`**: Includes geometric calculations like area and circumference of a circle.
   - **`statistics`**: Provides statistical functions like mean and median.

3. **Functions in Submodules**:
   - **`basic`**:
     - `add(a: i32, b: i32) -> i32`
     - `subtract(a: i32, b: i32) -> i32`
     - `multiply(a: i32, b: i32) -> i32`
     - `divide(a: i32, b: i32) -> Option<i32>` (handles division by zero by returning `None`)

   - **`geometry`**:
     - `circle_area(radius: f64) -> f64`
     - `circle_circumference(radius: f64) -> f64`

   - **`statistics`**:
     - `mean(numbers: &[f64]) -> Option<f64>`
     - `median(numbers: &mut [f64]) -> Option<f64>`

4. **Project Structure**:
   Organize your files like this:
    - **`main.rs`**: Entry point of the application. Use the modules from `math_utils` to perform and display mathematical operations.
    - **`math_utils/mod.rs`**: Root module for `math_utils`. Expose submodules `basic`, `geometry`, and `statistics`.
    - **`math_utils/basic.rs`**: Contains functions like addition, subtraction, multiplication, and division.
    - **`math_utils/geometry.rs`**: Contains functions like area and circumference calculations for a circle.
    - **`math_utils/statistics.rs`**: Contains functions like mean and median calculations.

5. **Entry Point (`main.rs`)**:
Use the library to perform mathematical operations and display the results.

## Example Usage
```rust
use math_utils::basic;
use math_utils::geometry;
use math_utils::statistics;

fn main() {
 println!("Add: {}", basic::add(5, 10));
 println!("Circle Area: {:.2}", geometry::circle_area(5.0));
 println!("Mean: {:?}", statistics::mean(&[1.0, 2.0, 3.0]));
}