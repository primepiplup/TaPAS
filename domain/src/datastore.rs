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

    pub fn retrieve_datapoints(&self) -> Vec<Datapoint> {
        let lock = self.datapoints.lock().expect("mutex holder crashed");
        lock.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::datapoint;

    use super::*;

    #[test]
    fn datapoints_are_stored_in_datastore_after_add_function() {
        let datastore = Datastore::new();

        datastore.add_datapoint("Some text with +some +tags");
        let lock = datastore.datapoints.lock().expect("mutex holder crashed");

        assert_eq!(1, lock.len());
    }

    #[test]
    fn datapoints_can_be_retrieved_from_datastore() {
        let datastore = Datastore::new();
        let input_text = "Some text with +some +tags";
        let expected_datapoint = datapoint::create_datapoint(input_text);

        datastore.add_datapoint(input_text);
        let actual_datapoint = &datastore.retrieve_datapoints()[0];

        assert!(actual_datapoint.data_same_as(&expected_datapoint));
        assert!(actual_datapoint.tags_same_as(&expected_datapoint));
    }
}
