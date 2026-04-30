# Hello Pax!

Your journey begins in Middle-earth, during the Third Age. Your goal: defeat Sauron. To do that, we’re going to write our very first Pax program!

## Terminal tools

For all terminal commands, we are going to use ```bash```. If you are on Windows, I recommend using PowerShell or WSL (Windows Subsystem for Linux).

## Project Directory Setup

Listen carefully: as I already said before, if you write messy code in your ```home``` folder, I don't care, it is none of my bussiness. But we are going to take a clean approach. First, we will create our project directory.

```bash
$ mkdir journey_middle_earth
$ cd journey_middle_earth
```

Note: You should start by creating a dedicated folder for all your programming projects.

## Pax Program Basics

Next, create a source file and name it ```journey.pax```. Pax files can technically end with any extension you like; however, if you don't use ````.pax````, you will be issued a warning. If you use more than one word in your filename, the convention is to use underscores to separate them. For example, use ```hello_world.pax``` instead of ```helloWorld.pax```.

Now open ```journey.pax``` and type:

```pax
var msg = "I defeated Sauron !";

print(msg);
```

To run it, go back to your terminal and type:

```bash
$ pax journey.pax
```

You should see:

```bash
I defeated Sauron !
```

## The explanation of this journey

Unlike C++ or Rust, and more like Python, Pax does not require a ```main``` function to work. You can start coding directly, and it will run without any issues.

However, and this is for my messy boys and girls out there, line breaks and indentation do not influence how your code runs. In Pax, every instruction ends with a semicolon ```;```, and conditions, loops, and functions use curly brackets ```{}```. We will dive deeper into these in the next chapters.

```pax
var msg = "I defeated Sauron !";
```

The first line of ```journey.pax``` is a [variable](./../common_programming_concepts/variables.md) declaration. It is identified by the keyword ```var```. Immediately following the keyword is the name of the variable, in this case, ```msg```. Next comes the assignment token ```=```, followed by a string: ```"I defeated Sauron!"```. A string is easily recognizable by the double quotes ```" "``` surrounding the text. [(We will dive into data types later!)](./../common_programming_concepts/data_types.md)

```pax
print(msg);
```

The second line is the most important one here. The ```print``` function will be your best friend. It is the function that writes to your terminal. Note that ```print``` outputs text without a line break; if you need one, please use ```println```. Here, you are calling ```print``` and passing it a parameter: the content of our variable ```msg```.

## Conclusion

Congratulations! You made it through your first journey with Pax! Next chapters will deep more into Pax logics and possibilities.