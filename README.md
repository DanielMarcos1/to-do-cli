# To-Do CLI

A simple and efficient command-line task manager developed in Rust. It allows you to add, list, and mark tasks as completed, storing them in a JSON file.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Technologies Used](#technologies-used)
- [Installation](#installation)
  - [Prerequisites](#prerequisites)
  - [Steps](#steps)
- [Usage](#usage)
  - [Add a Task](#add-a-task)
  - [List Tasks](#list-tasks)
  - [Mark Task as Completed](#mark-task-as-completed)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Overview

The **To-Do CLI** is a command-line application developed to help users manage their tasks in a simple and quick manner. With intuitive commands, you can add new tasks, view all registered tasks, and mark tasks as completed.

## Features

- **Add Tasks**: Create new tasks with a descriptive title.
- **List Tasks**: Display all existing tasks with their status.
- **Mark as Completed**: Update the status of a task to completed.

## Technologies Used

- **Rust**: A high-performance and secure programming language.
- **Clap**: A library for creating command-line interfaces.
- **Serde**: A library for serializing and deserializing data.
- **Chrono**: A library for date and time manipulation.

## Installation

### Prerequisites

- **Rust**: Ensure that Rust is installed on your system. You can install Rust through [rustup](https://rustup.rs/).

### Steps

1. **Clone the Repository**

   ```bash
   git clone https://github.com/your-username/to-do-cli.git
   cd to-do-cli
   ```

2. **Build the Project**

   ```bash
   cargo build --release
   ```

   The executable will be generated in `target/release/to-do-cli`.

3. **Install Globally (Optional)**

   To install the application globally on your system:

   ```bash
   cargo install --path .
   ```

   After installation, you can run the `to-do-cli` command from anywhere.

## Usage

After installation, you can use the following commands:

### Add a Task

Adds a new task with the specified title.

```bash
   cargo run -- add "Buy groceries"
   ```

### List Tasks

Displays all existing tasks.

```bash
   cargo run -- list
   ```  

**Example output:**

```bash
[ ] 1 - Buy groceries (Created on: 25/04/2024 14:30)
[X] 2 - Read a book (Created on: 24/04/2024 10:15)
```

### Mark Task as Completed

Marks a task as completed by its ID.

```bash
   cargo run -- done 1
   ```  

## Project Structure

```bash
to-do-cli/
├── Cargo.toml
├── README.md
└── src
├── armazenamento.rs
├── main.rs
└── tarefa.rs
```

- **`armazenamento.rs`**: Handles the storage and retrieval of tasks from a JSON file.
- **`main.rs`**: Contains the main logic for command-line interface and task management.
- **`tarefa.rs`**: Defines the `Tarefa` struct and related functions.

## Contributing 

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.
