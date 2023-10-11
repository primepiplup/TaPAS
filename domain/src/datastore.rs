use crate::datapoint::{create_datapoint, Datapoint};
use std::sync::Mutex;

pub struct Datastore {
    datapoints: Mutex<Vec<Datapoint>>,
}

impl Datastore {
    pub fn new() -> Datastore {
        Datastore {
            datapoints: Mutex::new(Vec::new()),
        }
    }

    pub fn add_datapoint(&self, input: &str) -> () {
        let datapoint = create_datapoint(input);
        let mut lock = self.datapoints.lock().expect("mutex holder crashed");
        lock.push(datapoint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datapoints_are_stored_in_datastore_after_add_function() {
        let datastore = Datastore::new();

        datastore.add_datapoint("Some text with +some +tags");
        let lock = datastore.datapoints.lock().expect("mutex holder crashed");

        assert_eq!(1, lock.len());
    }
}
