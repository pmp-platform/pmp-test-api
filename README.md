# PMP Test API

A Rust API for platform health checks and connectivity validation. This API helps verify that various infrastructure components (databases, APIs) are working correctly.

## Features

- **Health Check**: Simple endpoint to verify the API is running
- **SQL Database Checks**: Connect to and verify SQL databases (PostgreSQL support)
- **NoSQL Database Checks**: Connect to and verify NoSQL databases (Redis support)
- **HTTP API Checks**: Make requests to external APIs and return responses
- **Environment Inspection**: View all environment variables
- **Concurrent Checks**: All database and API checks run in parallel for optimal performance

## Endpoints

### `GET /_/health`

Simple health check endpoint.

**Response**: `200 OK` (no body)

### `GET /_/info`

Comprehensive platform information and connectivity checks.

**Response**: JSON object containing:
- `environments`: All environment variables
- `sql`: SQL database check results (if configured)
- `nosql`: NoSQL database check results (if configured)
- `http`: HTTP API check results (if configured)

## Configuration

Configure checks using environment variables with specific prefixes:

### SQL Database Checks

Format: `SQL_{identifier}_{param}`

**Required variables:**
- `SQL_{id}_DRIVER`: Database driver (currently supports: `postgres`)
- `SQL_{id}_HOST`: Database host
- `SQL_{id}_PORT`: Database port
- `SQL_{id}_USER`: Database username
- `SQL_{id}_PASSWORD`: Database password
- `SQL_{id}_DATABASE`: Database name

**Example:**
```bash
SQL_MYDB_DRIVER=postgres
SQL_MYDB_HOST=localhost
SQL_MYDB_PORT=5432
SQL_MYDB_USER=testuser
SQL_MYDB_PASSWORD=testpass
SQL_MYDB_DATABASE=testdb
```

### NoSQL Database Checks

Format: `NOSQL_{identifier}_{param}`

**Required variables:**
- `NOSQL_{id}_DRIVER`: Database driver (currently supports: `redis`)
- `NOSQL_{id}_HOST`: Database host
- `NOSQL_{id}_PORT`: Database port
- `NOSQL_{id}_PASSWORD`: Database password (optional)

**Example:**
```bash
NOSQL_CACHE_DRIVER=redis
NOSQL_CACHE_HOST=localhost
NOSQL_CACHE_PORT=6379
```

### HTTP API Checks

Format: `HTTP_{identifier}_{param}`

**Required variables:**
- `HTTP_{id}_URL`: API URL to request
- `HTTP_{id}_METHOD`: HTTP method (default: `GET`)
- `HTTP_{id}_HEADERS`: JSON object with headers (default: `{}`)

**Example:**
```bash
HTTP_TESTAPI_URL=http://localhost:8081/status/200
HTTP_TESTAPI_METHOD=GET
HTTP_TESTAPI_HEADERS={"User-Agent":"pmp-test-api"}
```

## Quick Start

### Prerequisites

- Rust 1.82+ (with 2024 edition support)
- Docker and Docker Compose (for local testing)

### Local Development

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd pmp-test-api
   ```

2. **Start test services**:
   ```bash
   docker-compose up -d
   ```

   This starts PostgreSQL, Redis, and HTTPBin for testing (without the API itself).

3. **Copy environment configuration**:
   ```bash
   cp .env.example .env
   ```

4. **Build and run**:
   ```bash
   cargo run
   ```

   The API will start on `http://localhost:8080` (configurable via `PORT` env var).

5. **Test the endpoints**:
   ```bash
   # Health check
   curl http://localhost:8080/_/health

   # Platform info and checks
   curl http://localhost:8080/_/info | jq
   ```

## Environment Variables

### Server Configuration

- `PORT`: Server port (default: `8080`)
- `RUST_LOG`: Logging level (default: `info,pmp_test_api=debug`)

## Docker

### Running with Docker

Pull and run the pre-built image:

```bash
# Pull the latest image
docker pull ironedge/pmp-test-api:latest

# Run the container
docker run -p 8080:8080 \
  -e SQL_MYDB_DRIVER=postgres \
  -e SQL_MYDB_HOST=host.docker.internal \
  -e SQL_MYDB_PORT=5432 \
  -e SQL_MYDB_USER=testuser \
  -e SQL_MYDB_PASSWORD=testpass \
  -e SQL_MYDB_DATABASE=testdb \
  ironedge/pmp-test-api:latest
```

### Building Docker Image Locally

```bash
# Build the image
docker build -t pmp-test-api .

# Run the image
docker run -p 8080:8080 pmp-test-api
```

### Docker Compose

The docker-compose.yaml includes the API service under the `app` profile, which allows flexible usage:

```bash
# Option 1: Run only test services (postgres, redis, httpbin)
# Good for local development with cargo run
docker-compose up -d

# Option 2: Run the entire stack including the API
# Good for testing the full containerized setup
docker-compose --profile app up -d

# Build and run the API with latest code changes
docker-compose --profile app up -d --build

# View logs (all services)
docker-compose logs -f

# View logs (only app service)
docker-compose logs -f app

# Stop the stack
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

**Note**: The `app` service is configured with environment variables to automatically test all services in the docker-compose stack (PostgreSQL, Redis, and HTTPBin).

## Building for Production

```bash
# Build optimized binary
cargo build --release

# Run the optimized binary
./target/release/pmp-test-api
```

## CI/CD

This project uses GitHub Actions for continuous integration and deployment:

### PR Workflow (`.github/workflows/pr.yml`)
- Triggered on pull requests to `main`
- Runs on every push to the PR
- Steps:
  - Checks code formatting (`cargo fmt`)
  - Runs linter (`cargo clippy`)
  - Builds the project
  - Runs tests
  - Builds release binary

### Docker Build Workflow (`.github/workflows/docker.yml`)
- Triggered when PRs are merged to `main`
- Builds multi-architecture Docker image (amd64, arm64)
- Pushes to Docker Hub: `ironedge/pmp-test-api`
- Tags:
  - `latest` (for main branch)
  - `main-<sha>` (git commit SHA)
  - `main` (branch name)

### Required GitHub Secrets

To enable Docker image publishing, configure these secrets in your GitHub repository:
- `DOCKER_USERNAME`: Your Docker Hub username
- `DOCKER_PASSWORD`: Your Docker Hub access token

## Example Response

```json
{
  "environments": {
    "PATH": "/usr/local/bin:/usr/bin",
    "SQL_TESTDB_DRIVER": "postgres",
    ...
  },
  "sql": {
    "TESTDB": {
      "success": true,
      "driver": "postgres",
      "host": "localhost",
      "port": 5432,
      "database": "testdb",
      "tables": ["users", "products", "orders"]
    }
  },
  "nosql": {
    "CACHE": {
      "success": true,
      "driver": "redis",
      "host": "localhost",
      "port": 6379,
      "info": {
        "redis_version": "7.0.0",
        "connected_clients": "1",
        ...
      }
    }
  },
  "http": {
    "TESTAPI": {
      "success": true,
      "url": "http://localhost:8081/status/200",
      "method": "GET",
      "status_code": 200,
      "response_headers": {
        "content-type": "text/html; charset=utf-8",
        ...
      },
      "response_body": "..."
    }
  }
}
```

## Error Handling

The `/_/info` endpoint always returns `200 OK`, even if individual checks fail. Failed checks will have `"success": false` and include an `"error"` field with details.

This design allows monitoring systems to always receive a response and inspect individual check results.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Author

Gustavo Falco
