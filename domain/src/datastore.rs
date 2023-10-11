use crate::datapoint::{create_datapoint, Datapoint};

struct Datastore {
    datapoints: Vec<Datapoint>,
}

impl Datastore {
    pub fn new() -> Datastore {
        Datastore {
            datapoints: Vec::new(),
        }
    }

    pub fn add_datapoint(&mut self, input: &str) -> () {
        let datapoint = create_datapoint(input);
        self.datapoints.push(datapoint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datapoints_are_stored_in_datastore_after_add_function() {
        let mut datastore = Datastore::new();

        datastore.add_datapoint("Some text with +some +tags");

        assert_eq!(1, datastore.datapoints.len());
    }
}
