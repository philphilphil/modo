use regex::Regex;

pub struct Query {
    pub input_string: String,
    pub input_clauses: Option<Vec<String>>,
    pub clauses: Option<Vec<Clause>>,
}

// Todo: rename to sth else?
pub struct Clause {
    pub property: Properties,
    pub operator: Operator,
    pub value: String,
}

pub enum Operator {
    Equals,
    DoesNotEqual,
    Contains,
    DoesNotContain,
}

pub enum Properties {
    Name,
    FileName,
    Path,
    Heading,
    Done,
}

impl Query {
    pub fn new(input_string: &str) -> Query {
        Query {
            input_string: input_string.to_string(),
            clauses: None,
            input_clauses: Some(input_string.split("and").map(|s| s.to_string()).collect()),
        }
    }

    pub fn is_valid_query(&self) -> bool {
        for q in self.input_clauses.as_ref().unwrap() {
            // https://regex101.com/r/1g3YHS/1
            let re = Regex::new("(done|path|filename|filepath|heading) (==|<>|<<) (.*)").unwrap();

            if !re.is_match(q) {
                return false;
            }
        }

        true
    }
}
