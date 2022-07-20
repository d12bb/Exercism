pub fn create_empty() -> Vec<u8> {
    vec![]
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

// vec![1, 1, 2, 3, 5] would be boring, so onto something more interesting..
struct Fib(u8, u8);
impl Fib {
    fn new() -> Fib {
        Fib(0, 1)
    }
}
impl Iterator for Fib {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Fib(144, 233) => None, // prevent u8 overflow
            _ => {
                (self.0, self.1) = (self.1, self.0 + self.1);
                Some(self.0)
            }
        }
    }
}
pub fn fibonacci() -> Vec<u8> {
    Fib::new().take(5).collect()
}
