# modo
markdown todo cli interface

## Filter
### Properties
Filters always bear upon a todo.
- name (entire line of the todo)
- path (Full path starting from the given folder)
- heading (First heading (line that starts with #) that is found above the todo)
- done (if todo is marked as done)

### Operators
equals (e)
does not equal (ne)
contains (c)
does not contain (nc)

### Examples
- done equals false
- path contains customername and done equals false
- done e true and path nc "Arbeit"
- done e false and heading e "Daily"

