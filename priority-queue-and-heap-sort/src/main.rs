use std::collections::BinaryHeap;
use rand::Rng;

enum Order {
    Asc,
    Desc
}

fn initialize_bheap(size: u32) -> BinaryHeap<i32> {
    let mut bheap: BinaryHeap<i32> = BinaryHeap::new();
    let mut rng = rand::rng();

    for _ in 0..size {        
        let random_number = rng.random_range(-100..=100);
        bheap.push(random_number);
    }

    bheap
}

fn pop_bheap(bheap: &mut BinaryHeap<i32>) {
    while let Some(element) = bheap.pop() {
        println!("Popped element: {}", element);
    }
}

fn heap_sort(a_vec: &Vec<i32>, an_order: Order) -> Vec<i32> {
    
    let mut bheap: BinaryHeap<i32> = a_vec.iter().copied().collect();
    let mut sorted_vec = Vec::new();

    while let Some(element) = bheap.pop() {
        sorted_vec.push(element);
    }
    
    if let Order::Asc = an_order {
        sorted_vec.reverse();
    }

    sorted_vec
}

fn main() {

    // Part 1
    println!("Part #1:");
    let mut bheap: BinaryHeap<i32> = initialize_bheap(10);
    println!("BinaryHeap elements: {:?}", bheap);
    pop_bheap(&mut bheap);

    // Part 2
    println!("Part #2:");
    let a_vec: Vec<i32> = vec![1, 10, -2, 100, -5, 80];
    println!("Sorted vector (DESC): {:?}", heap_sort(&a_vec, Order::Desc));

    // Part 3
    println!("Part #3:");
    let a_vec: Vec<i32> = vec![1, 10, -2, 100, -5, 80];
    println!("Sorted vector (ASC): {:?}", heap_sort(&a_vec, Order::Asc));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        let mut bheap = BinaryHeap::new();
        bheap.push(10);
        bheap.push(5);
        bheap.push(30);
        bheap.push(20);

        assert_eq!(bheap.pop(), Some(30));
        assert_eq!(bheap.pop(), Some(20));
        assert_eq!(bheap.pop(), Some(10));
        assert_eq!(bheap.pop(), Some(5));
        assert_eq!(bheap.pop(), None);
    }

    #[test]
    fn test_heap_sort_desc() {
        let data = vec![4, 1, 7, 3, 8, 5];
        let sorted = heap_sort(&data, Order::Desc);
        assert_eq!(sorted, vec![8, 7, 5, 4, 3, 1]);
    }

    #[test]
    fn test_heap_sort_asc() {
        let data = vec![4, 1, 7, 3, 8, 5];
        let sorted = heap_sort(&data, Order::Asc);
        assert_eq!(sorted, vec![1, 3, 4, 5, 7, 8]);
    }

    #[test]
    fn test_initialize_bheap_size() {
        let bheap = initialize_bheap(20);
        assert_eq!(bheap.len(), 20);
    }

    #[test]
    fn test_heap_sort_single_element() {
        let data = vec![42];
        let sorted = heap_sort(&data, Order::Asc);
        assert_eq!(sorted, vec![42]);
    }

    #[test]
    fn test_heap_sort_empty() {
        let data = vec![];
        let sorted = heap_sort(&data, Order::Asc);
        assert_eq!(sorted, vec![]);
    }
}
