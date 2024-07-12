# Dining Philosophers Problem in Rust

This project implements the classic Dining Philosophers problem in Rust. The Dining Philosophers problem is a well-known synchronization problem in computer science that illustrates the challenges of resource allocation and potential for deadlock in concurrent systems.

## Problem Description

The problem involves five philosophers who spend their time alternately thinking and eating. They sit around a circular table, with a bowl of spaghetti in the center and a fork placed between each pair of adjacent philosophers. Each philosopher needs both forks to eat, but can only pick up one fork at a time.

### Key Concepts
1. **Mutual Exclusion**: Only one philosopher can use a fork at any given time.
2. **Deadlock**: A situation where all philosophers pick up one fork and wait for the other, leading to a standstill.
3. **Starvation**: A situation where a philosopher never gets both forks and thus never gets to eat.

## Solution Approach

This implementation uses Rust's standard library features for thread synchronization:
- **Threads**: To represent each philosopher.
- **Mutexes**: To represent each fork, ensuring mutual exclusion.

### Avoiding Deadlock
In this implementation, deadlock is avoided by making philosophers pick up the forks in a specific order. For example, one philosopher will pick up the right fork first and then the left, while others pick up the left fork first and then the right. This ensures that at least one philosopher can always eat.
