# Conditions

Conditions. Ah, conditions! You want to know if a number is odd or even? Conditions. You want to compare things? Conditions. Have you ever seen anything more powerful? (If you code in Rust, you know about macro rules, so you *have* seen something more powerful,but those aren't included in Pax... for now). Let's see how to use them right now!

## If

The keyword ```if``` is the start of your condition chain. Imagine a condition chain as a long hallway with many doors. The ```if``` is the very first one; to open it, you must meet the specific condition written on that door. If you manage to open the ```if``` door and walk through, all the other doors in the hallway lock, you can't enter any others.

To use ```if```, all you need is a boolean.

### Boolean

I may not have told you everything you need to know about booleans yet, my apologies! In Pax, you have access to eight different operations for now. These are split into two categories: comparison operators and logical operators.

#### Comparison operators

Comparison operators are used to compare two values, whether they are strings, numbers, booleans, or variables. You have six at your disposal:
- ```==``` : The "Equal to" operator. It checks if two values are the same.
- ```!=``` : The "Not equal to" operator. It checks if two values are different.
- ```>```: The "Strictly more" operator. It checks if a value is strictly superior to another.
- ```<```: The "Strictly less" operator. It checks if a value is striclty inferior to another.
- ```>=```: The "More or equal" operator. It checks if a value is superior or equal to another.
- ```<=```: The "Less or equal" operator. It checks if a value is inferior or equal to another.


Examples:

```pax
var alpha = 10;
var beta = 20;

alpha == beta // Return false
alpha != beta // Return true
alpha > beta  // Return false
alpha < beta  // Return true
alpha >= beta  // Return false
alpha <= beta  // Return true
```

#### Logical operators

Logical operators are used to combine booleans together. Just like with comparison, you have two logical operators:
- ```&``` (AND): Returns ```true``` only if both booleans are ```true```. If even one of them is ```false```, the whole thing returns ```false```.
- ```|``` (OR): Returns ```true``` if at least one of the two booleans is ```true```. It only returns ```false``` if both are ```false```.

Examples:

```pax
var alpha = true;
var beta = false;

alpha & beta // Return false
alpha | beta // Return true
```

Now that we understand the basics, let’s look at an example:

```pax
var alpha = 10;
var beta = 20;

if alpha == beta | alpha != beta {
    println("I will be printed!");
}
```

Why will this message be printed? First, ```alpha == beta``` returns ```false```. However, ```alpha != beta``` returns ```true```. Since the operator in the middle is ```|``` (OR), the entire expression evaluates to ```true```. In an ```if``` statement, if the boolean is ```true```, the code inside the curly brackets is executed.

Note: Pax follows a strict order of operations. It will always evaluate comparison operators first, then logical operators. And no, we don't support parentheses here. Why? Because nested parentheses are messy and unreadable. If your logic is too complex for a single line, use variables to break it down. It makes your code cleaner, and your brain will thank you later.

## Elif

Let's have some fun again! If you need to handle **A LOT OF CONDITIONS**, you can use ```elif``` after an ```if```. Let's see if a number is odd or even... the "pioneer" way!

Example:

```pax
var number = 3; // Put whatever you want here
var is_even = false;

if number == 1 {
    is_even = false;
} elif number == 2 {
    is_even = true;
} elif number == 3 {
    is_even = false;
} elif number == 4 {
    is_even = true;
} elif number == 5 {
    is_even = false;
} elif number == 6 {
    is_even = true;
} elif number == 7 {
    is_even = false;
}
// To be continued... until you lose your mind.
```

Note: **YOU SHOULD NEVER DO THIS**. It is messy, inefficient, and a terrible way to use ```if``` and ```elif```. However, it perfectly illustrates how ```elif``` works: it's just another door in the hallway that only checks its condition if the previous ones were locked.

## Else

But what if the number is 8, 100, or a billion? We can’t write a billion elif statements, unless you have a ridiculous amount of free time (or macro rules, but we said no macros for now!).

That is why we need a final safety net: the else keyword. Think of it as the "catch-all" door at the very end of your hallway. It doesn't have a condition because it doesn't care; it simply opens if every other door stayed locked.

```pax
if number == 1 {
    println("It's one!");
} elif number == 2 {
    println("It's two!");
} else {
    println("It's something else, and I'm too lazy to check!");
}
```

## Nested Conditions

Sometimes, one condition isn't enough. You might need to check something only if another condition is already ```true```. This is called nesting.

```pax
var is_day = true;
var is_sunny = true;

if is_day {
    if is_sunny {
        println("It's a beautiful day!");
    } else {
        println("It's day, but bring an umbrella.");
    }
} else {
    println("It's night time.");
}
```

### The Pyramid Of Doom

Be careful! While nesting is useful, if you nest too many ```if``` statements, your code will start to shift further and further to the right. This is what developers call the Pyramid of Doom.

If you find yourself nesting more than 3 levels deep:
- 1. Stop. 
- 2. Breathe. 
- 3. Refactor. Use logical operators (```&```, ```|```) or variables to flatten your logic.

Bad (The Messy Way):

```pax
if a == 1 {
    if b == 2 {
        if c == 3 {
            // Your brain is now melting
        }
    }
}
```

Good (The Pax Way):

```pax
var is_combo_valid = a == 1 & b == 2 & c == 3; // It works but it is not the best
// Let's do it the REAL Pax way:
var check_a = a == 1;
var check_b = b == 2;
var check_c = c == 3;

if check_a & check_b & check_c {
    // Clean, flat, and beautiful.
}
```