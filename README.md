# Axum Backend

## Features

## Preparations

To run this project, you will need:

- [Rust](https://rust-lang.org/tools/install/) installed on your machine.
- [Docker](https://www.docker.com/) installed on your machine.
- [mise](https://mise.jdx.dev/getting-started.html) on your machine.

## Installation

### 1. Clone the repository

```shell
git clone https://github.com/DarcyJason/AxumBackend
```

### 2. Start SurrealDB server on docker

```shell
docker run --name surrealdb \
  -p 10086:8000 \
  -v ~/surrealdb:/data \
  -d surrealdb/surrealdb:v2.4.0 \
  start \
  --user root \
  --pass root \
  rocksdb:/data/mydatabase.db
```

### 3. Start Redis server on docker

```shell
docker run --name redis \
  -p 6379:6379 \
  -v ~/redis:/data \
  -d redis:8.4.0
```

### 4. Install surrealdb-migrations by mise

```shell
mise run prepare
```

### 5. Install tools in mise.toml by mise

```shell
mise install
```

### 6. Apply migrations to your SurrealDB

```shell
mise run migration
```

### 7. Configure .env

```shell
cp .env.example .env
```

- replace SYSTEM_OWNER_NAME to your name
- replace SYSTEM_OWNER_EMAIL to your email
- replace SYSTEM_OWNER_PASSWORD to your password
- replace JWT_SECRET to your jwt secret
- replace FROM_EMAIL to your email
- replace RESEND_API_KEY to your [resend api key](https://resend.com/api-keys)

### 8. Start the server

```shell
mise run backend
```
