use chrono::prelude::*;

use crate::{datapoint::Datapoint, parsedquery::ParsedQuery};

pub struct QueryResult {
    datapoints: Vec<Datapoint>,
    query: ParsedQuery,
}

impl QueryResult {
    pub fn from(datapoints: Vec<Datapoint>, query: ParsedQuery) -> QueryResult {
        QueryResult { datapoints, query }
    }

    pub fn apply_query_commands(self) -> QueryResult {
        let mut transformed = self.datapoints;
        for element in self.query.get_raw_parsed() {
            if element.len() > 1 {
                transformed = apply_command(transformed, element);
            }
        }
        QueryResult::from(transformed, self.query)
    }

    pub fn get_datapoints(&self) -> Vec<Datapoint> {
        self.datapoints.clone()
    }

    pub fn get_query(&self) -> ParsedQuery {
        self.query.clone()
    }

    pub fn get_numeric_data(&self) -> Vec<f64> {
        let data = self.get_datapoints();

        let mut number_collector = Vec::new();
        for datapoint in data {
            match datapoint.get_as_numeric() {
                Ok(num) => {
                    number_collector.push(num);
                }
                Err(_) => (),
            };
        }

        return number_collector;
    }

    pub fn get_date_numeric_data(&self) -> Option<Vec<(DateTime<Local>, f64)>> {
        let datapoints = self.get_datapoints();
        if datapoints.len() == 0 {
            return None;
        }
        let mut collector: Vec<(DateTime<Local>, f64)> = Vec::new();
        for datapoint in datapoints {
            let datetime = datapoint.get_datetime().to_owned();
            let value = match datapoint.get_as_numeric() {
                Ok(value) => value,
                Err(_) => return None,
            };
            collector.push((datetime, value));
        }
        return Some(collector);
    }
}

fn apply_command(datapoints: Vec<Datapoint>, command: Vec<String>) -> Vec<Datapoint> {
    match command[1].to_lowercase().as_str() {
        "date" => select_for_date(datapoints, command),
        "value" => strip_non_numeric(datapoints),
        "exclude" => remove_where_tag(datapoints, command[0].clone()),
        _ => datapoints,
    }
}

fn select_for_date(datapoints: Vec<Datapoint>, command: Vec<String>) -> Vec<Datapoint> {
    if command.len() < 4 {
        return datapoints;
    }
    match command[2].to_lowercase().as_str() {
        "start" => from_date(datapoints, command[3].clone(), true),
        "from" => from_date(datapoints, command[3].clone(), true),
        "till" => from_date(datapoints, command[3].clone(), false),
        "until" => from_date(datapoints, command[3].clone(), false),
        "end" => from_date(datapoints, command[3].clone(), false),
        _ => datapoints,
    }
}

fn from_date(datapoints: Vec<Datapoint>, date: String, return_before: bool) -> Vec<Datapoint> {
    let date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => return datapoints,
    };
    let datetime = if return_before {
        date.and_hms_opt(0, 0, 0).expect("00:00:00 is invalid")
    } else {
        date.and_hms_opt(23, 59, 59).expect("23:59:59 is invalid")
    };
    let mut location = 0;
    while location < datapoints.len()
        && datapoints[location].get_datetime().naive_local() < datetime
    {
        location += 1;
    }
    let mut before = datapoints.clone();
    let after = before.drain(0..location).collect();
    if return_before {
        return before;
    } else {
        return after;
    };
}

fn strip_non_numeric(datapoints: Vec<Datapoint>) -> Vec<Datapoint> {
    datapoints
        .into_iter()
        .map(|point| point.get_non_numeric_stripped())
        .collect()
}

fn remove_where_tag(datapoints: Vec<Datapoint>, tag: String) -> Vec<Datapoint> {
    let mut collector = Vec::new();
    for datapoint in datapoints {
        if !datapoint.get_tags().contains(&tag) {
            collector.push(datapoint);
        }
    }
    return collector;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datastore::Datastore;

    #[test]
    fn getting_date_numeric_returns_a_vector_of_associated_datetime_value_tuples() {
        let datastore = Datastore::new();
        datastore.add_datapoint("1 +value +DATE:2023-10-10");
        datastore.add_datapoint("2 +value +DATE:2023-10-11");
        datastore.add_datapoint("3 +value +DATE:2023-10-14");
        let queryresult = datastore.query("+value");

        let data = queryresult.get_date_numeric_data().unwrap();

        assert_eq!(data[0].1, 1.0);
        assert_eq!(data[1].1, 2.0);
        assert_eq!(data[2].1, 3.0);
    }

    #[test]
    fn getting_date_numeric_for_invalid_data_returns_none() {
        let datastore = Datastore::new();
        datastore.add_datapoint("11.1.4 +value +DATE:2023-10-10");
        datastore.add_datapoint("0.1 +value +DATE:2023-10-11");
        datastore.add_datapoint("8.9 +value +DATE:2023-10-14");
        let queryresult = datastore.query("+value");

        let data = queryresult.get_date_numeric_data();

        assert_eq!(data, None);
    }

    #[test]
    fn getting_date_numeric_for_non_numeric_data_returns_none() {
        let datastore = Datastore::new();
        datastore.add_datapoint("words and not numbers +value +DATE:2023-10-10");
        datastore.add_datapoint("more words +value +DATE:2023-10-11");
        datastore.add_datapoint("where are the numbers +value +DATE:2023-10-14");
        let queryresult = datastore.query("+value");

        let data = queryresult.get_date_numeric_data();

        assert_eq!(data, None);
    }

    #[test]
    fn getting_date_numeric_for_an_empty_query_returns_none() {
        let datastore = Datastore::new();
        datastore.add_datapoint("32.1 +value +DATE:2023-10-10");
        datastore.add_datapoint("6 +value +DATE:2023-10-11");
        datastore.add_datapoint("8.1 +value +DATE:2023-10-14");
        let queryresult = datastore.query("+invalid");

        let data = queryresult.get_date_numeric_data();

        assert_eq!(data, None);
    }

    #[test]
    fn malformed_date_in_query_returns_all() {
        let datastore = Datastore::new();
        datastore.add_datapoint("one +DATE:2023-10-10");
        datastore.add_datapoint("two +DATE:2023-10-11");
        datastore.add_datapoint("three +DATE:2023-10-14");
        let datapoints = datastore.retrieve_datapoints();

        let queryresult = datastore.query("*:DATE:FROM:2023-qwoop-10");
        let queryresult_datapoints = queryresult.get_datapoints();

        assert_eq!(queryresult_datapoints, datapoints);
    }

    #[test]
    fn incorrect_from_specifier_in_query_returns_all() {
        let datastore = Datastore::new();
        datastore.add_datapoint("one +DATE:2023-10-10");
        datastore.add_datapoint("two +DATE:2023-10-11");
        datastore.add_datapoint("three +DATE:2023-10-14");
        let datapoints = datastore.retrieve_datapoints();

        let queryresult = datastore.query("*:DATE:FRUMPL:2023-10-11");
        let queryresult_datapoints = queryresult.get_datapoints();

        assert_eq!(queryresult_datapoints, datapoints);
    }

    #[test]
    fn incorrect_date_selector_in_query_returns_all() {
        let datastore = Datastore::new();
        datastore.add_datapoint("one +DATE:2023-10-10");
        datastore.add_datapoint("two +DATE:2023-10-11");
        datastore.add_datapoint("three +DATE:2023-10-14");
        let datapoints = datastore.retrieve_datapoints();

        let queryresult = datastore.query("*:DATE:2023-10-11");
        let queryresult_datapoints = queryresult.get_datapoints();

        assert_eq!(queryresult_datapoints, datapoints);
    }

    #[test]
    fn from_selector_in_query_allows_selection_based_on_date() {
        let datastore = Datastore::new();
        datastore.add_datapoint("one +DATE:2023-10-10");
        datastore.add_datapoint("two +DATE:2023-10-11");
        datastore.add_datapoint("three +DATE:2023-10-14");
        let datapoints = datastore.retrieve_datapoints();

        let queryresult = datastore.query("*:DATE:FROM:2023-10-11");
        let queryresult_datapoints = queryresult.get_datapoints();

        assert_eq!(queryresult_datapoints[0], datapoints[1]);
        assert_eq!(queryresult_datapoints[1], datapoints[2]);
    }

    #[test]
    fn different_selectors_allow_date_before_selection_like_from() {
        let datastore = Datastore::new();
        datastore.add_datapoint("one +DATE:2023-10-10");
        datastore.add_datapoint("two +DATE:2023-10-12");
        datastore.add_datapoint("three +DATE:2023-10-14");

        let queryresult = datastore.query("*:DATE:FROM:2023-10-11");
        let datapoints_from = queryresult.get_datapoints();
        let queryresult = datastore.query("*:DATE:START:2023-10-11");
        let datapoints_start = queryresult.get_datapoints();

        assert_eq!(datapoints_from, datapoints_start);
    }

    #[test]
    fn until_selector_in_query_allows_selection_based_on_date() {
        let datastore = Datastore::new();
        datastore.add_datapoint("one +DATE:2023-10-10");
        datastore.add_datapoint("two +DATE:2023-10-12");
        datastore.add_datapoint("three +DATE:2023-10-13");
        let datapoints = datastore.retrieve_datapoints();

        let queryresult = datastore.query("*:DATE:UNTIL:2023-10-13");
        let queryresult_datapoints = queryresult.get_datapoints();

        assert_eq!(queryresult_datapoints[0], datapoints[0]);
        assert_eq!(queryresult_datapoints[1], datapoints[1]);
    }

    #[test]
    fn different_selectors_in_query_allows_selection_like_until() {
        let datastore = Datastore::new();
        datastore.add_datapoint("one +DATE:2023-10-10");
        datastore.add_datapoint("two +DATE:2023-10-12");
        datastore.add_datapoint("three +DATE:2023-10-14");

        let queryresult = datastore.query("*:DATE:UNTIL:2023-10-13");
        let datapoints_until = queryresult.get_datapoints();
        let queryresult = datastore.query("*:DATE:END:2023-10-13");
        let datapoints_end = queryresult.get_datapoints();
        let queryresult = datastore.query("*:DATE:TILL:2023-10-13");
        let datapoints_till = queryresult.get_datapoints();

        assert_eq!(datapoints_until, datapoints_end);
        assert_eq!(datapoints_until, datapoints_till);
    }

    #[test]
    fn get_datapoints_returns_a_vector_of_contained_datapoints() {
        let datastore = Datastore::new();
        datastore.add_datapoint("something +tag");
        datastore.add_datapoint("another +tag");
        let datapoints = datastore.retrieve_datapoints();

        let queryresult = datastore.query("tag");
        let queryresult_datapoints = queryresult.get_datapoints();

        assert_eq!(queryresult_datapoints, datapoints);
    }

    #[test]
    fn get_query_returns_the_parsed_query() {
        let datastore = Datastore::new();
        datastore.add_datapoint("something +tag");
        datastore.add_datapoint("another +tag");

        let queryresult = datastore.query("tag");
        let query = queryresult.get_query().get_raw_parsed();

        assert_eq!(query, vec![vec!["tag".to_string()]]);
    }

    #[test]
    fn apply_command_with_unknown_command_returns_full_datapoint_vector() {
        let datastore = Datastore::new();
        datastore.add_datapoint("cool information +tag");
        datastore.add_datapoint("More cool information +tag");
        let datapoints = datastore.retrieve_datapoints();

        let datapoints_after_command = apply_command(
            datapoints.clone(),
            vec!["Whatever".to_string(), "Unknown".to_string()],
        );

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
    fn value_command_strips_non_numeric_information_from_data() {
        let datastore = Datastore::new();
        datastore.add_datapoint("80kg +weight");

        let datapoints = datastore.retrieve_datapoints();
        let valuestripped =
            apply_command(datapoints, vec!["weight".to_string(), "value".to_owned()]);

        assert_eq!(valuestripped[0].get_data(), "80");
    }
}
