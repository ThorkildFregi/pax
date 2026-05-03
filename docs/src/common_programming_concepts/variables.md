# Variables

Variables are the most important concept in programming. They are the tools you use to store data so you can retrieve it later. There are three main actions you can perform with variables: declaring them, updating them, and calling them.

## Variable declaration

If you have read the previous chapters, you already know how to declare a variable. But since I know my fellow developers, and I know that "RTFM" (Read The Fucking Manual) is not exactly their favorite hobby, I will do a quick recap here.

To declare a variable, you need the keyword ```var```. Then, give it a cute little name. (Convention says you should name a variable based on what it does. Avoid ```a```, ```b```, ```c```, or even ```i_dont_know```, and yes, I have actually seen that before).

When declared, a variable can take any [data types](./data_types.md). Note that type detection is automatic, like in Python or Rust, unlike my nemesis: Java...

Examples:

```pax
var str_var = "A string for a var"; // A string variable
var int_var = 73;                   // An integer variable
var bool_var = true;                // A boolean variable
```

## Change a variable

Don't like the content of your variable? No problem! You can change its value at any moment. Just write its name, without the ```var``` keyword, and assign a new value.
Note: the new value must be of the same type as the original one. Otherwise, you will have to re-declare your variable using the ```var``` keyword. (I know, I’m being as annoying as Rust!)

Example:

```pax
var mood = "Unhappy"; // I had to read Java code
mood = "Happy"; // I can finally code in Pax
```

## Call a variable

Having a variable is cool, but using it is even cooler! You can call a variable in many situations: inside a ```print``` statement, during the declaration of another variable, or even within conditions. Let’s dive into some examples.

Examples:

```pax
var alpha = 10; // Declaring 'alpha' as an integer with a value of 10

var beta = alpha * 2; // Declaring 'beta' using the value of 'alpha' times 2

println(beta + alpha); // Printing the sum of beta and alpha

if alpha == beta { // Checking if alpha is equal to beta
    println("This is impossible!");
} else {
    println("This is possible!");
}
```

Note: Don't worry about the ```if``` and ```else``` for now, [we will explore conditions in the next chapter!](./conditions.md)

## Constant Variables

Do you really, really, **REALLY** like the value of your variable? If you want to make sure it stays the same forever, you can add the keyword ```const``` after ```var``` in your declaration. Once you do that, nobody can ever change its value!

Example:

```pax
var const PI = 3.14;

PI = 4; // Error! You can't do that.

print(PI); // This works perfectly.
```

Note: Convention dictates that ```const``` variables should be named in UPPERCASE. This helps other developers (and your future self) immediately recognize that this value is set in stone.

## Scoping

Pax uses a Scope Stack mechanism to manage variables. This ensures memory safety and prevents "variable pollution" by automatically destroying local data when it is no longer needed.

### The Scope Stack

Every Every time you enter a conditional, loop or function block, Pax pushes a new "level" onto the memory stack.

When you use a variable, Pax searches from the top of the stack (most local) to the bottom (global).

If you declare a variable with the same name as one in a parent scope, the local version "shadows" the outer one until the current block ends.

When a block ends, the top scope is "popped" (deleted), and all local variables inside it are cleared from memory.

### Variable declaration

By default, variables are local to the block where they are declared.

```pax
var x = 10
if true {
    var y = 20
    print(x + y) // Works: x is found in the parent scope
}
print(y) // Error: y was destroyed when the 'if' block ended
```

### Global Variables

If you need a variable to persist or be accessible from anywhere, use the ```global``` modifier. This forces the variable to be stored at the very bottom of the stack (Index 0), making it survive even after the current block closes.

Note: Use local variables by default. Only use ```global``` when data must transcend its current hallway.

```pax
if true {
    var global total = 100
}
print(total) // Works: 'total' is stored in the Global Scope
```

## Advanced Modifiers

In most languages, you have to remember if it's ```static public readonly``` or ```public static readonly```. In Pax, we believe your brain is better used for logic than for memorizing keywords order.

As long as you start with ```var```, you can mix your modifiers however you like.

All of these are valid and strictly equivalent:

```pax
var global const pi = 3.14
var const global pi = 3.14
var const pi = 3.14  // (Local constant)
var global pi = 3.14 // (Global mutable)
```

Why? Because our parser uses a "smart-collect" loop. It gathers all your flags first, then builds the variable. No more syntax errors because you swapped two words.