<h1 style="text-align: center">Axum 后端</h1>

<h2 style="text-align: center"><a href="./README.md">English</a> | 简体中文</h2>

## 功能

## 准备

要运行此项目，你需要：

- [Rust](https://rust-lang.org/tools/install/) 安装在你的电脑上。
- [Docker](https://www.docker.com/) 安装在你的电脑上。
- [mise](https://mise.jdx.dev/getting-started.html) 安装在你的电脑上。

## 安装

### 1. 克隆该仓库

```shell
git clone https://github.com/DarcyJason/AxumBackend
```

### 2. 在 Docker 中启动 SurrealDB 服务器

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

### 3. 在 Docker 中启动 Redis 服务器

```shell
docker run --name redis \
  -p 6379:6379 \
  -v ~/redis:/data \
  -d redis:8.4.0
```

### 4. 在 Docker 中启动 RustFS 服务器

```shell
docker run -d \
  --name rustfs \
  -p 9000:9000 \
  -p 9001:9001 \
  -v ~/rustfs/data:/data \
  rustfs/rustfs:1.0.0-alpha.70
```

### 5. 使用 mise 安装 surrealdb-migrations

```shell
mise run prepare
```

### 6. 使用 mise 安装 mise.toml 中的工具

```shell
mise install
```

### 7. 迁移数据到你的 SurrealDB 中

```shell
mise run migration
```

### 8. 配置 .env

```shell
cp .env.example .env
```

- 替换 SYSTEM_OWNER_NAME 为你的名字
- 替换 SYSTEM_OWNER_EMAIL 为你的邮箱
- 替换 SYSTEM_OWNER_PASSWORD 为你的密码
- 替换 JWT_SECRET 为你的 JWT 密钥
- 替换 FROM_EMAIL 为你的邮箱
- 替换 RESEND_API_KEY 为你的 [resend api key](https://resend.com/api-keys)

### 9. 启动服务

```shell
mise run backend
```
