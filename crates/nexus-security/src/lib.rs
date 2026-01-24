//! Nexus Security - Security framework module
//! Nexus安全 - 安全框架模块
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - `@PreAuthorize` - PreAuthorize
//! - `@Secured` - Secured
//! - `@RolesAllowed` - RolesAllowed
//! - `UserDetails` - User
//! - `GrantedAuthority` - Permission/Role
//! - `Authentication` - Auth
//! - `SecurityContext` - SecurityContext
//!
//! # Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_security::{PreAuthorize, Secured, User, Role};
//!
//! struct UserService {
//!     // ... fields
//! }
//!
//! impl UserService {
//!     #[pre_authorize("hasRole('ADMIN')")]
//!     async fn delete_user(&self, id: u64) -> Result<(), Error> {
//!         // Only accessible by users with ADMIN role
//!         Ok(())
//!     }
//!
//!     #[secured("ROLE_USER")]
//!     async fn get_profile(&self) -> Result<Profile, Error> {
//!         // Only accessible by authenticated users
//!         Ok(Profile::default())
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

mod auth;
mod authority;
mod context;
mod encoder;
mod error;
mod pre_authorize;
mod role;
mod secured;
mod user;

pub use auth::{Authentication, AuthenticationManager};
pub use authority::{Authority, GrantedAuthority};
pub use context::SecurityContext;
pub use encoder::PasswordEncoder;
pub use error::{SecurityError, SecurityResult};
pub use pre_authorize::{PreAuthorize, SecurityExpression};
pub use role::{Role, Roles};
pub use secured::{Secured, SecuredHelper, SecurityMetadata};
pub use user::{User, UserDetails, UserService};

/// Re-exports of commonly used types
/// 常用类型的重新导出
pub mod prelude {
    pub use super::{
        Authentication, AuthenticationManager, Authority, GrantedAuthority, PasswordEncoder,
        PreAuthorize, Role, Role as Roles, SecurityContext, Secured, SecurityExpression, User,
        UserDetails, UserService,
    };
}

/// Version of the security module
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default role prefix
/// 默认角色前缀
pub const DEFAULT_ROLE_PREFIX: &str = "ROLE_";

/// Anonymous user principal
/// 匿名用户主体
pub const ANONYMOUS_USER: &str = "anonymousUser";

/// Remember me key
/// 记住我密钥
pub const REMEMBER_ME_KEY: &str = "remember_me";
