# Referral Service API

A RESTful API service built with Rust for managing referral systems.

## 📋 Tech Stack

- **Backend**: Rust
- **Database**: PostgreSQL
- **Cache**: Redis
- **Docker**: Containerization
- **API**: RESTful endpoints
- **Query Builder**: SQLx

## 📋 Prerequisites

Before you begin, ensure you have the following installed:
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust](https://www.rust-lang.org/tools/install) (for local development)

## 🛠️ Setup and Installation

### Using Docker (Recommended)

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <project-directory>/api-service
   ```

2. Create a `.env` file:
   ```bash
   cp .env.example .env
   ```

3. Start the database first:
   ```bash
   # Start only the database service
   docker-compose up db -d

   # Wait for database to be ready
   sleep 5
   ```

4. Prepare SQLx query data:
   ```bash
   # Install sqlx-cli if not already installed
   cargo install sqlx-cli --no-default-features --features postgres

   # Set DATABASE_URL for local development
   export DATABASE_URL=postgres://postgres:postgres@localhost:5432/referral_db

   # Create and migrate database
   sqlx database create
   sqlx migrate run

   # Generate SQLx prepare data
   cargo sqlx prepare
   ```

5. Build and start all services:
   ```bash
   # Clean up existing containers and build cache (optional)
   docker-compose down
   docker system prune -f

   # Build and start services
   docker-compose up --build

   # Or run in detached mode
   docker-compose up -d --build
   ```

### Docker Commands

Common commands for development:

```bash
# Start all services in detached mode
docker-compose up -d

# View logs
docker-compose logs -f

# View logs for specific service
docker-compose logs -f api
docker-compose logs -f db

# Stop all services
docker-compose down

# Rebuild specific service
docker-compose up -d --build api

# Clean up Docker resources
docker-compose down
docker system prune -f

# Check service status
docker-compose ps

# View Redis logs
docker-compose logs -f redis

# Connect to Redis CLI
docker-compose exec redis redis-cli

# Monitor Redis
docker-compose exec redis redis-cli monitor
```

### Local Development

1. Install Rust dependencies:
   ```bash
   cd api-service
   cargo build
   ```

2. Set up the database:
   ```bash
   # Set DATABASE_URL temporarily for local development
   export DATABASE_URL=postgres://postgres:postgres@localhost:5432/referral_db

   # Create database and run migrations
   sqlx database create
   sqlx migrate run

   # Generate SQLx prepare data
   cargo sqlx prepare
   ```

3. Run the service:
   ```bash
   cargo run
   ```

## 🔍 API Endpoints

The service runs on port 8080 by default. Available endpoints:

### Create Referral
- **Endpoint**: `POST /api/referrals`
- **Body**:
  ```json
  {
    "user_id": "string",
    "referral_code": "string"
  }
  ```
- **Response**: 201 Created

### Get All Referrals
- **Endpoint**: `GET /api/referrals`
- **Response**: 200 OK

## 🔧 Configuration

Environment variables (`.env` file):
- `DATABASE_URL`: PostgreSQL connection string
- `DATABASE_MAX_CONNECTIONS`: Maximum database connections (default: 5)
- `REDIS_URL`: Redis connection string (default: redis://redis:6379)
- `SERVER_PORT`: API server port (default: 8080)
- `RUST_LOG`: Logging level (debug/info/warn/error)

## 📦 Project Structure

```
api-service/
├── src/
│   ├── main.rs          # Application entry point
│   ├── config.rs        # Configuration handling
│   ├── handlers.rs      # API route handlers
│   ├── models.rs        # Data models
│   ├── repository.rs    # Database operations
│   ├── cache.rs         # Redis cache operations
│   └── error.rs         # Error handling
├── migrations/          # Database migrations
├── Cargo.toml          # Rust dependencies
├── Dockerfile          # Docker configuration
└── docker-compose.yml  # Docker services setup
``` 