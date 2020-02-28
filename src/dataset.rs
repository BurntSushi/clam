use super::types::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Dataset {
    pub data: Box<Data>,
    pub metric: String,
}

impl Dataset {
    pub fn len(&self) -> Index {
        self.data.len() as u64
    }
    pub fn distance(&self, left: Indices, right: Indices) -> Radius {
        left.iter().zip(&right).fold(0, |sum, (a, b)| sum + a + b) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _d = Dataset { data: Box::new(vec![0, 0]), metric: String::from("euclidean") };
    }
}