use crate::datapoint::{create_datapoint, Datapoint};
use std::sync::Mutex;

pub struct Datastore {
    datapoints: Mutex<Vec<Datapoint>>,
    tags: Mutex<Vec<String>>,
    counter: Mutex<u64>,
}

impl Datastore {
    pub fn new() -> Datastore {
        Datastore {
            datapoints: Mutex::new(Vec::new()),
            tags: Mutex::new(Vec::new()),
            counter: Mutex::new(0),
        }
    }

    pub fn add_datapoint(&self, input: &str) -> Datapoint {
        let mut new_datapoint = create_datapoint(input);
        self.append_tags(new_datapoint.get_tags());
        let counter = self.increment_counter();
        new_datapoint.set_key(counter);
        let mut old_datapoints = self.datapoints.lock().expect("mutex holder crashed");

        let length = old_datapoints.len();

        if length == 0 {
            old_datapoints.push(new_datapoint.clone());
            return new_datapoint;
        } else {
            let mut check_position = length;
            while check_position > 0 {
                check_position -= 1;
                if old_datapoints[check_position].get_datetime() < new_datapoint.get_datetime() {
                    old_datapoints.insert(check_position + 1, new_datapoint.clone());
                    return new_datapoint;
                }
            }
            old_datapoints.insert(0, new_datapoint.clone());
            return new_datapoint;
        }
    }

    pub fn update_datapoint(&self, input: &str, key: u64) -> Datapoint {
        let mut new_datapoint = create_datapoint(input);
        self.append_tags(new_datapoint.get_tags());
        new_datapoint.set_key(key);
        let mut datapoint_vector = self.datapoints.lock().expect("mutex holder crashed");
        for i in 0..datapoint_vector.len() {
            if datapoint_vector[i].get_key() == key {
                datapoint_vector[i] = new_datapoint.clone();
                break;
            }
        }
        return new_datapoint;
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

    fn increment_counter(&self) -> u64 {
        let mut counter = self.counter.lock().expect("counter holder crashed");
        *counter += 1;
        *counter
    }
}

impl From<Vec<Datapoint>> for Datastore {
    fn from(datapoints: Vec<Datapoint>) -> Datastore {
        let mut max_key = 0;
        for datapoint in datapoints.clone() {
            let key = datapoint.get_key();
            if max_key < key {
                max_key = key;
            }
        }
        let datastore = Datastore {
            datapoints: Mutex::new(datapoints.clone()),
            tags: Mutex::new(Vec::new()),
            counter: Mutex::new(max_key),
        };
        for datapoint in datapoints {
            datastore.append_tags(datapoint.get_tags());
        }
        return datastore;
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
    fn datapoints_can_be_updated_based_on_key() {
        let datastore = Datastore::new();
        datastore.add_datapoint("A datapoint +tag");
        datastore.add_datapoint("Another datapoint +another");

        datastore.update_datapoint("Different information +different", 1);

        assert_eq!(
            datastore.retrieve_datapoints()[0].get_data(),
            &"Different information".to_string()
        );
    }

    #[test]
    fn final_datapoint_can_also_be_updated_based_on_key() {
        let datastore = Datastore::new();
        datastore.add_datapoint("A datapoint +tag");
        datastore.add_datapoint("Another datapoint +another");
        datastore.add_datapoint("Even more +more");

        datastore.update_datapoint("Different information +different", 3);

        assert_eq!(
            datastore.retrieve_datapoints()[2].get_data(),
            &"Different information".to_string()
        );
    }

    #[test]
    fn from_a_vector_of_datapoints_a_datastore_is_born() {
        let mut datapoints = Vec::new();
        datapoints.push(datapoint::create_datapoint("some stuff +tag"));
        datapoints.push(datapoint::create_datapoint("more stuff +another +stuff"));
        datapoints.push(datapoint::create_datapoint("whatever +whatever +tag"));
        datapoints[0].set_key(1);
        datapoints[1].set_key(2);
        datapoints[2].set_key(3);

        let datastore = Datastore::from(datapoints.clone());

        assert_eq!(datastore.datapoints.lock().unwrap()[0], datapoints[0]);
        assert_eq!(datastore.datapoints.lock().unwrap()[1], datapoints[1]);
        assert_eq!(datastore.datapoints.lock().unwrap()[2], datapoints[2]);
        assert_eq!(
            *datastore.tags.lock().unwrap(),
            vec![
                "tag".to_string(),
                "another".to_string(),
                "stuff".to_string(),
                "whatever".to_string()
            ]
        );
        assert_eq!(*datastore.counter.lock().unwrap(), 3);
    }

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
    fn data_added_tagged_at_various_times_are_added_in_the_expected_locations() {
        let datastore = Datastore::new();
        datastore.add_datapoint("80kg +weight +DATE:2023-10-3");
        datastore.add_datapoint("81kg +weight +DATE:2023-10-2");
        datastore.add_datapoint("79kg +weight +DATE:2023-10-1");
        datastore.add_datapoint("83kg +weight +DATE:2023-10-5");
        datastore.add_datapoint("82kg +weight +DATE:2023-10-4");

        let datapoints = datastore.retrieve_datapoints();

        println!(
            "{:?}",
            datapoints
                .clone()
                .into_iter()
                .map(|point| (point.get_key(), point.get_data().to_owned()))
                .collect::<Vec<(u64, String)>>()
        );
        assert_eq!(datapoints[0].get_data(), "79kg");
        assert_eq!(datapoints[1].get_data(), "81kg");
        assert_eq!(datapoints[2].get_data(), "80kg");
        assert_eq!(datapoints[3].get_data(), "82kg");
        assert_eq!(datapoints[4].get_data(), "83kg");
    }

    #[test]
    fn value_command_in_query_strips_non_numeric_information() {
        let datastore = Datastore::new();
        datastore.add_datapoint("80kg +weight");
        datastore.add_datapoint("8kg +curl");

        let (retrieved, _) = datastore.query("curl:value");

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

        let (query_result, _) = datastore.query("+different");

        assert_eq!(1, query_result.len());
    }

    #[test]
    fn query_with_word_only_retrieves_tagged_datapoint() {
        let datastore = Datastore::new();

        datastore.add_datapoint("information +with +tags");
        datastore.add_datapoint("information +different");

        let (query_result, _) = datastore.query("different");

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
