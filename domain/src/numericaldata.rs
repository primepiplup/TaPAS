use crate::queryresult::QueryResult;

pub struct NumericalData {
    data: Vec<f64>,
    title: String,
}

impl From<QueryResult> for NumericalData {
    fn from(queryresult: QueryResult) -> NumericalData {
        let data = queryresult.get_numeric_data();
        let title = queryresult.get_query().generate_plot_title();
        NumericalData { data, title }
    }
}

impl From<&QueryResult> for NumericalData {
    fn from(queryresult: &QueryResult) -> NumericalData {
        let data = queryresult.get_numeric_data();
        let title = queryresult.get_query().collect_query();
        NumericalData { data, title }
    }
}

impl NumericalData {
    pub fn get_data(&self) -> Vec<f64> {
        self.data.clone()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() < 1
    }
}
