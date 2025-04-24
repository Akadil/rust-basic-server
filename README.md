# Rust Basic Server

A DDD-style Rust server with Axum, JWT authentication, and role-based access control.

## Features

- **Domain-Driven Design (DDD)** architecture
- **Axum** web framework
- **JWT** authentication
- **Role-based access control**
- **PostgreSQL** database integration
- **In-memory** repository for testing
- **Docker** support for easy deployment
- **CORS** configuration
- **Tracing** for logging and monitoring
- **Environment variables** for configuration

## Project Structure

The project follows a DDD architecture with the following layers:

- **Domain**: Core business logic, entities, and repository interfaces
- **Application**: Use cases and application services
- **Infrastructure**: Technical implementations of interfaces defined in the domain
- **App**: Entry point and configuration

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Docker (optional, for running PostgreSQL)

## Setup

### Option 1: Using Docker (Recommended)

1. Clone the repository:

```bash
git clone https://github.com/yourusername/rust-basic-server.git
cd rust-basic-server
```

2. Run the application using Docker Compose:

```bash
# Using the provided script
./scripts/run_docker.sh

# Or manually
docker-compose up -d
```

This will start:
- The main application with PostgreSQL repository on port 8080
- A version of the application with in-memory repository on port 8081
- A PostgreSQL database

3. To stop the containers:

```bash
# Using the provided script
./scripts/stop_docker.sh

# Or manually
docker-compose down
```

### Option 2: Manual Setup

1. Clone the repository:

```bash
git clone https://github.com/yourusername/rust-basic-server.git
cd rust-basic-server
```

2. Set up the database:

```bash
# Using Docker
docker run --name postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_USER=postgres -e POSTGRES_DB=rust_server -p 5432:5432 -d postgres

# Or use your existing PostgreSQL installation
createdb rust_server
```

3. Configure environment variables:

Copy the `.env.example` file to `.env` and adjust the values as needed:

```bash
cp .env.example .env
```

4. Run the server:

```bash
cargo run
```


## Authentication

The server uses JWT for authentication. To authenticate:

1. Register a new user:

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@example.com","password":"password","role":"Admin"}'
```

2. Login to get a JWT token:

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password"}'
```

3. Use the token in subsequent requests:

```bash
curl -X GET http://localhost:8080/api/users \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## User Roles

The server supports the following roles:

- **Admin**: Full access to all endpoints
- **Manager**: Access to user management (except deleting users)
- **User**: Access to own user data
- **Guest**: Limited access to public endpoints

## Development

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
