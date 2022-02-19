use crate::clause::{Operator, Property};
use crate::query::Query;
use crate::todo::Todo;

pub fn filter(query: &Query, todos: &mut Vec<Todo>) {
    for clause in &query.clauses {
        match clause.property {
            Property::Heading => match clause.operator {
                Operator::Contains => todos.retain(|t| t.heading.contains(&clause.value)),
                Operator::DoesNotContain => todos.retain(|t| !t.heading.contains(&clause.value)),
                Operator::Equals => todos.retain(|t| t.heading == clause.value),
                Operator::DoesNotEqual => todos.retain(|t| t.heading != clause.value),
            },
            _ => {}
        }
    }

    // match &caps[1] {
    //     "path" => {
    //         todos = todos
    //             .into_iter()
    //             .filter(|t| t.filename.to_string().contains(&caps[3]))
    //             .collect()
    //     }
    //     "heading" => {
    //         todos = todos
    //             .into_iter()
    //             .filter(|t| {
    //                 t.first_heading
    //                     .to_lowercase()
    //                     .contains(&caps[3].to_lowercase())
    //             })
    //             .collect()
    //     }
}
