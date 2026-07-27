use node::Node;
use pod::Pod;
use scheduler::{Scheduler, FirstFit, SchedulingPolicy};
use util::sum_by;


mod util {
    pub fn sum_by<T, F>(items: &[T], f: F) -> u32 
    where F: Fn(&T) -> u32 {
        items.iter().map(f).sum()
    }
}

mod error {
    use std::{fmt::Display};

    #[derive(Debug)]
    pub enum ClusterError {
        NodeNotFound(String),
        NotEnoughResources{ node: String, requested: u32, available: u32},
        NodeNotAvailable
    }

    impl Display for ClusterError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ClusterError::NodeNotFound(name) => write!(f, "node not found: {}", name),
                ClusterError::NotEnoughResources { node, requested, available } => {
                    write!(f, "not enough resources on {} (requested {}, available {})", node, requested, available)
                },
                ClusterError::NodeNotAvailable => write!(f, "no node available"),
            }
        }
    }
    
    impl std::error::Error for ClusterError {}

}

mod node {
    use std::{cell::RefCell, fmt::{Display}, rc::Rc};
    use super::error::ClusterError;

    pub struct Node {
        pub name: String,
        pub cpu_total: u32,
        pub cpu_used: RefCell<u32>
    }

    impl Node {
        pub fn new(name: &str, cpu_total: u32) -> Rc<Node> {
            Rc::new(Node { name: String::from(name), cpu_total: cpu_total, cpu_used: 0.into() })
        }

        pub fn available(&self) -> u32 {
            return self.cpu_total - *self.cpu_used.borrow();
        }

        pub fn allocate(&self, cpu: u32) -> Result<(), ClusterError> {
            if cpu < self.available() {
                *self.cpu_used.borrow_mut() += cpu;
                Ok(())
            } else {
                Err(ClusterError::NotEnoughResources { node: self.name.clone(), requested: cpu, available: self.available() })
            }
        }
    }

    impl Display for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}: {}/{} cpu", self.name, self.cpu_used.borrow(), self.cpu_total)
        }
    }
}

mod pod {
    use std::{cell::RefCell, fmt::{Display}, rc::Rc};
    use super::node::Node;

    pub struct Pod {
        pub name: String,
        pub cpu: u32,
        pub node: RefCell<Option<Rc<Node>>>
    }

    impl Pod {
        pub fn new(name: &str, cpu: u32) -> Self {
            Pod { name: String::from(name), cpu, node: RefCell::new(Option::None) }
        }

        pub fn assign(&self, node: Rc<Node>) {
            let _ = node.allocate(self.cpu);
            *self.node.borrow_mut() = Some(node)
        }

        pub fn assigned_node(&self) -> Option<Rc<Node>> {
            self.node.borrow().clone()
        }
    }

    impl Display for Pod  {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} ({} cpu)", self.name, self.cpu)
        }
    }
}

mod scheduler {
    use std::rc::Rc;
    use super::error::ClusterError;
    use super::node::Node;
    use super::pod::Pod;

    pub trait SchedulingPolicy {
        fn choose(&self, nodes: &[Rc<Node>], cpu: u32) -> Option<Rc<Node>>;
        fn name (&self) -> &str;
    }

    pub struct FirstFit;

    impl SchedulingPolicy for FirstFit  {
        fn choose(&self, nodes: &[Rc<Node>], cpu: u32) -> Option<Rc<Node>> {
            nodes.iter().find(|n| n.available() >= cpu).cloned()
        }
    
        fn name (&self) -> &str {
            "first-fit"
        }
    }


    pub struct Scheduler {
        nodes: Vec<Rc<Node>>,
        policy: Box<dyn SchedulingPolicy>,
    }

    impl Scheduler {
        pub fn new(nodes: Vec<Rc<Node>>, policy: Box<dyn SchedulingPolicy>) -> Scheduler {
            Scheduler { nodes, policy }
        }

        pub fn node(&self, name: &str) -> Result<Rc<Node>, ClusterError> {
            self.nodes.iter()
            .find(|n| n.name == name).cloned()
            .ok_or_else(|| ClusterError::NodeNotFound(String::from(name)))
        }

        pub fn schedule(&self, pod: &Pod) -> Result<(), ClusterError> {            
            
            match self.policy.choose(&self.nodes, pod.cpu) {
                Some(node) => Ok(pod.assign(node)),
                None => Err(ClusterError::NodeNotAvailable),
            }
        } 
    }
}

fn main() {

    let nodea = Node::new("node-a", 4);
    let nodeb = Node::new("node-b", 8);

    let firstfit: Box<dyn SchedulingPolicy> = Box::new(FirstFit);
    println!("=== Cluster Scheduler ({}) ===\n", &firstfit.name());

    
    let scheduler: Scheduler = Scheduler::new(vec![nodea.clone(), nodeb.clone()], firstfit);
    
    let p1 = Pod::new("p1", 3);
    let p2 = Pod::new("p2", 5);
    let p3 = Pod::new("p3", 2);
    let p4 = Pod::new("p4", 4);
    
    
    for pod in [p1, p2, p3, p4]  {
        match scheduler.schedule(&pod) {
            Ok(_) => println!("scheduled {} -> {}", pod, pod.assigned_node().unwrap().name),
            Err(err) => println!("failed to schedule {}: {}", pod.name, err),
        }    
    }
    println!("\n--- Cluster state ---");
    println!("{}", nodea.clone());
    println!("{}", nodeb.clone());
    
    let nodes = &[nodea.clone(), nodeb.clone()];
    println!("\ntotal capacity: {} cpu", sum_by(nodes, |n| n.cpu_total));
    println!("total used: {} cpu", sum_by(nodes, |n| *n.cpu_used.borrow()));
    
    println!("\n--- Error handling ---");
    match scheduler.node("ghost") {
        Ok(_) => todo!(),
        Err(err) => println!("lookup ghost: {}", err),
    }

    match nodea.allocate(5) {
        Ok(_) => println!("scheduled {}", "test"),
        Err(err) => println!("over-allocate {} by 5: {}", nodea.name, err),
    } 

    
}
