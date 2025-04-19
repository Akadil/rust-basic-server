# Rust Basic Server

A basic backend server built with Rust and Axum framework with JWT authentication.

## Features

- User registration and login
- JWT-based authentication
- Protected routes that require authentication
- In-memory user storage (for simplicity)
- Error handling
- Environment variable configuration

## Project Structure

```
src/
├── main.rs           # Entry point
├── config.rs         # Configuration
├── error.rs          # Error handling
├── routes/           # API routes
│   ├── mod.rs
│   ├── auth.rs       # Authentication routes
│   └── protected.rs  # Protected routes
├── handlers/         # Request handlers
│   ├── mod.rs
│   ├── auth.rs       # Authentication handlers
│   └── protected.rs  # Protected route handlers
├── middleware/       # Middleware
│   ├── mod.rs
│   └── auth.rs       # Authentication middleware
├── models/           # Data models
│   ├── mod.rs
│   └── user.rs       # User model
└── utils/            # Utilities
    ├── mod.rs
    └── jwt.rs        # JWT utilities
```

## Getting Started

### Prerequisites

- Rust and Cargo (https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository
2. Navigate to the project directory
3. Build and run the project:

```bash
cargo run
```

The server will start on http://127.0.0.1:3000

## API Endpoints

### Authentication

- `POST /api/auth/register` - Register a new user
  - Request body: `{ "username": "string", "email": "string", "password": "string" }`
  - Response: User data and JWT token

- `POST /api/auth/login` - Login with existing user
  - Request body: `{ "email": "string", "password": "string" }`
  - Response: User data and JWT token

### Protected Routes

- `GET /api/protected` - Access protected data (requires authentication)
  - Header: `Authorization: Bearer <token>`
  - Response: Protected data

## Environment Variables

Create a `.env` file in the root directory with the following variables:

```
JWT_SECRET=your_jwt_secret_key
JWT_EXPIRES_IN=86400
RUST_LOG=info
```

## License

This project is licensed under the MIT License.
