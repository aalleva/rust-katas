#[derive(Debug)]
enum ItemType {
    Perishable (String),
    Nonperishable,
}

struct Item {
    name: String,
    quantity: u32,
    item_type: ItemType,
}

impl Item {
    fn description(&self) -> String {
        match &self.item_type {
            ItemType::Perishable(expiration_date) => format!("{} ({} left) - Expires on {}", self.name, self.quantity, expiration_date),
            ItemType::Nonperishable => format!("{} ({} left)", self.name, self.quantity),
        }
    }

    fn new(name: String, quantity: u32, item_type: ItemType) -> Self {
        Self {
            name,
            quantity,
            item_type,
        }
    }

    fn reduce_quantity(&mut self, amount: u32) -> Result<(), String> {
        if self.quantity >= amount {
            self.quantity -= amount;
            Ok(())
        } else {
            Err(format!("Insufficient quantity of {}", self.name))
        }
    }

}

fn main() {
    let mut orange = Item::new("Orange".to_string(), 100, ItemType::Perishable("2025-06-30".to_string()));
    println!("{}", orange.description());
    orange.reduce_quantity(50).unwrap();
    println!("{}", orange.description());

    let mut oil = Item::new("Oil".to_string(), 10, ItemType::Nonperishable);
    oil.reduce_quantity(4).unwrap();
    println!("{}", oil.description());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_quantity() {
        let mut orange = Item::new("Orange".to_string(), 100, ItemType::Perishable("2025-06-30".to_string()));
        assert_eq!(orange.reduce_quantity(50), Ok(()));
        assert_eq!(orange.quantity, 50);
        assert_eq!(orange.reduce_quantity(60), Err("Insufficient quantity of Orange".to_string()));
    }
}