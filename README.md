# Robust Rust - Backend API Project

![Rust Logo](https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png)

**Table of Contents**
- [Robust Rust - Backend API Project](#robust-rust---backend-api-project)
    - [Introduction](#introduction)
    - [Purpose](#purpose)
    - [Features](#features)
    - [Getting Started](#getting-started)
    - [Technologies Used](#technologies-used)
    - [Project Structure](#project-structure)
    - [How to Contribute](#how-to-contribute)
    - [Acknowledgments](#acknowledgments)

### Introduction
This project was born out of my enthusiasm for learning Rust and exploring its potential as a backend language for building APIs. Rust is a modern, safe, and performance-oriented programming language that excels in systems programming, but it is also well-suited for web development, making it an excellent choice for building backend APIs.

### Purpose
The primary purpose of this project is to serve as a learning experience for both myself and anyone else interested in Rust and API development. By creating a robust backend API in Rust, I aim to demonstrate best practices, design patterns, and idiomatic Rust code that ensures both functionality and maintainability.

### Features
The project aims to showcase various features commonly found in a backend API. Some of the planned features include:
- RESTful API endpoints for CRUD operations on resources
- Authentication and user management
- Database integration (e.g., PostgreSQL, MySQL, or Redis)
- Input validation and error handling
- Pagination and filtering of data
- Continous Integration and Development

### Getting Started
To run the project locally or contribute to its development, follow these steps:
1. Clone the repository: `git clone https://github.com/utilitycoder/robust-rust.git`
2. Install Rust and Cargo (the Rust package manager) if you haven't already.
3. Navigate to the project directory: `cd robust-rust`
4. Install project dependencies: `cargo build`
5. Configure the environment variables and database connection settings. Customize the .env file and run both PostgreSQL and Redis through docker images. Run the files in the `scripts` folder.
6. Run the project: `cargo run`
7. The API should now be accessible at `http://localhost:8000`.

### Technologies Used
The project utilizes the following technologies:
- **Rust:** The core programming language used for backend development.
- **Actix-web** A web framework for Rust that provides routing, request handling, and middleware support.
- **Serde:** A serialization/deserialization library for Rust, used to work with JSON or other data formats.
- **SQLX:** Rust library that provides a safe and convenient way to work with SQL databases. .
- **Tokio:** A runtime for writing asynchronous applications in Rust.
- A host of other crates.

### Project Structure
The project follows a modular structure to keep the code organized and maintainable. The main components include:
- `src/`: Contains the source code for the backend API.
- `src/routes/`: Defines the API endpoints and request handlers.
- `src/domain/`: Contains the data structure and cleanup of subscriber details.
- `src/authentication/`: Contains business logic and data processing related to user authentication.
- `src/idempotency/`: Includes code related to idempotency functionality.
- `src/*.rs`: Other Rust source files that do not fall under specific folders.
- `configuration/`: Deployment environment configuration files for the project.
- `migrations/`: Database schema migration scripts.
- `scripts/`: Scripts for building, initializing both PostgreSQL and Redis database.
- `tests/`: Contains integration tests for the project.

### How to Contribute
Contributions to this project are welcome! Whether you want to fix a bug, add a new feature, or improve documentation, follow these steps:
1. Fork the repository.
2. Create a new branch for your feature: `git checkout -b feature-name`
3. Make your changes and commit them with descriptive messages: `git commit -m "Add feature-name"`
4. Push your changes to your forked repository: `git push origin feature-name`
5. Create a pull request, detailing your changes and the problem they solve.


### Acknowledgments
I would like to thank Luca Palmieri for the amazing work he did with zero2prod book. His book exposed me to the world of building robust backend API with Rust.
