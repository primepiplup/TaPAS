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
            .map(|elem| {
                if elem.len() > 1 {
                    match elem[1].as_str() {
                        "exclude" => format!("without {}", elem[0]),
                        _ => elem[0].clone(),
                    }
                } else {
                    elem[0].clone()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn empty(&self) -> bool {
        self.query.len() < 1 || self.query[0][0] == ""
    }

    pub fn can_all_be_found_in(&self, datapoint_tags: &Vec<String>) -> bool {
        let searchtags: Vec<String> = self.without_excluded_tags();
        let truthvalues: Vec<bool> = searchtags
            .into_iter()
            .map(|tag| datapoint_tags.contains(&tag))
            .collect();
        !truthvalues.contains(&false)
    }

    fn without_excluded_tags(&self) -> Vec<String> {
        let mut collector: Vec<String> = Vec::new();
        for tag in self.query.clone() {
            if tag.len() > 1 {
                match tag[1].as_str() {
                    "exclude" => continue,
                    _ => collector.push(tag[0].clone()),
                };
            } else {
                collector.push(tag[0].clone());
            }
        }
        return collector;
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

    #[test]
    fn generate_plot_title_sees_exclude_command_and_changes_title_appropriately() {
        let parsed = vec![
            vec!["something".to_string(), "exclude".to_string()],
            vec!["tag".to_string()],
            vec!["else".to_string()],
        ];
        let parsed = ParsedQuery::from(parsed);

        let title = parsed.generate_plot_title();

        assert_eq!(title, "Plot for: without something, tag, else")
    }
}
