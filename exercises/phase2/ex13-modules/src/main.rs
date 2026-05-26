use queue::Queue;
use task::{Priority, Task};

mod task {
    use std::fmt::Display;

    pub enum Priority {
        Low,
        Medium,
        High,
    }

    impl Display for Priority {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Priority::High => write!(f, "HIGH  "),
                Priority::Medium => write!(f, "MEDIUM"),
                Priority::Low => write!(f, "LOW   "),
            }
        }
    }

    pub struct Task {
        id: u32,
        pub title: String,
        pub priority: Priority,
        done: bool,
    }

    impl Display for Task {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}] #{} {}", self.priority, self.id(), self.title)
        }
    }

    impl Task {
        pub fn new(id: u32, title: &str, priority: Priority) -> Self {
            Task {
                id,
                title: String::from(title),
                priority,
                done: false,
            }
        }
        pub fn complete(&mut self) {
            self.done = true
        }
        pub fn is_done(&self) -> bool {
            self.done
        }
        pub fn id(&self) -> u32 {
            self.id
        }
    }
}

mod queue {

    use super::task::Task;

    pub struct Queue {
        name: String,
        tasks: Vec<Task>,
    }

    impl Queue {
        pub fn new(name: &str) -> Self {
            Queue {
                name: String::from(name),
                tasks: vec![],
            }
        }
        pub fn push(&mut self, task: Task) {
            self.tasks.push(task);
        }
        pub fn complete_by_id(&mut self, id: u32) -> bool {
            match self.tasks.iter_mut().find(|t| t.id() == id) {
                Some(task) => {
                    task.complete();
                    true
                }
                None => false,
            }
        }
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn pending(&self) -> Vec<&Task> {
            self.tasks
                .iter()
                .filter(|t| !t.is_done())
                .collect::<Vec<&Task>>()
        }
        pub fn done(&self) -> Vec<&Task> {
            self.tasks
                .iter()
                .filter(|t| t.is_done())
                .collect::<Vec<&Task>>()
        }
    }
}

mod report {
    use super::queue::Queue;

    pub fn print_status(queue: &Queue) {
        println!("=== {} ===", queue.name());
        println!("\nPending tasks:");
        let mut count_pending = 0;
        for task in queue.pending().iter() {
            println!("  {}", task);
            count_pending += 1;
        }

        let mut count_completed = 0;
        println!("\nCompleted tasks:");
        for task in queue.done().iter() {
            println!("  {}", task);
            count_completed += 1;
        }

        println!("\n{} pending, {} done", count_pending, count_completed);
    }
}

fn main() {
    let mut queue = Queue::new("DevOps Queue");
    queue.push(Task::new(1, "Fix critical bug", Priority::High));
    queue.push(Task::new(2, "Run integration tests", Priority::Medium));
    queue.push(Task::new(3, "Deploy to production", Priority::High));
    queue.push(Task::new(4, "Update load balancer", Priority::Medium));
    queue.push(Task::new(5, "Archive old logs", Priority::Low));

    queue.complete_by_id(1);
    queue.complete_by_id(2);

    use report::print_status;
    print_status(&queue);
}
