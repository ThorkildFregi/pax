# Installation

To begin your journey into Middle-Earth, you first need to arrive there. So, the first step is to install Pax. Don't worry, it will be easy, I promise!


## Command Line Notation

Like in [The best documentation that ever lived](https://doc.rust-lang.org/book/), we're gonna use a simple process for terminal command line. Lines that you should enter in a terminal all start with $. You don’t need to type the $ character; it’s the command line prompt shown to indicate the start of each command. Lines that don’t start with $ typically show the output of the previous command. Additionally, PowerShell-specific examples will use > rather than $.

## Install Pax on Linux and macOS

Are you on a Unix-based system? First of all, I like you (especially if you're on Linux). Second, installing Pax is a breeze: we are going to use ```curl``` to fetch the script and run it.

```bash
$ curl -sSf https://raw.githubusercontent.com/ThorkildFregi/pax/main/install.sh | sh
```

Note: There is a high possibility that you will need to use sudo before this command; don't forget it if the installation fails!

## Install Pax on Windows

As a Windows user, you know how it goes. Windows can be a little, okay, a lot, protective, so we need to temporarily lift some execution restrictions. Don't worry, I'm not a hacker; otherwise, I wouldn't have written such a long documentation. (Which is exactly what a hacker would say, but anyway...)

```PowerShell
> Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/ThorkildFregi/pax/main/install.ps1'))
```

## Verification

Do you think you’ve successfully completed the installation ? Verify it with this command:

```bash
$ pax --version
```

If you see something like:

```bash
Pax Language vX.Y.Z-maybe_something
```

You're good to go!

Note: Was the ```pax``` command not found by your terminal? You might need to restart your terminal!

## Update

For now, don't worry about a complex update process. Simply run the installation command again as if Pax were never there, and it will do the job perfectly.

## Reading the documentation

I know you're already doing it, I'm not crazy (at least for now). But did you know you can also access it locally? Just type this command:

```bash
$ pax docs
```

## Using Text Editors And IDE

This documentation makes no assumptions about which tools you use. However, I have developed a dedicated IDE specifically for this language (created even before the language itself, I know, it's genius!). I highly recommend using it; it's called [Platrix](https://github.com/ThorkildFregi/platrix).