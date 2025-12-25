# Axum 后端

<h1><a href="./README.md">English</a> | 简体中文</h1>

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

### 2. 在Docker中启动SurrealDB服务器

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

### 3. 在Docker中启动Redis服务器

```shell
docker run --name redis \
  -p 6379:6379 \
  -v ~/redis:/data \
  -d redis:8.4.0
```

### 4. 使用mise安装surrealdb-migrations

```shell
mise run prepare
```

### 5. 使用mise安装mise.toml中的工具

```shell
mise install
```

### 6. 迁移数据到你的SurrealDB中

```shell
mise run migration
```

### 7. 配置 .env

```shell
cp .env.example .env
```

- 替换 SYSTEM_OWNER_NAME 为你的名字
- 替换 SYSTEM_OWNER_EMAIL 为你的邮箱
- 替换 SYSTEM_OWNER_PASSWORD 为你的密码
- 替换 JWT_SECRET 为你的JWT密钥
- 替换 FROM_EMAIL 为你的邮箱
- 替换 RESEND_API_KEY 为你的 [resend api key](https://resend.com/api-keys)

### 8. 启动服务

```shell
mise run backend
```
