# modo
markdown todo cli interface is a smal application to query markdown todos with a sql like syntax.

## Filter Queries
If no filter for "done" is found in the query, "done equals false" will automatically be added.
An empty query will return all open todos.

### Properties
Filters always bear upon a todo.
- name (entire line of the todo)
- filename
- path (Full path starting from the given folder)
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

## Display options (TBD)
You can specify what information about the todo you want to see with the "select" operator, it does not matter if its before or after the filter query, but they have to be seperated with a semicolon. The properties are the same as in the filters.

Default display is: Name, filename

### Operators



### Examples
- select name path heading done where heading e "Daily"