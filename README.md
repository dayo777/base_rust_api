# Base API Template (Rust)

## Overview
This is a lightweight and reusable API template written in Rust, designed for quick deployment in hackathons, prototypes, and other projects. It provides a structured yet flexible foundation that can be easily extended.

## Features
- Built with **Actix-web**
- Supports **JWT authentication** (optional)
- **PostgreSQL/MySQL/SQLite** integration (if applicable)
- Configurable **logging and error handling**
- Ready for **Docker and cloud deployment**

## Quick Start

### Prerequisites
- Rust (latest stable version)
- Cargo package manager

### Installation
1. Clone this repository:
   ```sh
   git clone https://github.com/dayo777/base-api-rust.git
   cd base-api-rust
   ```

2. Install dependencies:
   ```sh
   cargo build
   ```

3. Run the API:
   ```sh
   cargo run
   ```

## Configuration
Edit the `.env` file for database credentials, API settings, and other environment variables.

## Customization
Modify routes, middleware, and database configurations as needed. This API is structured for flexibility and quick edits.

## Project Structure
The project follows a modular structure for better scalability and maintainability:
```
base-api-rust/
│── src/
│   ├── main.rs            # Entry point of the application
│   ├── lib.rs             # Library module (optional for reuse)
│   ├── config.rs          # Configuration management (loading .env, settings)
│   ├── routes/
│   │   ├── mod.rs         # Routes module
│   │   ├── health.rs      # Health check route
│   │   ├── route_1.rs     # Route-1
│   │   ├── route_2.rs     # Route-2, add more routes as needed   
│   ├── handlers/
│   │   ├── mod.rs         # Handlers module
│   │   ├── handler_1.rs   # App logic for route-1 & model-1
│   │   ├── handler_2.rs   # App logic for route-2 & model-2
│   ├── models/
│   │   ├── mod.rs         # Models module
│   │   ├── model_1.rs     # Model-1 definition
│   │   ├── model_2.rs     # Model-2 definition
│   ├── db/
│   │   ├── mod.rs         # Database module
│   │   ├── connection.rs  # Database connection setup
│   ├── errors/
│   │   ├── mod.rs         # Error handling module
│   │   ├── app_error.rs   # Custom application errors
│   ├── middleware/
│   │   ├── mod.rs         # Middleware module
│   │   ├── auth.rs        # Authentication middleware (e.g., JWT)
│   ├── utils/
│   │   ├── mod.rs         # Utility module
│   │   ├── logger.rs      # Logging utilities
│── .env                   # Environment variables
│── Cargo.toml             # Rust dependencies
│── Dockerfile             # Docker configuration
│── README.md              # Documentation
```

## Deployment
For Docker-based deployment, use:
```sh
docker build -t base-api .
docker run -p 8080:8080 base-api
```
