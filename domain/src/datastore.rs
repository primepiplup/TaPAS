use crate::datapoint::{create_datapoint, Datapoint};

struct Datastore {
    datapoints: Vec<Datapoint>,
}

impl Datastore {
    pub fn add_datapoint(&mut self, input: &str) -> () {
        let datapoint = create_datapoint(input);
        self.datapoints.push(datapoint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {}
}
