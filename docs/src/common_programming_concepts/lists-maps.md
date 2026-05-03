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

Need to resize your list? We've got you covered. Use the ```append``` to add an element to the end, or ```pop``` to remove the last one.

Examples:

```pax
var my_list = [0, 1, 2];

my_list.append(3); // my_list is now [0, 1, 2, 3]
my_list.pop();     // my_list is now [0, 1, 2]
```

Note: The pop method can optionally accept an index as a parameter. If provided, it will remove the element at that specific index instead of the last one.

Example:

```pax
var list = ["a", "b", "c"];
list.pop(1); // Removes "b"
print(list);  // Output: ["a", "c"]
```

## Maps

Maps (also known as Dictionaries) are collections of key-value pairs. They are the perfect structure when you need to associate unique identifiers (keys) with specific data (values), allowing for lightning-fast lookups.

### Definition

In Pax, you define a Map using curly braces ```{}```. Every entry consists of a String key, a colon ```:```, and a Value.

```pax
var user = {
    "name": "Paximus",
    "level": 42,
    "is_pro": true
};
```

### Accessing and modifying

To get a value or update it, use the key inside square brackets ```[]```. If the key doesn't exist when assigning, Pax will automatically create it for you!

```pax
print(user["name"]);     // Output: Paximus

user["level"] = 43;     // Updates the value
user["city"] = "Paris"; // Adds a new key-value pair
```

### Removing elements

If you need to delete a key and its associated value, use the ```remove``` keyword followed by the key.

Example:

```pax
var settings = {
    "theme": "dark",
    "notifications": true,
    "font_size": 14
};

settings.remove("notifications"); 

print(settings); // Output: {"theme": "dark", "font_size": 14}
```