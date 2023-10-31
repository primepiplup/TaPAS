pub struct ParsedQuery {
    query: Vec<Vec<String>>,
}

impl From<Vec<Vec<String>>> for ParsedQuery {
    fn from(parsed_query: Vec<Vec<String>>) -> ParsedQuery {
        ParsedQuery {
            query: parsed_query,
        }
    }
}

impl ParsedQuery {
    pub fn generate_plot_title(&self) -> String {
        format!("Plot for: {}", self.collect_query())
    }

    pub fn collect_query(&self) -> String {
        self.query
            .clone()
            .into_iter()
            .map(|elem| elem[0].clone())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn empty(&self) -> bool {
        self.query.len() < 1 || self.query[0][0] == ""
    }

    pub fn can_all_be_found_in(&self, tags: &Vec<String>) -> bool {
        let truthvalues: Vec<bool> = self
            .query
            .clone()
            .into_iter()
            .map(|tag| tags.contains(&tag[0]))
            .collect();
        !truthvalues.contains(&false)
    }

    pub fn get_raw_parsed(&self) -> Vec<Vec<String>> {
        self.query.clone()
    }

    pub fn get_parsed_tags(&self) -> Vec<String> {
        self.query
            .clone()
            .into_iter()
            .map(|tagelem| tagelem[0].clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_plot_title_takes_all_elements_of_vector_and_returns_title() {
        let parsed = vec![
            vec!["something".to_string(), "value".to_string()],
            vec!["tag".to_string()],
            vec!["else".to_string()],
        ];
        let parsed = ParsedQuery::from(parsed);

        let title = parsed.generate_plot_title();

        assert_eq!(title, "Plot for: something, tag, else")
    }
}
