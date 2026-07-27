use std::{cell::RefCell, fmt::{Display}, rc::Rc};

fn main() {

    enum Status {
        Up,
        Down,
    }

    impl Display for Status {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Status::Up => write!(f, "up"),
                Status::Down => write!(f, "down"),
            }
        }
    }

    struct Service {
        name: String,
        status: RefCell<Status>,
        deps: Vec<Rc<Service>>,
    }

    impl Service {
        fn new(name: &str, deps: Vec<Rc<Service>>) -> Rc<Service> {
            Rc::new(Service{ name: String::from(name), status: RefCell::new(Status::Up), deps })
        }

        fn set_status(&self, status:Status) {
            *self.status.borrow_mut() = status;
        }

        fn print_status(&self) {
            println!("{}: {}", self.name, self.status.borrow());
        }

        fn is_healthy(&self) -> bool {
            if self.deps.is_empty() {
                matches!(*self.status.borrow(), Status::Up)
            } else {
                ! self.deps.iter().any(|d| d.is_healthy() == false)
            }
        }
    }


    let disk = Service::new("disk", vec![]);
    let database = Service::new("database", vec![Rc::clone(&disk)]);

    let api = Service::new("api", vec![Rc::clone(&database)]);
    let web = Service::new("web", vec![Rc::clone(&database)]);

    println!("=== Infra Status ===");
    disk.print_status();
    database.print_status();
    api.print_status();
    web.print_status();

    println!("");
    println!("api healthy? {}", api.is_healthy());
    println!("web healthy? {}", web.is_healthy());

    println!("");
    println!("-- disk goes down --");
    disk.set_status(Status::Down);

    println!("");
    println!("api healthy? {}", api.is_healthy());
    println!("web healthy? {}", web.is_healthy());

    println!("");
    println!("references to disk: {}", Rc::strong_count(&disk));
    println!("references to database: {}", Rc::strong_count(&database));
}

