//! Tests for #[Data] derive macro
//! #[Data] 派生宏测试

use nexus_lombok::Data;

#[test]
fn test_data_macro() {
    #[derive(Data, Clone, PartialEq, Debug)]
    struct User {
        id: i64,
        username: String,
        email: String,
    }

    // Test constructor / 测试构造函数
    let user = User::new(1, "alice".into(), "alice@example.com".into());
    assert_eq!(user.id, 1);
    assert_eq!(user.username, "alice");

    // Test getters / 测试 getters
    assert_eq!(user.id(), 1);
    assert_eq!(user.username(), "alice");

    // Test setters / 测试 setters
    let mut user = User::new(0, String::new(), String::new());
    user.set_id(2);
    user.set_username("bob".into());
    assert_eq!(user.id, 2);
    assert_eq!(user.username, "bob");

    // Test with methods / 测试 with 方法
    let user2 = user.with_id(3).with_username("charlie".into());
    assert_eq!(user2.id, 3);
    assert_eq!(user2.username, "charlie");
    // Original should remain unchanged / 原始对象应保持不变
    assert_eq!(user.id, 2);
}

#[test]
fn test_getter_macro() {
    use nexus_lombok::Getter;

    #[derive(Getter)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    assert_eq!(point.x(), 10);
    assert_eq!(point.y(), 20);
}

#[test]
fn test_setter_macro() {
    use nexus_lombok::Setter;

    #[derive(Setter)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut point = Point { x: 0, y: 0 };
    point.set_x(10);
    point.set_y(20);
    assert_eq!(point.x, 10);
    assert_eq!(point.y, 20);
}

#[test]
fn test_all_args_constructor() {
    use nexus_lombok::AllArgsConstructor;

    #[derive(AllArgsConstructor, PartialEq, Debug)]
    struct User {
        id: i64,
        username: String,
    }

    let user = User::new(1, "alice".into());
    assert_eq!(user.id, 1);
    assert_eq!(user.username, "alice");
}

#[test]
fn test_no_args_constructor() {
    use nexus_lombok::NoArgsConstructor;

    #[derive(NoArgsConstructor, PartialEq, Debug)]
    struct User {
        id: i64,
        username: String,
    }

    let user = User::new();
    assert_eq!(user.id, 0);
    assert_eq!(user.username, "");
}

#[test]
fn test_value_macro() {
    use nexus_lombok::Value;

    #[derive(Value, Clone, PartialEq, Debug)]
    struct User {
        id: i64,
        username: String,
    }

    let user1 = User::new(1, "alice".into());

    // Test getters / 测试 getters
    assert_eq!(user1.id(), 1);
    assert_eq!(user1.username(), "alice");

    // Test with methods (creates new instance) / 测试 with 方法（创建新实例）
    let user2 = user1.with_id(2);
    assert_eq!(user2.id(), 2);
    // Original should remain unchanged / 原始对象应保持不变
    assert_eq!(user1.id(), 1);
}

#[test]
fn test_builder_macro() {
    use nexus_lombok::Builder;

    #[derive(Builder, PartialEq, Debug)]
    struct User {
        id: i64,
        username: String,
        email: String,
    }

    let user = User::builder()
        .id(1)
        .username("alice".into())
        .email("alice@example.com".into())
        .build()
        .unwrap();

    assert_eq!(user.id, 1);
    assert_eq!(user.username, "alice");
    assert_eq!(user.email, "alice@example.com");
}

#[test]
fn test_with_macro() {
    use nexus_lombok::With;

    #[derive(With, Clone, PartialEq, Debug)]
    struct User {
        id: i64,
        username: String,
    }

    let user1 = User { id: 1, username: "alice".into() };
    let user2 = user1.with_id(2);
    assert_eq!(user2.id, 2);
    assert_eq!(user1.id, 1); // Original unchanged / 原始未改变
}
