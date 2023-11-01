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
                transformed = apply_command(transformed, element[1].clone(), element[0].clone());
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
}

fn apply_command(datapoints: Vec<Datapoint>, command: String, tag: String) -> Vec<Datapoint> {
    match command.as_str() {
        "value" => strip_non_numeric(datapoints),
        "exclude" => remove_where_tag(datapoints, tag),
        _ => datapoints,
    }
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
            "Unknown".to_string(),
            "Whatever".to_string(),
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
        let valuestripped = apply_command(datapoints, "value".to_owned(), "weight".to_string());

        assert_eq!(valuestripped[0].get_data(), "80");
    }
}
