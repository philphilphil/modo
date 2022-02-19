#[derive(Debug)]
pub struct Predicate {
    pub property: Property,
    pub operator: Operator,
    pub expr_string: String,
    pub expr_bool: bool,
}

impl Predicate {
    pub fn new(property: &str, operator: &str, expression: &str) -> Predicate {
        let mut clause = Predicate::default();

        match property {
            "path" => clause.property = Property::Path,
            "heading" => clause.property = Property::Heading,
            "filename" => clause.property = Property::FileName,
            "name" => clause.property = Property::Name,
            "done" => clause.property = Property::Done,
            _ => panic!("Can't parse property."),
        }

        match operator {
            "==" => clause.operator = Operator::Equals,
            "!=" => clause.operator = Operator::DoesNotEqual,
            "<<" => clause.operator = Operator::Contains,
            "!<" => clause.operator = Operator::DoesNotContain,
            _ => panic!("Can't parse operator."),
        }

        // TODO: check for "" and remove
        clause.expr_string = expression.to_lowercase();

        if matches!(clause.property, Property::Done) {
            // TODO: Error handling
            clause.expr_bool = clause.expr_string.parse().unwrap()
        }

        clause
    }
}

impl Default for Predicate {
    fn default() -> Predicate {
        Predicate {
            property: Property::Done,
            operator: Operator::Equals,
            expr_string: String::from(""),
            expr_bool: false,
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
