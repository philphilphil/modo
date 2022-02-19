use crate::predicate::{Operator, Property};
use crate::query::Query;
use crate::todo::Todo;

pub fn filter(query: &Query, todos: &mut Vec<Todo>) {
    for clause in &query.predicates {
        match clause.property {
            Property::Heading => match clause.operator {
                Operator::Contains => todos.retain(|t| t.heading.contains(&clause.expr_string)),
                Operator::DoesNotContain => todos.retain(|t| !t.heading.contains(&clause.expr_string)),
                Operator::Equals => todos.retain(|t| t.heading == clause.expr_string),
                Operator::DoesNotEqual => todos.retain(|t| t.heading != clause.expr_string),
            },
            Property::Path => match clause.operator {
                Operator::Contains => todos.retain(|t| t.filepath_as_string.contains(&clause.expr_string)),
                Operator::DoesNotContain => todos.retain(|t| !t.filepath_as_string.contains(&clause.expr_string)),
                Operator::Equals => todos.retain(|t| t.filepath_as_string == clause.expr_string),
                Operator::DoesNotEqual => todos.retain(|t| t.filepath_as_string != clause.expr_string),
            },
            Property::FileName => match clause.operator {
                Operator::Contains => todos.retain(|t| t.filename.contains(&clause.expr_string)),
                Operator::DoesNotContain => todos.retain(|t| !t.filename.contains(&clause.expr_string)),
                Operator::Equals => todos.retain(|t| t.filename == clause.expr_string),
                Operator::DoesNotEqual => todos.retain(|t| t.filename != clause.expr_string),
            },
            Property::Name => match clause.operator {
                Operator::Contains => todos.retain(|t| t.filter_name.contains(&clause.expr_string)),
                Operator::DoesNotContain => todos.retain(|t| !t.filter_name.contains(&clause.expr_string)),
                Operator::Equals => todos.retain(|t| t.filter_name == clause.expr_string),
                Operator::DoesNotEqual => todos.retain(|t| t.filter_name != clause.expr_string),
            },
            Property::Done => match clause.operator {
                Operator::Equals => todos.retain(|t| t.done == clause.expr_bool),
                Operator::DoesNotEqual => todos.retain(|t| t.done != clause.expr_bool),
                _ => {}
            },
        }
    }
}
