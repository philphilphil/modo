use crate::predicate::Predicate;
use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
pub struct Query {
    pub input_string: String,
    pub input_predicates: Vec<String>,
    pub predicates: Vec<Predicate>,
}

impl Query {
    pub fn new(input_string: &str) -> Query {
        Query {
            input_string: input_string.to_string(),
            predicates: vec![],
            input_predicates: input_string
                .split("and")
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        if self.input_string == "" {
            return Ok(());
        }

        for q in &self.input_predicates {
            // https://regex101.com/r/1g3YHS/1
            // Todo: split regex, done can only have == true/false
            let re = Regex::new("(done|path|filename|heading|name) (==|!=|<<|!<) (.*)").unwrap();

            if !re.is_match(&q) {
                return Err(anyhow!("Could not parse query."));
            }
            let caps = re.captures(&q).unwrap();
            let clause = Predicate::new(&caps[1], &caps[2], &caps[3])?;
            self.predicates.push(clause);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_query_failure() {
        let query = String::from("done /// false");
        let mut parsed_query = Query::new(&query);
        assert!(parsed_query.parse().is_err());
    }

    #[test]
    fn parse_query_success() {
        let query = String::from("done == false");
        let mut parsed_query = Query::new(&query);
        assert!(parsed_query.parse().is_ok());
    }
}
