# URL Shortener

A simple, high-performance URL shortener built with Rust. It provides an efficient way to convert long URLs into unique short codes, with built-in caching for rapid redirection.

## Features

- **Shorten URLs:** Generate a unique Base62 encoded short code for any long URL.
- **Fast Redirection:** Redirect from short codes to original URLs with minimal latency.
- **Caching:** Utilizes Redis to cache URL mappings, significantly reducing database load and speeding up redirects.
- **Persistence:** Uses PostgreSQL for reliable, long-term storage of all shortened URLs.
- **Simple Frontend:** A clean, minimal web interface for interacting with the service.
- **Dockerized:** Fully containerized environment for easy deployment and development.

## Tools & Technologies

### Backend
- **[Rust](https://www.rust-lang.org/):** The core programming language.
- **[Actix-web](https://actix.rs/):** A powerful, pragmatic, and extremely fast web framework for Rust.
- **[SQLx](https://github.com/launchbadge/sqlx):** A modern SQL client with compile-time checked queries.
- **[Tokio](https://tokio.rs/):** An asynchronous runtime for the Rust programming language.
- **[Redis](https://redis.io/):** An in-memory data structure store used as a cache.
- **[PostgreSQL](https://www.postgresql.org/):** A powerful, open-source object-relational database system.
- **[Base62](https://github.com/Screaming-Viking/rust-base62):** Used for generating short, URL-friendly identifiers.

### Frontend
- **HTML/CSS/JS:** Pure web technologies for the user interface.
- **Nginx:** High-performance web server for serving the frontend assets.

### Infrastructure
- **Docker & Docker Compose:** Containerization and orchestration for the entire stack.

## Getting Started

### Prerequisites
- Docker and Docker Compose installed on your machine.
- (Optional) Rust toolchain if you wish to run the backend locally without Docker.

### Running with Docker

1. Clone the repository.
2. Create a `.env` file based on `.env.example`.
3. Start the services:
   ```bash
   docker-compose up --build
   ```
4. Access the frontend at `http://localhost:3000`.
5. The API will be running at `http://localhost:8000`.

