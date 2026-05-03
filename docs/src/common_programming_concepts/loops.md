# Loops

Do you want to repeat the same line for an eternity for no particular reason? First, you're weird. Second, you want to do something every developer has already done. Third, you can in Pax! Pax provides two main ways to repeat logic: ```while``` and ```for```.

## While

Do you know [```If```](./conditions.md)? Then, you know ```while```! It is just an ```if``` that repeats its code until the condition is ```false```.

Example:

```pax
var fuel = 3;

while fuel > 0 {
    print "Engine running...";
    fuel = fuel - 1;
}

print "Out of gas.";
```

Warning: If your condition is always ```true``` (like ```while 1 == 1```), you've just created an infinite loop. Welcome to the "weird" club.

## For

You do not like chaos. ```For``` is here to help you. The ```for``` loop is designed to iterate through a collection (like a [List](./lists-maps.md)). It automatically creates a local variable that takes the value of each element in the sequence.

Example:

```pax
var names = ["Alice", "Bob", "Charlie"];

for name in names {
    print("Hello, " + name);
}
```

Note: The loop variable (e.g., ```name```) only exists inside the loop's curly braces.