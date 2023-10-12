use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Datapoint {
    datetime: DateTime<Local>,
    data: String,
    tags: Vec<String>,
}

impl Datapoint {
    pub fn data_same_as(&self, other: &Datapoint) -> bool {
        self.data == other.data
    }

    pub fn tags_same_as(&self, other: &Datapoint) -> bool {
        self.tags == other.tags
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }

    pub fn get_datetime(&self) -> &DateTime<Local> {
        &self.datetime
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn strip_non_numeric(self) -> Datapoint {
        Datapoint {
            datetime: self.datetime,
            data: self.data.chars().filter(|c| c.is_digit(10)).collect(),
            tags: self.tags,
        }
    }
}

pub fn create_datapoint(text: &str) -> Datapoint {
    Datapoint {
        datetime: Utc::now().with_timezone(&Local),
        data: get_data_from(text),
        tags: get_tags_from(text),
    }
}

fn get_data_from(text: &str) -> String {
    match text.split('+').next() {
        Some(data) => data.trim().to_owned(),
        None => "".to_owned(),
    }
}

fn get_tags_from(text: &str) -> Vec<String> {
    let mut tag_iterator = text.split('+');
    if tag_iterator.next().is_none() {
        return Vec::new();
    } else {
        return collect_tags(tag_iterator.collect());
    }
}

fn collect_tags(tag_iterator: Vec<&str>) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();
    for tag in tag_iterator {
        if tag.starts_with(char::is_whitespace) {
            continue;
        };
        let tag = match tag.split_whitespace().next() {
            Some(text) => text,
            None => continue,
        };
        tags.push(tag.to_owned());
    }
    return tags;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_non_numeric_strips_non_numeric_data() {
        let datapoint = create_datapoint("some data 40 numbers");

        let transformed = datapoint.strip_non_numeric();

        assert_eq!(transformed.get_data(), "40");
    }

    #[test]
    fn two_datapoints_created_with_same_data_are_data_equal() {
        let datapoint_a = create_datapoint("same data +with +tags");
        let datapoint_b = create_datapoint("same data +different +tags +asdfasdf");
        assert!(datapoint_a.data_same_as(&datapoint_b));
    }

    #[test]
    fn two_datapoints_created_with_same_tags_are_tag_equal() {
        let datapoint_a = create_datapoint("different data +same +tags");
        let datapoint_b = create_datapoint(
            "waaay differnet data aaa totally different +same +tags   accidental input",
        );
        assert!(datapoint_a.tags_same_as(&datapoint_b));
    }

    #[test]
    fn all_text_preceding_plus_becomes_data() {
        let expected_data = "text preceding";

        let data = get_data_from("text preceding +some +tags");

        assert_eq!(data, expected_data);
    }

    #[test]
    fn tags_are_extracted() {
        let mut expected_tags: Vec<&str> = Vec::new();
        expected_tags.push("tag");
        expected_tags.push("another");

        let tags = get_tags_from("+tag +another");

        assert_eq!(tags, expected_tags);
    }

    #[test]
    fn string_without_tags_returns_empty_vector() {
        let expected_tags: Vec<&str> = Vec::new();

        let tags =
            get_tags_from("some text with ^ wacky @ things 1234 inside but no plus *( signs)");

        assert_eq!(tags, expected_tags);
    }

    #[test]
    fn double_tags_are_ignored() {
        let mut expected_tags: Vec<&str> = Vec::new();
        expected_tags.push("tags");
        expected_tags.push("mistake");

        let tags = get_tags_from("a string with +tags ++mistake");

        assert_eq!(tags, expected_tags);
    }

    #[test]
    fn empty_tags_are_ignored() {
        let mut expected_tags: Vec<&str> = Vec::new();
        expected_tags.push("tags");
        expected_tags.push("mistake");

        let tags = get_tags_from("a string with +tags   +    +mistake   ");

        assert_eq!(tags, expected_tags);
    }

    #[test]
    fn words_not_directly_attached_to_plus_are_ignored() {
        let mut expected_tags: Vec<&str> = Vec::new();
        expected_tags.push("tags");
        expected_tags.push("mistake");

        let tags = get_tags_from("a string with +tags   +  some stuff  +mistake   ");

        assert_eq!(tags, expected_tags);
    }
}
