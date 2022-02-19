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
            input_clauses: input_string
                .split("and")
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }

    pub fn parse(&mut self) -> bool {
        if self.input_string == "" {
            self.clauses.push(Clause::default());
            return true;
        }

        for q in &self.input_clauses {
            // https://regex101.com/r/1g3YHS/1
            // Todo: split regex, done can only have == true/false
            let re = Regex::new("(done|path|filename|heading|name) (==|<>|<<|!=) (.*)").unwrap();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_query_failure() {
        let query = String::from("done /// false");
        let mut parsed_query = Query::new(&query);
        assert!(!parsed_query.parse());
    }

    #[test]
    fn parse_query_success() {
        let query = String::from("done == false");
        let mut parsed_query = Query::new(&query);
        assert!(parsed_query.parse());
    }
}
