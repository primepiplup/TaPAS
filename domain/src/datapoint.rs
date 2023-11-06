use std::num::ParseFloatError;

use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Datapoint {
    datetime: DateTime<Local>,
    data: String,
    tags: Vec<String>,
    key: u64,
}

impl Datapoint {
    pub fn new(datetime: DateTime<Local>, data: String, tags: Vec<String>, key: u64) -> Datapoint {
        Datapoint {
            datetime,
            data,
            tags,
            key,
        }
    }

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

    pub fn get_non_numeric_stripped(self) -> Datapoint {
        Datapoint {
            datetime: self.datetime,
            data: self.strip_non_numeric(),
            tags: self.tags,
            key: self.key,
        }
    }

    pub fn get_as_numeric(&self) -> Result<f64, ParseFloatError> {
        let num = self.strip_non_numeric();
        num.parse()
    }

    pub fn set_key(&mut self, key: u64) -> () {
        self.key = key;
    }

    pub fn get_key(&self) -> u64 {
        self.key
    }

    pub fn add_tag(&mut self, tag: &String) -> () {
        self.tags.push(tag.clone());
    }

    pub fn remove_tag(&mut self, tag: &String) -> () {
        let mut i = 0;
        while i < self.tags.len() {
            if &self.tags[i] == tag {
                self.tags.remove(i); // this mutates the vector... after one removal, 'i' values will not be accurate anymore...
            } else {
                i += 1;
            }
        }
    }

    fn strip_non_numeric(&self) -> String {
        self.data
            .chars()
            .filter(|c| c.is_digit(10) || c == &'.')
            .collect()
    }
}

pub fn create_datapoint(text: &str) -> Datapoint {
    let tags = get_tags_from(text);
    let data = get_data_from(text);
    handle_tags_and_create_datapoint(data, tags)
}

fn handle_tags_and_create_datapoint(data: String, tags: Vec<String>) -> Datapoint {
    let mut datetime: DateTime<Local> = Local::now();
    let mut date = datetime.date_naive();
    let mut time = datetime.time();

    let mut tag_collector = Vec::new();

    for tag in &tags {
        let command: Vec<&str> = tag.split(':').collect();
        match command[0] {
            "D" => date = parse_date(command, date),
            "DATE" => date = parse_date(command, date),
            "T" => time = parse_time(command, time),
            "TIME" => time = parse_time(command, time),
            non_command => tag_collector.push(non_command.to_string()),
        }
    }

    datetime = Local.from_local_datetime(&date.and_time(time)).unwrap();

    Datapoint {
        data,
        tags: tag_collector,
        datetime,
        key: 0,
    }
}

fn parse_date(command: Vec<&str>, fallback: NaiveDate) -> NaiveDate {
    if command.len() < 2 {
        return fallback;
    };
    match NaiveDate::parse_from_str(command[1], "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => fallback,
    }
}

fn parse_time(command: Vec<&str>, fallback: NaiveTime) -> NaiveTime {
    if command.len() < 2 {
        return fallback;
    };
    match NaiveTime::parse_from_str(command[1], "%H-%M-%S") {
        Ok(date) => date,
        Err(_) => fallback,
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
    fn get_data_from_text_will_return_empty_string_if_nothing_entered() {
        let data = get_data_from("+");

        assert_eq!(data, "");
    }

    #[test]
    fn using_d_tag_followed_by_date_returns_datapoint_with_datetime_of_said_date() {
        let expected = NaiveDate::from_ymd_opt(2022, 2, 10).unwrap();

        let datapoint = create_datapoint("Some data tagged with +D:2022-02-10");

        assert_eq!(datapoint.get_datetime().date_naive(), expected);
    }

    #[test]
    fn using_date_tag_followed_by_date_returns_datapoint_with_datetime_of_said_date() {
        let expected = NaiveDate::from_ymd_opt(2022, 2, 10).unwrap();

        let datapoint = create_datapoint("Some data tagged with +DATE:2022-02-10");

        assert_eq!(datapoint.get_datetime().date_naive(), expected);
    }

    #[test]
    fn using_t_tag_followed_by_time_returns_datapoint_with_datetime_of_said_time() {
        let expected = NaiveTime::from_hms_opt(12, 34, 56).unwrap();

        let datapoint = create_datapoint("Some data tagged with +T:12-34-56");

        assert_eq!(datapoint.get_datetime().time(), expected);
    }

    #[test]
    fn using_time_tag_followed_by_time_returns_datapoint_with_datetime_of_said_time() {
        let expected = NaiveTime::from_hms_opt(12, 34, 56).unwrap();

        let datapoint = create_datapoint("Some data tagged with +TIME:12-34-56");

        assert_eq!(datapoint.get_datetime().time(), expected);
    }

    #[test]
    fn parse_date_falls_back_on_given_date_if_no_string_to_parse() {
        let command = vec!["DATE"];
        let fallback = NaiveDate::from_ymd_opt(2022, 10, 15).unwrap();

        let result = parse_date(command, fallback);

        assert_eq!(result, fallback);
    }

    #[test]
    fn parse_date_falls_back_on_given_date_if_parse_errors_out() {
        let command = vec!["DATE", "certainly not a date"];
        let fallback = NaiveDate::from_ymd_opt(2022, 10, 15).unwrap();

        let result = parse_date(command, fallback);

        assert_eq!(result, fallback);
    }

    #[test]
    fn parse_time_falls_back_on_given_time_if_no_string_to_parse() {
        let command = vec!["TIME"];
        let fallback = NaiveTime::from_hms_opt(12, 34, 56).unwrap();

        let result = parse_time(command, fallback);

        assert_eq!(result, fallback);
    }

    #[test]
    fn parse_time_falls_back_on_given_time_if_parse_errors_out() {
        let command = vec![
            "TIME",
            "text that will cause the parse to error out for sure",
        ];
        let fallback = NaiveTime::from_hms_opt(12, 34, 56).unwrap();

        let result = parse_time(command, fallback);

        assert_eq!(result, fallback);
    }

    #[test]
    fn strip_non_numeric_strips_non_numeric_data() {
        let datapoint = create_datapoint("some data 40 numbers");

        let transformed = datapoint.get_non_numeric_stripped();

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
