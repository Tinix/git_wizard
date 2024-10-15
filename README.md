# Git Wizard

**Git Wizard** is a CLI tool written in Rust that automates common Git workflow operations, making it easier to create branches, switch between them, push changes, and merge branches in a simplified manner. It is designed to streamline repetitive tasks and improve productivity for developers who follow a Git branching strategy like **feature development**.

## Features

- List existing Git branches.
- Create a new branch with a ticket number and description.
- Automatically push new branches to the remote repository.
- Easily switch back to the `develop` branch after committing changes.
- Automatically increment branch numbers for feature development.
- Supports automatic commit messages with ticket numbers included.

## Installation

To install **Git Wizard**, you need to have Rust and Cargo installed. Then, clone the repository and build the project:

```bash
git clone https://github.com/your-username/git_wizard.git
cd git_wizard
cargo build --release
