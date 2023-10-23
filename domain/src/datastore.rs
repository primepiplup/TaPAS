use crate::datapoint::{create_datapoint, Datapoint};
use std::sync::Mutex;

pub struct Datastore {
    datapoints: Mutex<Vec<Datapoint>>,
    tags: Mutex<Vec<String>>,
}

impl Datastore {
    pub fn new() -> Datastore {
        Datastore {
            datapoints: Mutex::new(Vec::new()),
            tags: Mutex::new(Vec::new()),
        }
    }

    pub fn add_datapoint(&self, input: &str) -> () {
        let datapoint = create_datapoint(input);
        self.append_tags(datapoint.get_tags());
        let mut lock = self.datapoints.lock().expect("mutex holder crashed");
        for i in 0..lock.len() {
            if lock[i].get_datetime() > datapoint.get_datetime() {
                lock.insert(i, datapoint.clone());
                return;
            }
        }
        lock.push(datapoint);
    }

    pub fn retrieve_datapoints(&self) -> Vec<Datapoint> {
        let lock = self.datapoints.lock().expect("mutex holder crashed");
        lock.clone()
    }

    pub fn retrieve_taglist(&self) -> Vec<String> {
        let lock = self.tags.lock().expect("Mutex holder crashed...");
        lock.clone()
    }

    pub fn query(&self, query: &str) -> (Vec<Datapoint>, Vec<Vec<String>>) {
        let mut collector: Vec<Datapoint> = Vec::new();
        let parsed: Vec<Vec<String>> = query_parser(query);
        let datapoints: Vec<Datapoint> = self.retrieve_datapoints();
        if parsed.len() < 1 || &parsed[0][0] == "" {
            return (datapoints, parsed);
        }
        for datapoint in datapoints {
            let parsed = &parsed;
            let truthvalues: Vec<bool> = parsed
                .into_iter()
                .map(|tag| datapoint.get_tags().contains(&tag[0]))
                .collect();
            if !truthvalues.contains(&false) {
                collector.push(datapoint.clone());
            }
        }
        apply_query_commands(collector, parsed)
    }

    fn append_tags(&self, tags: &Vec<String>) -> () {
        let mut lock = self.tags.lock().expect("Mutex holder crashed...");
        for tag in tags {
            if !lock.contains(tag) {
                lock.push(tag.clone());
            }
        }
    }
}

fn query_parser(query: &str) -> Vec<Vec<String>> {
    let plus_replaced = query.trim().replace("+", " ");
    plus_replaced
        .split_whitespace()
        .map(|s| s.split(":").map(|s| s.to_string()).collect())
        .collect()
}

fn apply_query_commands(
    datapoints: Vec<Datapoint>,
    queries: Vec<Vec<String>>,
) -> (Vec<Datapoint>, Vec<Vec<String>>) {
    let mut transformed = datapoints;
    for element in &queries {
        if element.len() > 1 {
            transformed = apply_command(transformed, element[1].clone());
        }
    }
    (transformed, queries)
}

fn apply_command(datapoints: Vec<Datapoint>, command: String) -> Vec<Datapoint> {
    match command.as_str() {
        "value" => strip_non_numeric(datapoints),
        _ => datapoints,
    }
}

fn strip_non_numeric(datapoints: Vec<Datapoint>) -> Vec<Datapoint> {
    datapoints
        .into_iter()
        .map(|point| point.get_non_numeric_stripped())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::datapoint;

    use super::*;

    #[test]
    fn empty_query_returns_all_datapoints_from_store() {
        let datastore = Datastore::new();
        datastore.add_datapoint("cool information +tag");
        datastore.add_datapoint("More cool information +tag");

        let (datapoints, _) = datastore.query("+");

        assert_eq!("cool information", datapoints[0].get_data());
        assert_eq!("More cool information", datapoints[1].get_data());
    }

    #[test]
    fn apply_command_with_unknown_command_returns_full_datapoint_vector() {
        let datastore = Datastore::new();
        datastore.add_datapoint("cool information +tag");
        datastore.add_datapoint("More cool information +tag");
        let datapoints = datastore.retrieve_datapoints();

        let datapoints_after_command = apply_command(datapoints.clone(), "Unknown".to_string());

        assert_eq!(
            datapoints[0].get_data(),
            datapoints_after_command[0].get_data()
        );
        assert_eq!(
            datapoints[1].get_data(),
            datapoints_after_command[1].get_data()
        );
    }

    #[test]
    fn append_tags_does_not_append_tags_already_included() {
        let datastore = Datastore::new();
        datastore.add_datapoint("80kg +something");
        datastore.add_datapoint("stuff +tag");

        datastore.append_tags(&vec![
            "something".to_string(),
            "tag".to_string(),
            "weight".to_string(),
        ]);

        assert_eq!(
            datastore.retrieve_taglist(),
            vec![
                "something".to_string(),
                "tag".to_string(),
                "weight".to_string()
            ]
        );
    }

    #[test]
    fn append_tags_function_appends_tags_to_datastore_tag_list() {
        let datastore = Datastore::new();
        datastore.add_datapoint("80kg +weight ");
        datastore.add_datapoint("stuff +tag");

        datastore.append_tags(&vec!["something".to_string()]);

        assert_eq!(
            datastore.retrieve_taglist(),
            vec![
                "weight".to_string(),
                "tag".to_string(),
                "something".to_string()
            ]
        );
    }

    #[test]
    fn data_added_tagged_before_current_datetime_is_inserted_in_appropriate_location() {
        let datastore = Datastore::new();
        datastore.add_datapoint("80kg +weight +DATE:2023-10-16");
        datastore.add_datapoint("80.5kg +weight +DATE:2023-10-15");

        let datapoints = datastore.retrieve_datapoints();

        assert_eq!(datapoints[0].get_data(), "80.5kg");
    }

    #[test]
    fn value_command_in_query_strips_non_numeric_information() {
        let datastore = Datastore::new();
        datastore.add_datapoint("80kg +weight");
        datastore.add_datapoint("8kg +curl");

        let (retrieved, parsed) = datastore.query("curl:value");

        assert_eq!(retrieved[0].get_data(), "8");
    }

    #[test]
    fn value_command_strips_non_numeric_information_from_data() {
        let datastore = Datastore::new();
        datastore.add_datapoint("80kg +weight");

        let datapoints = datastore.retrieve_datapoints();
        let valuestripped = apply_command(datapoints, "value".to_owned());

        assert_eq!(valuestripped[0].get_data(), "80");
    }

    #[test]
    fn query_parser_removes_plusses() {
        let mut expected: Vec<&str> = Vec::new();
        expected.push("tag");

        let parsed: Vec<String> = query_parser("+tag")
            .into_iter()
            .map(|tagelem| tagelem[0].clone())
            .collect();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn query_parser_separates_on_whitespace() {
        let mut expected: Vec<&str> = Vec::new();
        expected.push("tag");
        expected.push("another");

        let parsed: Vec<String> = query_parser("+tag another")
            .into_iter()
            .map(|tagelem| tagelem[0].clone())
            .collect();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn query_parser_separates_on_plusses() {
        let mut expected: Vec<&str> = Vec::new();
        expected.push("tag");
        expected.push("another");

        let parsed: Vec<String> = query_parser("+tag+another")
            .into_iter()
            .map(|tagelem| tagelem[0].clone())
            .collect();

        assert_eq!(expected, parsed);
    }

    #[test]
    fn query_with_tag_only_retrieves_tagged_datapoint() {
        let datastore = Datastore::new();

        datastore.add_datapoint("information +with +tags");
        datastore.add_datapoint("information +different");

        let (query_result, parsed) = datastore.query("+different");

        assert_eq!(1, query_result.len());
    }

    #[test]
    fn query_with_word_only_retrieves_tagged_datapoint() {
        let datastore = Datastore::new();

        datastore.add_datapoint("information +with +tags");
        datastore.add_datapoint("information +different");

        let (query_result, parsed) = datastore.query("different");

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
