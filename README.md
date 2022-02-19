# modo
markdown todo cli interface is a small application to query markdown todos with a code like syntax.
This is my first project in rust and will be improved over time. Some parts may be implemented not ideal.

## Usage
call modo with a path to a folder where the markdown files are.
- modo /User/phil/Notes

From there, you can enter queries to get lists of todos.
Navigation in todo list is done via arrow keys. Pressing enter or space in a todolist will check/uncheck the selected todo. 
Pressing ESC gets you back to the query prompt.

## Filter Queries
If no filter for "done" is found in the query, "done equals false" will automatically be added.
An empty query will return all open todos. All filters are case insentivite.
Multiple filters can be used with "and" between them.

Syntax has to be: \<todo-property\> \<operator\> \<value\>

### Properties
Filters always bear upon a single todo.
- name (entire line of the todo)
- filename (file where the todo is located)
- path (full path, including filename)
- heading (First heading (line that starts with #) that is found above the todo)
- done (if todo is marked as done)

### Operators
- == equals
- != does not equal
- << contains 
- !< does not contain

### Examples
- done == false (same as no query)
- path << customername and done == true
- done == true and path !< "Arbeit"
- done == false and heading == "Daily"
- heading == "Daily" (same result as query overhead)
- path << "Work" and path !< "customer"

## Feature ideas
- Ordering todos via query 
- Give feedback about query parts that are wrong
- Shortcut to open the .md file a todo is in the systems set .md editor
- Ability to save queries and quickly execute them via shortcut
- Grouping and OR support for filters
- Display options, specify what properties of the todo should be displayed