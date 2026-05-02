# Pax : a programming language

Pax is a modern, lightweight programming language built in Rust.

### Table of contents

- [About](#about)
    - [Features](#features)
    - [Documentation](#documentation)
- [Download](#download)
- [Version](#version)
- [License](#license)
- [Contribute](#contribute)

## About

Our goals with Pax ? Ensure a fast and  memory-safe environment for data science and statistics.

Matrix, exponential, logarithm and more, a whole suite of mathematical tools is built directly into Pax's core to ensure fast and direct usage without heavy external dependencies.

We aim to bridge the gap between existing languages: the conciseness of C++, the memory safety of Rust, and the ease of use of Python.

We are a fully open-source project and we encourage [contributions](#contribute) or forks as long as you respect our [license](#license).

### Features

**Type inference**: Like Python, there is no need to specify the type of your variable during declaration. However, to ensure type security, a variable cannot change its type once set. If you need a different type, simply re-declare the variable (shadowing).

**Clean logic**: Tired of long ```if``` statements with a thousand nested conditions? Not here! Pax encourages and enforces the use of intermediate variables for better visibility and cleaner code. No parentheses allowed in conditions—keep it flat, keep it readable.

**Scope Stack**: No more memory leaks or global mess. Pax uses a stack-based scoping system that keeps your data where it belongs. Variables declared with ```var``` are local. Once your ```if``` block ends, they're gone. No pollution, no leftovers. Need a variable from a parent scope? Pax automatically climbs down the stack to find it for you. Want a variable to survive the block? Add ```global``` to your declaration and it’ll stay at the base of the stack until the program ends.

### Documentation

You can access the full documentation online or locally to explore all the possibilities of Pax.

[Online](https://thorkildfregi.github.io/pax/) | [Download](#download)

## Download

### Via script

The fastest way to get Pax is via our automated installer:

Unix systems:

```bash
$ curl -sSf https://raw.githubusercontent.com/ThorkildFregi/pax/main/install.sh | sh
```

Windows:

```PowerShell
> Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/ThorkildFregi/pax/main/install.ps1'))
```

### Manual Installation

You can find pre-compiled binaries for Linux, macOS, and Windows on our [GitHub](https://github.com/ThorkildFregi/pax) Releases page. Just download the version corresponding to your OS and add it to your PATH.

## Version

Actually in v0.2.0-alpha, here are the new features:

- Add Scope Stack

## License

This project is licensed under the MIT License. You are free to use, modify, and distribute this software as long as the original license and copyright notice are included.

## Contribute

We welcome contributions from everyone! Whether you want to fix a bug, suggest a new mathematical feature, or improve the documentation, your help is appreciated.

To contribute:
1. **Fork** the repositery
2. **Create** a branch for your feature (```git checkout -b feature/amazing-feature```).
3. **Commit** your changes (```git commit -m 'Add some amazing feature'```).
4. **Push** to the branch (```git push origin feature/amazing-feature```).
5. **Open** a Pull Request.