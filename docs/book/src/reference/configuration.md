# Configuration / 配置

> **Status**: Phase 2 Available ✅  
> **状态**: 第2阶段可用 ✅

Nexus provides flexible configuration management similar to Spring Boot.

Nexus 提供灵活的配置管理，类似于 Spring Boot。

---

## Overview / 概述

Configuration features:

配置功能：

- **Multiple Formats** / **多格式** - Properties, YAML, TOML, JSON
- **Environment Variables** / **环境变量** - Override with env vars
- **Profiles** / **配置文件** - Environment-specific configs
- **Hot Reload** / **热重载** - Reload without restart

---

## Configuration Files / 配置文件

### Properties / Properties

```properties
app.name=MyApp
app.port=3000
database.url=jdbc:postgresql://localhost/mydb
```

### YAML / YAML

```yaml
app:
  name: MyApp
  port: 3000
database:
  url: jdbc:postgresql://localhost/mydb
```

### TOML / TOML

```toml
[app]
name = "MyApp"
port = 3000

[database]
url = "jdbc:postgresql://localhost/mydb"
```

---

## Using Configuration / 使用配置

```rust
use nexus_config::{Config, PropertiesConfig};
use serde::Deserialize;

#[derive(PropertiesConfig, Deserialize)]
#[prefix = "app"]
struct AppConfig {
    name: String,
    port: u16,
}

// Load configuration / 加载配置
let config = Config::load()?;
let app_config: AppConfig = config.get()?;
```

---

## Environment Variables / 环境变量

Override with environment variables:

使用环境变量覆盖：

```bash
export APP_PORT=8080
export APP_DATABASE_URL=postgresql://prod/db
```

---

## Profiles / 配置文件

Environment-specific configurations:

环境特定配置：

```rust
use nexus_config::{Config, Profile};

let config = Config::builder()
    .with_profile(Profile::Production)
    .build()?;
```

**Profile Files** / **配置文件**:
- `application.yml` - Default
- `application-prod.yml` - Production
- `application-dev.yml` - Development

---

## Spring Boot Comparison / Spring Boot 对比

| Spring Boot | Nexus | Description |
|-------------|-------|-------------|
| `@ConfigurationProperties` | `PropertiesConfig` | Type-safe config |
| `@Value` | `Value` | Single value |
| `Environment` | `Environment` | Environment access |
| `@Profile` | `Profile` | Environment profiles |

---

*← [Previous / 上一页](./api.md) | [Next / 下一页](./performance.md) →*
