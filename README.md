# modo
markdown todo cli interface is a small application to query markdown todos with a SQL like syntax.

## Filter Queries
If no filter for "done" is found in the query, "done equals false" will automatically be added.
An empty query will return all open todos.
Multiple filters can be used with "and" between them.

### Properties
Filters always bear upon a single todo.
- name (entire line of the todo)
- filename (file where the todo is located)
- path (full path, including filename)
- heading (First heading (line that starts with #) that is found above the todo)
- done (if todo is marked as done)

### Operators
- equals (e)
- does not equal (ne)
- contains (c)
- does not contain (nc)
- order by desc (obd)
- order by asc (oba)

### Examples
- done equals false
- path contains customername and done equals false
- done e true and path nc "Arbeit"
- done e false and heading e "Daily"
- heading e "Daily" (same result as query overhead)
- path e "Work" oba filename

### Examples
- select name path heading done where heading e "Daily"

## Feature ideas
- Shortcut to open the .md file a todo is in the systems set .md editor
- Ability to save queries and quickly execute them via shortcut
- Grouping and OR support for filters
- Display options, specify what properties of the todo should be displayed