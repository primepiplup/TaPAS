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

    pub fn query(&self, query: &str) -> Vec<Datapoint> {
        let mut collector = Vec::new();
        let parsed = query_parser(query);
        let datapoints = self.retrieve_datapoints();
        if parsed.len() < 1 || &parsed[0] == "" {
            return datapoints;
        }
        for datapoint in datapoints {
            let parsed = &parsed;
            let truthvalues: Vec<bool> = parsed
                .into_iter()
                .map(|tag| datapoint.get_tags().contains(&tag))
                .collect();
            if !truthvalues.contains(&false) {
                collector.push(datapoint.clone());
            }
        }
        collector
    }
}

fn query_parser(query: &str) -> Vec<String> {
    let plus_replaced = query.trim().replace("+", " ");
    plus_replaced
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::datapoint;

    use super::*;

    #[test]
    fn query_parser_removes_plusses() {
        let mut expected: Vec<&str> = Vec::new();
        expected.push("tag");

        let parsed = query_parser("+tag");

        assert_eq!(expected, parsed);
    }

    #[test]
    fn query_parser_separates_on_whitespace() {
        let mut expected: Vec<&str> = Vec::new();
        expected.push("tag");
        expected.push("another");

        let parsed = query_parser("+tag another");

        assert_eq!(expected, parsed);
    }

    #[test]
    fn query_parser_separates_on_plusses() {
        let mut expected: Vec<&str> = Vec::new();
        expected.push("tag");
        expected.push("another");

        let parsed = query_parser("+tag+another");

        assert_eq!(expected, parsed);
    }

    #[test]
    fn query_with_tag_only_retrieves_tagged_datapoint() {
        let datastore = Datastore::new();

        datastore.add_datapoint("information +with +tags");
        datastore.add_datapoint("information +different");

        let query_result = datastore.query("+different");

        assert_eq!(1, query_result.len());
    }

    #[test]
    fn query_with_word_only_retrieves_tagged_datapoint() {
        let datastore = Datastore::new();

        datastore.add_datapoint("information +with +tags");
        datastore.add_datapoint("information +different");

        let query_result = datastore.query("different");

        assert_eq!(1, query_result.len());
    }

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
