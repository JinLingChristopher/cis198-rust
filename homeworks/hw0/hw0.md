# Homework 0

Setting Up Cargo and Hello World!

This assignment is merely setting up Rust and the programming environment for the course. Cargo is the package manager and build tool for rust (no makefiles here!).

1) **Email me your github username you plan to use for submitting the assignments!**
2) Set up a Linux System
3) Set up Rust

### Operating System
We will be programming systems programs which often rely on OS implementation details to work. Specifically we will be focusing on Linux x86-64. You may use any distro you like for this, while I expect the distro not to make much of a difference, I will be grading/testing the assignments with Ubuntu 18.04 LTS using kernel version 4.15.

Note: If your implementation acts differently on the grading machine because of subtle differences or bugs across distros or OS versions, don't worry. We will figure it out together and no harm will come to your grade.

### Rust Versions
As of this writing, the latest version of Rust is 1.37.0. We will use the newest stable version available for every assignment. Please read  about [the rust train release model](https://doc.rust-lang.org/book/second-edition/appendix-07-nightly-rust.html#choo-choo-release-channels-and-riding-the-trains) for more information. I am using the rust target stable-x86_64-unknown-linux-gnu

Note: For our purposes, it won't really make a difference. But this will ensure everyone is working off the same base. So don't really worry about it.

## Setup

1) **Virtualbox**: If you don't already have a computer with a Linux environment running I recommend setting up a virtual machine using VirtualBox using a Ubuntu 18.04 LTS [image](https://www.ubuntu.com/download/desktop). If you don't already know how to set up a virtual machine, a quick google found [this tutorial](https://www.lifewire.com/run-ubuntu-within-windows-virtualbox-2202098). Setting up a virtual machine is a good skill to have a will be useful throughout your programming career.

2) **Rustup**: Rustup is the rust tool chain manager. It allows you to have several rust versions at once, seamlessly switch between them, upgrade your rust tools. On your Linux system (either native or virtual machine) [install rustup](https://www.rust-lang.org/en-US/install.html). For a more detailed procedure please follow the [rust book](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)

## Assignment
Cargo is the rust package manager and build tool. We will have a future detailed lecture on cargo and all of it's features. For now, make sure you're able to create a new project, say hello-world:

```bash
cargo new hello-world
```
Then, inside the project, edit the `src/main.rs` file to print to standard output the string "Hello World".

You can run the program by typing in:
```bash
cargo run
```

For more detailed steps please see the Rust book's [introduction to cargo](https://doc.rust-lang.org/book/second-edition/ch01-03-hello-cargo.html). You may also be interested in the [Cargo Book](https://doc.rust-lang.org/cargo/) as a reference.

## Submission
This assignment will **NOT** be submitted. It is merely for setting up your system. Assignment 1 will be the first true assignment.

## Help
Do not hesitate to get help for this assignment. Setting up environments can be a major pain point of development!
