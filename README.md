<h1 style="text-align: center">Axum Backend</h1>

<h2 style="text-align: center">English | <a href="./README_zh_CN.md">简体中文</a></h2>

## Features

## Preparations

To run this project, you will need:

- [Rust](https://rust-lang.org/tools/install/) installed on your machine.
- [Docker](https://www.docker.com/) installed on your machine.
- [mise](https://mise.jdx.dev/getting-started.html) installed on your machine.

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

### 4. Start RustFS server on docker

```shell
docker run -d \
  --name rustfs \
  -p 9000:9000 \
  -p 9001:9001 \
  -v ~/rustfs/data:/data \
  rustfs/rustfs:1.0.0-alpha.70
```

### 5. Install surrealdb-migrations by mise

```shell
mise run prepare
```

### 6. Install tools in mise.toml by mise

```shell
mise install
```

### 7. Apply migrations to your SurrealDB

```shell
mise run migration
```

### 8. Configure .env

```shell
cp .env.example .env
```

- replace SYSTEM_OWNER_NAME to your name
- replace SYSTEM_OWNER_EMAIL to your email
- replace SYSTEM_OWNER_PASSWORD to your password
- replace JWT_SECRET to your jwt secret
- replace FROM_EMAIL to your email
- replace RESEND_API_KEY to your [resend api key](https://resend.com/api-keys)

### 9. Start the server

```shell
mise run backend
```
