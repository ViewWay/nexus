# Lombok 注解快速参考卡 / Lombok Annotations Quick Reference Card

## 🎯 当前状态 / Current Status

```
┌─────────────────────────────────────────────────────────────┐
│  Lombok 注解支持进度 Lombok Annotations Progress            │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ✅ 已实现 Implemented:     1/10  (10%)                     │
│  ❌ 缺失 Missing:          9/10  (90%)                     │
│                                                              │
│  🔴 阻塞问题 Blocking:       无 None                        │
│  ⚠️  部分可用 Partial:       1/10 (@Slf4j)                 │
│                                                              │
└─────────────────────────────────────────────────────────────┘

预计完成时间 ETA: 8 weeks
优先级 Priority: 🟡 P1 (重要但非阻塞)
```

---

## 📋 完整清单 / Complete Checklist

### ✅ 已实现 / Implemented (1/10)

| # | 注解 Annotation | 状态 Status | 位置 Location |
|---|---------------|-----------|-------------|
| 1 | `#[slf4j]` | ✅ 完整 | nexus-macros/lib.rs:634 |

**使用示例**:
```rust
#[slf4j]
struct MyController {
    // 自动添加 log 字段
}

// 使用
self.log.info("message");
```

---

### ❌ 需要实施 / To Implement (9/10)

#### 🔴 P0 - 核心注解 (Week 1-4)

| # | 注解 | 功能 | 实施时间 |
|---|------|------|---------|
| 1 | `#[Data]` | Getter + Setter + Constructor + With | 2 weeks |
| 2 | `#[Getter]` | 生成 Getter 方法 | 3 days |
| 3 | `#[Setter]` | 生成 Setter 方法 | 3 days |
| 4 | `#[AllArgsConstructor]` | 全参构造函数 | 2 days |
| 5 | `#[NoArgsConstructor]` | 无参构造函数 | 2 days |

#### 🟡 P1 - 高级注解 (Week 5-7)

| # | 注解 | 功能 | 实施时间 |
|---|------|------|---------|
| 6 | `#[Builder]` | Builder 模式 | 1.5 weeks |
| 7 | `#[Value]` | 不可变类 | 1 week |
| 8 | `#[With]` | With 方法 | 3 days |

#### 🟢 P2 - 辅助注解 (Week 7-8)

| # | 注解 | 功能 | 实施时间 |
|---|------|------|---------|
| 9 | `#[Cleanup]` | 自动关闭资源 | 3 days |
| 10 | `#[SneakyThrows]` | 隐式抛出异常 | 2 days |

---

## 🚀 快速使用指南 / Quick Usage Guide

### 最常用：`#[Data]` (90% 的场景)

```rust
use nexus_lombok::Data;

#[Data]  // 一行搞定！
#[TableName("user")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[TableId(type = "auto")]
    pub id: i64,
    pub username: String,
    pub email: String,
}

// 自动生成：
// ✅ 构造函数 User::new(id, username, email)
// ✅ Getters user.username(), user.email()
// ✅ Setters user.set_username(...), user.set_email(...)
// ✅ With 方法 user.with_id(...), user.with_username(...)
```

### Builder 模式：`#[Builder]`

```rust
use nexus_lombok::Builder;

#[Builder]
pub struct User {
    pub id: i64,
    pub username: String,
}

// 使用
let user = User::builder()
    .id(1)
    .username("alice".into())
    .build()
    .unwrap();
```

### 不可变对象：`#[Value]`

```rust
use nexus_lombok::Value;

#[Value]
pub struct User {
    pub id: i64,
    pub username: String,
}

// 创建不可变对象
let user1 = User::new(1, "alice".into());

// 创建修改后的副本（with 方法）
let user2 = user1.with_username("bob".into());
// user1 保持不变！
```

---

## 📊 实施时间表 / Implementation Timeline

```
Week 1-2: ████████░░░░░░░░░░░░░ 40% 基础注解
  ├─ @Getter      (3 days)
  ├─ @Setter      (3 days)
  ├─ @AllArgs     (2 days)
  └─ @NoArgs      (2 days)

Week 3-4: ████████████████░░░░░░ 70% 核心功能
  ├─ @Data 完整版  (1 week)
  ├─ @Value        (3 days)
  └─ @With         (3 days)

Week 5-6: ██████████████████████░ 90% Builder
  └─ @Builder      (1.5 weeks)

Week 7-8: ████████████████████████ 100% 完成
  ├─ @Cleanup      (3 days)
  ├─ @SneakyThrows (2 days)
  └─ 测试 + 文档   (1 week)
```

---

## 💡 使用建议 / Usage Recommendations

### 何时使用 `#[Data]`

✅ **推荐使用**:
- Entity / 实体类
- DTO / 数据传输对象
- POJO / 简单 Java 对象

❌ **不推荐使用**:
- 性能关键代码
- 需要精确控制的方法

### 何时使用 `#[Builder]`

✅ **推荐使用**:
- 多于 5 个字段的结构体
- 可选参数较多
- 需要流畅 API

❌ **不推荐使用**:
- 少于 3 个字段（直接用构造函数）
- 所有字段都是必需的

### 何时使用 `#[Value]`

✅ **推荐使用**:
- 不可变对象
- 值对象（DDD）
- 线程安全的共享数据

❌ **不推荐使用**:
- 需要频繁修改的对象

---

## 🔄 迁移示例 / Migration Examples

### Java → Rust Lombok

#### Example 1: 简单 Entity

```java
// Java / Lombok
@Data
@Entity
@Table(name = "users")
public class User {
    @Id
    private Long id;

    @Column(name = "username")
    private String username;

    private String email;
}

// 使用
User user = new User();
user.setId(1L);
user.setUsername("alice");
String name = user.getUsername();
```

```rust
// Rust / Nexus Lombok
#[Data]  // 一行搞定！
#[TableName("users")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[TableId(type = "auto")]
    pub id: i64,

    #[TableField("username")]
    pub username: String,

    pub email: String,
}

// 使用
let mut user = User::default();
user.set_id(1);
user.set_username("alice".into());
let name = user.username();
```

#### Example 2: Builder 模式

```java
// Java / Lombok
@Builder
public class User {
    private Long id;
    private String username;
    private String email;
    private Integer age;
}

// 使用
User user = User.builder()
    .id(1L)
    .username("alice")
    .email("alice@example.com")
    .age(25)
    .build();
```

```rust
// Rust / Nexus Lombok
#[Builder]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub age: i32,
}

// 使用
let user = User::builder()
    .id(1)
    .username("alice".into())
    .email("alice@example.com".into())
    .age(25)
    .build()
    .unwrap();
```

---

## 📚 相关文档 / Related Documentation

- **完整计划**: [LOMBOK-IMPLEMENTATION.md](./LOMBOK-IMPLEMENTATION.md)
- **Spring 注解状态**: [SPRING-ANNOTATIONS-STATUS.md](./SPRING-ANNOTATIONS-STATUS.md)
- **MyBatis-Plus 风格**: [nexus-mybatis-plus-style.md](./nexus-mybatis-plus-style.md)

---

## 🎯 下一步行动 / Next Actions

### 立即开始 / Start Now (Week 1)

```bash
# 1. 创建 crate
cd crates
mkdir nexus-lombok
cd nexus-lombok
cargo init --lib

# 2. 添加依赖
# Cargo.toml
[dependencies]
syn = "1.0"
quote = "1.0"
proc-macro2 = "1.0"

[lib]
proc-macro = true

# 3. 创建第一个宏
mkdir src
touch src/getter.rs
touch src/setter.rs
touch src/lib.rs
```

### 第一个实现：`@Getter` (Day 1)

```rust
// src/lib.rs
use proc_macro::TokenStream;

#[proc_macro_derive(Getter)]
pub fn derive_getter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = extract_fields(&input);

    let getters = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            pub fn #field_name(&self) -> &#field_type {
                &self.#field_name
            }
        }
    });

    TokenStream::from(quote! {
        impl #name {
            #(#getters)*
        }
    })
}
```

---

**Last Updated**: 2026-01-25
**Status**: 🚧 Ready to Implement
**Priority**: 🟡 P1
**ETA**: 8 weeks to 100%
