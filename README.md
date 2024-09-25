# Microservices Project

This project consists of two main microservices: an Authentication Service and a Report Service. It uses Kafka for message queuing, PostgreSQL for the auth database, MongoDB for the report database, and Redis for caching.

## Services

1. Authentication Service (Rust)
   - Handles user registration, login, and token management
   - Uses PostgreSQL for data storage
   - Uses Redis for caching

2. Report Service (Python)
   - Manages report creation and listing
   - Uses MongoDB for data storage
   - Consumes messages from Kafka

## Prerequisites

- Docker and Docker Compose
- Rust (for local development of the Auth service)
- Python 3.11+ (for local development of the Report service)

## Getting Started

1. Clone the repository:
   ```
   git clone <repository-url>
   cd <project-directory>
   ```

2. Build and start the services:
   ```
   docker-compose up --build
   ```

3. The services will be available at:
   - Auth Service: http://localhost:50051
   - Report Service: http://localhost:50052

## Development

### Auth Service

1. Navigate to the auth directory:
   ```
   cd auth
   ```

2. Install dependencies:
   ```
   cargo build
   ```

3. Run the service locally:
   ```
   cargo run
   ```

### Report Service

1. Navigate to the report service directory:
   ```
   cd report/report-service
   ```

2. Create a virtual environment and install dependencies:
   ```
   python -m venv venv
   source venv/bin/activate  # On Windows, use `venv\Scripts\activate`
   pip install -r requirements.txt
   ```

3. Run the service locally:
   ```
   python main.py
   ```

## Testing

- For the Auth Service, run:
  ```
  cargo test
  ```

- For the Report Service, run:
  ```
  pytest
  ```