#[derive(Debug)]
pub struct Clause {
    pub property: Property,
    pub operator: Operator,
    pub value: String,
}

impl Clause {
    pub fn new(property: &str, operator: &str, value: &str) -> Clause {
        let mut clause = Clause::default();

        match property {
            "path" => clause.property = Property::Path,
            "heading" => clause.property = Property::Path,
            "filename" => clause.property = Property::Path,
            "name" => clause.property = Property::Path,
            "done" => clause.property = Property::Path,
            _ => panic!("{}", "Something went wrong here."),
        }

        match operator {
            "==" => clause.operator = Operator::Equals,
            "!=" => clause.operator = Operator::DoesNotEqual,
            "<<" => clause.operator = Operator::Contains,
            "<>" => clause.operator = Operator::DoesNotContain,
            _ => panic!("{}", "Something went wrong here."),
        }

        // TODO: check for "" and unwrap content
        clause.value = value.to_string();

        clause
    }
}

impl Default for Clause {
    fn default() -> Clause {
        Clause {
            operator: Operator::Equals,
            property: Property::Done,
            value: String::from("false"),
        }
    }
}
#[derive(Debug)]
pub enum Operator {
    Equals,
    DoesNotEqual,
    Contains,
    DoesNotContain,
}
#[derive(Debug)]
pub enum Property {
    Name,
    FileName,
    Path,
    Heading,
    Done,
}
