use crate::datapoint::{create_datapoint, Datapoint};
use crate::parsedquery::ParsedQuery;
use crate::queryresult::QueryResult;
use std::sync::{Mutex, MutexGuard};

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
        insert_sorted_by_time(new_datapoint.clone(), &mut old_datapoints);
        return new_datapoint;
    }

    pub fn update_datapoint(&self, input: &str, key: u64) -> Datapoint {
        let mut new_datapoint = create_datapoint(input);
        self.append_tags(new_datapoint.get_tags());
        new_datapoint.set_key(key);
        let mut datapoint_vector = self.datapoints.lock().expect("mutex holder crashed");
        for i in 0..datapoint_vector.len() {
            if datapoint_vector[i].get_key() == key {
                if datapoint_vector[i].get_datetime() == new_datapoint.get_datetime() {
                    datapoint_vector[i] = new_datapoint.clone();
                    break;
                } else {
                    datapoint_vector.remove(i);
                    insert_sorted_by_time(new_datapoint.clone(), &mut datapoint_vector);
                    break;
                }
            }
        }
        return new_datapoint;
    }

    pub fn delete_datapoint(&self, key: u64) -> Option<Datapoint> {
        let mut datapoints = self.datapoints.lock().expect("mutex holder crashed");
        for (i, datapoint) in datapoints.clone().into_iter().enumerate() {
            if datapoint.get_key() == key {
                return Some(datapoints.remove(i));
            }
        }
        return None;
    }

    pub fn retrieve_datapoints(&self) -> Vec<Datapoint> {
        let lock = self.datapoints.lock().expect("mutex holder crashed");
        lock.clone()
    }

    pub fn retrieve_taglist(&self) -> Vec<String> {
        let lock = self.tags.lock().expect("Mutex holder crashed...");
        lock.clone()
    }

    pub fn query(&self, query: &str) -> QueryResult {
        let mut collector: Vec<Datapoint> = Vec::new();
        let parsed: ParsedQuery = ParsedQuery::from(query);
        let datapoints: Vec<Datapoint> = self.retrieve_datapoints();
        if parsed.empty() {
            return QueryResult::from(datapoints, parsed);
        }
        for datapoint in datapoints {
            if parsed.can_all_be_found_in(datapoint.get_tags()) {
                collector.push(datapoint.clone());
            }
        }
        QueryResult::from(collector, parsed).apply_query_commands()
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

fn insert_sorted_by_time<'a>(datapoint: Datapoint, vector: &mut MutexGuard<'a, Vec<Datapoint>>) {
    let length = vector.len();

    if length == 0 {
        vector.push(datapoint.clone());
    } else {
        let mut check_position = length;
        while check_position > 0 {
            check_position -= 1;
            if vector[check_position].get_datetime() < datapoint.get_datetime() {
                vector.insert(check_position + 1, datapoint.clone());
                return ();
            }
        }
        vector.insert(0, datapoint.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::datapoint;

    use super::*;

    #[test]
    fn datapoints_can_be_deleted_based_on_key() {
        let datastore = Datastore::new();
        datastore.add_datapoint("data +one +two");
        datastore.add_datapoint("more +one");

        let deleted = datastore.delete_datapoint(1).unwrap();
        let remaining = datastore.retrieve_datapoints();

        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].get_data(), "more");
        assert_eq!(deleted.get_data(), "data");
    }

    #[test]
    fn deleting_based_on_non_accessible_key_returns_none() {
        let datastore = Datastore::new();
        datastore.add_datapoint("data +one +two");
        datastore.add_datapoint("more +one");

        let result = datastore.delete_datapoint(4);

        assert_eq!(result, None);
    }

    #[test]
    fn query_can_exclude_certain_tags() {
        let datastore = Datastore::new();
        datastore.add_datapoint("data +one +two");
        datastore.add_datapoint("more +one");

        let queryresult = datastore.query("one two:exclude");
        let found = queryresult.get_datapoints();

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].get_data(), "more");
    }

    #[test]
    fn datapoints_can_be_updated_based_on_key() {
        let datastore = Datastore::new();
        datastore.add_datapoint("A datapoint +tag +TIME:17-00-00");
        datastore.add_datapoint("Another datapoint +another +TIME:17-00-01");

        datastore.update_datapoint("Different information +different +TIME:17-00-00", 1);

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
    fn datapoints_can_be_updated_based_on_key_redoing_time_ordering() {
        let datastore = Datastore::new();
        datastore.add_datapoint("A datapoint +tag +TIME:17-00-00");
        datastore.add_datapoint("Another datapoint +another +TIME:17-00-01");

        datastore.update_datapoint("A datapoint +tag +TIME:17-00-02", 1);

        assert_eq!(
            datastore.retrieve_datapoints()[1].get_data(),
            &"A datapoint".to_string()
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

        let queryresult = datastore.query("+");
        let datapoints = queryresult.get_datapoints();

        assert_eq!("cool information", datapoints[0].get_data());
        assert_eq!("More cool information", datapoints[1].get_data());
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

        let queryresult = datastore.query("curl:value");
        let retrieved = queryresult.get_datapoints();

        assert_eq!(retrieved[0].get_data(), "8");
    }

    #[test]
    fn query_with_tag_only_retrieves_tagged_datapoint() {
        let datastore = Datastore::new();

        datastore.add_datapoint("information +with +tags");
        datastore.add_datapoint("information +different");

        let queryresult = datastore.query("+different");
        let query_result = queryresult.get_datapoints();

        assert_eq!(1, query_result.len());
    }

    #[test]
    fn query_with_word_only_retrieves_tagged_datapoint() {
        let datastore = Datastore::new();

        datastore.add_datapoint("information +with +tags");
        datastore.add_datapoint("information +different");

        let queryresult = datastore.query("different");
        let query_result = queryresult.get_datapoints();

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
