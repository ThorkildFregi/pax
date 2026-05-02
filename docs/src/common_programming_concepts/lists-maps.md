# Lists And Maps

Pax provides two powerful built-in data structures to organize and manage your data efficiently: Lists (ordered sequences) and Maps (key-value pairs).

## Lists

Lists in Pax are dynamic, ordered collections of values. They are ideal for storing sequences of data that you need to access by index or iterate over.

### Definition

Define a list using square brackets []:

Example:
```pax
var my_list = [0, 1, 2, 3, 4];
```

Note: Lists in Pax are zero-indexed. This means indices range from ```0``` to ```len - 1```.

### Accessing & Modifying Elements

To access or modify an element, use the list name followed by the index in square brackets:

Example:
```pax
my_list[2] = 5; // Changes the third element to 5
```

### Adding & Removing Elements

Need to resize your list? We've got you covered. Use the ```append``` keyword to add an element to the end, or ```pop``` to remove the last one.

Examples:

```pax
var my_list = [0, 1, 2];

my_list.append(3); // my_list is now [0, 1, 2, 3]
my_list.pop();     // my_list is now [0, 1, 2]
```