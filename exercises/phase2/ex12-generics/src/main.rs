use std::fmt::{self, Display};

struct Measurement<T> {
    label: String,
    value: T,
}

impl<T> Measurement<T> {
    fn new(label: &str, value: T) -> Self {
        Measurement {
            label: String::from(label),
            value,
        }
    }
}

impl<T> fmt::Display for Measurement<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.label, self.value)
    }
}

struct Bucket<T> {
    name: String,
    items: Vec<Measurement<T>>,
}

impl<T> Bucket<T> {
    fn new(name: &str) -> Self {
        Bucket {
            name: String::from(name),
            items: vec![],
        }
    }

    fn add(&mut self, m: Measurement<T>) {
        self.items.push(m);
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T> Bucket<T>
where
    T: PartialOrd,
{
    fn max_value(&self) -> Option<&Measurement<T>> {
        self.items
            .iter()
            .reduce(|acc, m| if m.value >= acc.value { m } else { acc })
    }

    fn min_value(&self) -> Option<&Measurement<T>> {
        self.items
            .iter()
            .reduce(|acc, m| if m.value <= acc.value { m } else { acc })
    }

    fn count_above(&self, threshold: &T) -> usize {
        self.items.iter().filter(|i| i.value > *threshold).count()
    }
}

struct Pair<A, B> {
    first: A,
    second: B,
}

impl<A, B> Pair<A, B> {
    fn new(first: A, second: B) -> Self {
        Pair { first, second }
    }

    fn swap(self) -> Pair<B, A> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

impl<A, B> fmt::Display for Pair<A, B>
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}

fn summary<T>(bucket: &Bucket<T>, threshold: &T)
where
    T: Display + PartialOrd,
{
    println!("=== {} ===", bucket.name);
    println!("{} measurements", bucket.len());
    match bucket.max_value() {
        Some(item) => println!("Max: {}", item),
        None => println!("Max: N/A"),
    }
    match bucket.min_value() {
        Some(item) => println!("Min: {}", item),
        None => println!("Min: N/A"),
    }
    println!(
        "Above {:.1}: {}\n",
        threshold,
        bucket.count_above(threshold)
    );
}

fn main() {
    let mut cpu = Bucket::new("CPU Usage (%)");
    cpu.add(Measurement::new("web-server", 87.3));
    cpu.add(Measurement::new("database", 72.1));
    cpu.add(Measurement::new("proxy", 51.5));
    cpu.add(Measurement::new("cache", 12.1));
    cpu.add(Measurement::new("worker", 34.8));

    let mut mem: Bucket<u64> = Bucket::new("Memory (bytes)");
    mem.add(Measurement::new("database", 8_589_934_592));
    mem.add(Measurement::new("app-server", 524_288_000));
    mem.add(Measurement::new("logs", 268_435_456));
    mem.add(Measurement::new("cache", 134_217_728));

    summary(&cpu, &50.0);
    summary(&mem, &1_000_000_000);

    println!("--- Pairs ---");
    let pair1 = Pair::new("web-server", 87.3_f64);
    println!("{}", pair1);
    println!("Swapped: {}", pair1.swap());

    let pair2 = Pair::new("database", 8_589_934_592_u64);
    println!("{}", pair2);
    println!("Swapped: {}", pair2.swap())
}
