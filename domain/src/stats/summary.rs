use crate::{datapoint::Datapoint, plotter::util::get_numeric_data, queryresult::QueryResult};

use super::stats::average;

pub struct Summary {
    name: String,
    mean: f64,
    p: f64,
}

impl Summary {
    pub fn set_name(self, title: String) -> Summary {
        Summary {
            name: title,
            mean: self.mean,
            p: self.p,
        }
    }

    pub fn set_p(self, p: f64) -> Summary {
        Summary {
            name: self.name,
            mean: self.mean,
            p,
        }
    }

    pub fn summaries_from(queryresults: Vec<QueryResult>) -> Vec<Summary> {
        let mut collector = Vec::new();
        for queryresult in queryresults {
            let title = queryresult.get_query().collect_query();
            let summary = Summary::from(queryresult.get_datapoints()).set_name(title);
            collector.push(summary);
        }
        return collector;
    }

    pub fn get_mean(&self) -> f64 {
        self.mean
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_p(&self) -> f64 {
        self.p
    }
}

impl From<Vec<Datapoint>> for Summary {
    fn from(datapoints: Vec<Datapoint>) -> Summary {
        let (_, data) = match get_numeric_data(&datapoints) {
            Some(value) => value,
            None => (Vec::new(), Vec::new()),
        };
        let mean = average(&data);
        Summary {
            name: "".to_string(),
            mean,
            p: 1.0,
        }
    }
}

impl From<Vec<f64>> for Summary {
    fn from(nums: Vec<f64>) -> Summary {
        let mean = average(&nums);
        Summary {
            name: "".to_string(),
            mean,
            p: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datastore::Datastore;

    #[test]
    fn empty_query_results_in_empty_summary() {
        let datastore = Datastore::new();
        datastore.add_datapoint("6 hours +coffee");
        datastore.add_datapoint("7 hours +coffee");
        datastore.add_datapoint("7 hours +tea");
        datastore.add_datapoint("8 hours +tea");
        let queryresult = datastore.query("cola");

        let summary = Summary::from(queryresult.get_datapoints());

        assert_eq!(summary.get_mean(), 0.0);
        assert_eq!(summary.get_name(), "".to_string());
        assert_eq!(summary.get_p(), 1.0);
    }

    #[test]
    fn vector_of_datapoints_can_be_summarized() {
        let datastore = Datastore::new();
        datastore.add_datapoint("6 hours +coffee");
        datastore.add_datapoint("7 hours +coffee");
        datastore.add_datapoint("7 hours +tea");
        datastore.add_datapoint("8 hours +tea");
        let queryresult = datastore.query("tea");

        let summary = Summary::from(queryresult.get_datapoints());

        assert_eq!(summary.get_mean(), 7.5);
    }

    #[test]
    fn vector_of_query_results_can_be_summarized() {
        let mut collector = Vec::new();
        let datastore = Datastore::new();
        datastore.add_datapoint("6 hours +coffee");
        datastore.add_datapoint("7 hours +coffee");
        datastore.add_datapoint("7 hours +tea");
        datastore.add_datapoint("8 hours +tea");
        collector.push(datastore.query("tea"));
        collector.push(datastore.query("coffee"));

        let summaries = Summary::summaries_from(collector);

        assert_eq!(summaries[0].get_mean(), 7.5);
        assert_eq!(summaries[0].get_name(), "tea".to_string());
        assert_eq!(summaries[1].get_mean(), 6.5);
        assert_eq!(summaries[1].get_name(), "coffee".to_string());
    }
}
