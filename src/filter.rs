use crate::todo::Todo;
use crate::query::Query;

pub fn filter(query: &Query, todos: &mut Vec<Todo>) {
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
