# Kata 12: Priority Queue and Heap Sort in Rust

## 📌 Description

In this kata, you will implement a **priority queue** using Rust’s `BinaryHeap` and use it to perform **heap sort** on a vector of integers.

A **priority queue** is a data structure that allows efficient insertion and removal of the highest (or lowest) priority element. The **BinaryHeap** in Rust is a **max-heap** by default, meaning it always retrieves the **largest** element first.

Your goal is to:
- Implement a **priority queue** using `BinaryHeap`.
- Sort a list of integers using a **heap sort algorithm**.

---

## 📜 Instructions

### **Part 1: Implement a Priority Queue**
1. Create a **function** that:
   - Initializes a `BinaryHeap<i32>`.
   - Inserts a set of integers into the heap.
   - Extracts and prints the integers in descending order (default behavior of `BinaryHeap`).

---

### **Part 2: Implement Heap Sort**
1. Implement a **`heap_sort` function** that:
   - Takes a `Vec<i32>` as input.
   - Pushes all elements into a `BinaryHeap`.
   - Extracts them one by one to produce a **sorted** vector.

2. **Sorting Behavior**:
   - Since `BinaryHeap` is a **max-heap** by default, the extracted order is **descending**.
   - To return an **ascending** sorted order, reverse the final result.

---

## 🎯 Objectives
✅ Understand how to use `BinaryHeap` in Rust.  
✅ Learn **priority queue** operations (push, pop).  
✅ Implement **heap sort** using a **max-heap** structure.  
✅ Gain experience handling sorting mechanisms in Rust.  

