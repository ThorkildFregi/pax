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

Examples:

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