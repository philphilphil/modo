use crate::clause::Clause;
use regex::Regex;
#[derive(Debug)]
pub struct Query {
    pub input_string: String,
    pub input_clauses: Vec<String>,
    pub clauses: Vec<Clause>,
}

impl Query {
    pub fn new(input_string: &str) -> Query {
        Query {
            input_string: input_string.to_string(),
            clauses: vec![],
            input_clauses: input_string.split("and").map(|s| s.trim().to_string()).collect(),
        }
    }

    pub fn parse(&mut self) -> bool {
        if self.input_string == "" {
            return true;
        }

        for q in &self.input_clauses {
            // https://regex101.com/r/1g3YHS/1
            let re = Regex::new("(done|path|filename|heading|name) (==|<>|<<) (.*)").unwrap();

            if !re.is_match(&q) {
                return false;
            }
            let caps = re.captures(&q).unwrap();
            let clause = Clause::new(&caps[1], &caps[2], &caps[3]);
            self.clauses.push(clause);
        }

        true
    }
}
