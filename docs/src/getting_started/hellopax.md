# Hello Pax!

Your journey begins in Middle-earth, during the Third Age. Your goal: defeat Sauron. To do that, we’re going to write our very first Pax program!

## Terminal tools

For all terminal commands, we are going to use ```bash```. If you are on Windows, I recommend using PowerShell or WSL (Windows Subsystem for Linux).

## Project Directory Setup

Listen carefully: as I already said before, if you write messy code in your ```home``` folder, I don't care, it is none of my bussiness. we are going to take a clean approach. First, we will create our project directory.

Note: You should start by creating a dedicated folder for all your programming projects.

```bash
$ mkdir journey_middle_earth
$ cd journey_middle_earth
```

## Pax Program Basics

Next, create a source file and name it ```journey.pax```. Pax files can technically end with any extension you like; however, if you don't use ````.pax````, you will be issued a warning. If you use more than one word in your filename, the convention is to use underscores to separate them. For example, use ```hello_world.pax``` instead of ```helloWorld.pax```.

Now open ```journey.pax``` and type:

```pax
var msg = "I defeated Sauron";

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