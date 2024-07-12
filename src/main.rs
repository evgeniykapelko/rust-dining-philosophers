use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const NUM_PHILOSOPHERS: usize = 5;

struct Philosopher {
    name: String,
    left_fork: usize,
    right_fork: usize,
}

impl Philosopher {
    fn new(name: &str, left_fork: usize, right_fork: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left_fork,
            right_fork,
        }
    }

    fn eat(&self, forks: &Vec<Mutex<()>>) {
        let _left = forks[self.left_fork].lock().unwrap();
        let _right = forks[self.right_fork].lock().unwrap();

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let forks = Arc::new((0..NUM_PHILOSOPHERS).map(|_| Mutex::new(())).collect::<Vec<_>>());

    let philosophers = vec![
        Philosopher::new("Philosopher 1", 0, 1),
        Philosopher::new("Philosopher 2", 1, 2),
        Philosopher::new("Philosopher 3", 2, 3),
        Philosopher::new("Philosopher 4", 3, 4),
        Philosopher::new("Philosopher 5", 4, 0),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let forks = Arc::clone(&forks);
        thread::spawn(move || {
            p.eat(&forks);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}

