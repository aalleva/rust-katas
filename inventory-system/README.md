# Kata 9: Inventory System with Structs and Enums

## Description
Create an inventory system that can store different types of items. Each inventory item should have common properties like name and quantity, but also be able to belong to different types, such as perishable or non-perishable. Use `Structs` and `Enums` to represent the inventory items.

## Instructions

1. **Define the `Item` struct**:
   - **Fields**:
     - `name`: `String`
     - `quantity`: `u32`
     - `item_type`: `ItemType` (an enum to indicate whether it is perishable or non-perishable)

2. **Define the `ItemType` enum**:
   - Variants:
     - `Perishable(expiration_date: String)`
     - `NonPerishable`

3. **Implement methods for `Item`**:
   - **`new`**: Creates a new `Item` and allows specifying its type.
   - **`description`**: Returns a formatted `String` describing the item, including the type and expiration date if applicable.

4. **Add logic to manage items**:
   - Implement a method to decrease the quantity of an item (`reduce_quantity`), and return a `Result` indicating if the operation was successful or if there is insufficient quantity.

## Example Usage
```rust
let mut apple = Item::new("Apple", 50, ItemType::Perishable("2024-12-01".to_string()));
println!("{}", apple.description());
assert_eq!(apple.reduce_quantity(10), Ok(()));
assert_eq!(apple.reduce_quantity(100), Err("Not enough quantity".to_string()));
