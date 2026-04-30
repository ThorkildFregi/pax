# Data Types

Imagine you are in a kitchen; every step of a recipe requires a different utensil. Programming is the same, and your utensils are the data types. There are three essential families of data types: numbers, strings, and booleans. We will now look at each of these variants and what they are used for.

## Numbers

For now, in this version of Pax, you only have access to integers. Floats (decimal numbers) will be added in a later update.

### Integer

Integers are the most basic type of number. If you are more than six years old, you already know how to use them! In Pax, this type is identified by the keyword ```int```. You can perform all the standard mathematical operations with them, which we will explore in detail later, but for now, think of them as your go-to tool for counting and whole-number calculations.

Keep in mind: any math you perform with an ```int``` will result in a whole number, unless the result is explicitly cast to another type. In the world of integers, 5 divided by 2 isn't 2.5... it’s 2! (Stay alive, my mathematician friends! If you die now, you will never see what other horrors we can do with math in Pax.)

## String

Do you know how to talk ? Yes, cool, because strings are litteraly just you talking to your computers. It is, indeed a chain of character.

Keep in mind that two string can be concatenated with a ```+```.

Note: don't try another math operations with strings, otherwise, you will be shown a beautiful error.

Examples:

```pax
"I am " + "awesome" // works and return "I am awesome"
"I am " - "awesome" // doesn't work and is not cool
```

## Boolean

A little French reference (for all the non-French speakers, go watch the movie "Les Visiteurs", it’s a classic!), you might know the character Jacquouille and his famous "DAY! NIGHT! DAY! NIGHT!" scene. That is literally a boolean. Yes, you could say Jacquouille invented the boolean!

A boolean is the equivalent of a simple light switch. It can only be in one of two states: ```true``` or ```false```.

In Pax, we use these to make decisions. Is the dragon awake? ```true```. Is the bridge safe? ```false```. [We will explore this more precisely in the chapter on conditions.](./conditions.md)