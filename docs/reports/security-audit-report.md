# Nexus Framework - Security Audit Report
# Nexus 框架 - 安全审计报告

## Audit Information / 审计信息

| Field / 字段 | Value / 值 |
|-------------|-----------|
| **Date / 日期** | 2026-01-25 |
| **Version / 版本** | 0.1.0-alpha |
| **Auditor / 审计员** | Nexus Security Team |
| **Scope / 范围** | All nexus-* crates |
| **Methodology / 方法** | Static analysis, code review |

---

## Executive Summary / 执行摘要

### Overall Risk Assessment / 整体风险评估

| Level / 级别 | Count / 数量 | Description / 描述 |
|--------------|-------------|-------------------|
| 🔴 **Critical** | 3 | Requires immediate fix |
| 🟠 **High** | 5 | Should fix soon |
| 🟡 **Medium** | 4 | Plan to fix |
| 🟢 **Low** | 2 | Nice to have |

**Recommendation / 建议**: 
- Address critical vulnerabilities before production deployment
- High priority issues should be fixed in the next sprint

---

## Critical Findings / 关键发现

### 🔴 CRITICAL-1: Weak Password Hashing Fallback (弱密码哈希回退)

**Location / 位置**: `crates/nexus-security/src/encoder.rs:71-76`

**Issue / 问题**:
```rust
fn encode(&self, raw: &str) -> String {
    bcrypt::hash(raw, self.cost).unwrap_or_else(|_| {
        // FALLBACK TO MD5 - CRITICAL SECURITY ISSUE!
        use md5::{Md5, Digest};
        let hash = Md5::digest(raw.as_bytes());
        hex::encode(hash)
    })
}
```

**Risk / 风险**:
- MD5 is cryptographically broken and vulnerable to collision attacks
- Passwords hashed with MD5 can be cracked quickly using rainbow tables
- If BCrypt fails for any reason, passwords fall back to insecure hashing

**Impact / 影响**:
- User passwords stored with MD5 can be compromised
- Compliance violations (GDPR, PCI-DSS prohibit weak hashing)

**Recommendation / 建议**:
```rust
fn encode(&self, raw: &str) -> String {
    bcrypt::hash(raw, self.cost).unwrap_or_else(|e| {
        // Log error but NEVER fall back to weak hashing
        error!("Failed to hash password: {}", e);
        panic!("Password encoding failure - application cannot continue safely")
    })
}
```

**CVSS Score / CVSS 评分**: 8.5 (High)

---

### 🔴 CRITICAL-2: Incorrect PBKDF2 Implementation (错误的 PBKDF2 实现)

**Location / 位置**: `crates/nexus-security/src/encoder.rs:237-252`

**Issue / 问题**:
```rust
let mut mac = Hmac::<Sha256>::new_from_slice(raw.as_bytes()).unwrap();
mac.update(&salt);

// This is NOT proper PBKDF2 iteration!
for _ in 1..self.iterations {
    mac.update(b"\0");  // Just adding null bytes, not re-hashing
}
```

**Risk / 风险**:
- The iteration loop doesn't actually re-hash - it just adds null bytes
- Reduces the effective work factor from 100,000 to essentially 1
- Makes passwords vulnerable to brute force attacks

**Impact / 影响**:
- Passwords stored with Pbkdf2PasswordEncoder are much weaker than intended
- False sense of security

**Recommendation / 建议**:
Use the `pbkdf2` crate for proper implementation:
```rust
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

let mut key = vec![0u8; self.key_length];
pbkdf2_hmac::<Sha256>(
    raw.as_bytes(),
    &salt,
    self.iterations as usize,
    &mut key,
);
```

**CVSS Score / CVSS 评分**: 7.8 (High)

---

### 🔴 CRITICAL-3: MD5 Used for Remember Me (MD5 用于记住我功能)

**Location / 位置**: `crates/nexus-security/src/auth.rs:405-419`

**Issue / 问题**:
```rust
pub fn new(key: &str) -> Self {
    use md5::{Md5, Digest};
    let hash = Md5::digest(key.as_bytes());
    Self { key_hash: hex::encode(hash) }
}
```

**Risk / 风险**:
- MD5 is cryptographically broken
- Remember me tokens can be forged if the key is guessed
- Session hijacking risk

**Impact / 影响**:
- Attacker who can observe a remember me token could potentially forge new tokens
- Extended session access after compromise

**Recommendation / 建议**:
```rust
use sha2::{Sha256, Digest};
use hmac::Hmac;
use hmac::Mac;

pub fn new(key: &str) -> Self {
    let mut mac = Hmac::<Sha256>::new_from_slice(REMEMBER_ME_SECRET_KEY.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(key.as_bytes());
    let result = mac.finalize().into_bytes();
    Self { key_hash: hex::encode(result) }
}
```

**CVSS Score / CVSS 评分**: 7.2 (High)

---

## High Severity Findings / 高危发现

### 🟠 HIGH-1: No Input Size Limits (无输入大小限制)

**Location / 位置**: `crates/nexus-extractors/src/json.rs:97-129`

**Issue / 问题**:
```rust
// No size limit check on body_bytes
let body = body_bytes.ok_or_else(|| {
    ExtractorError::Invalid("Request body is not available".to_string())
})?;
```

While `DEFAULT_JSON_LIMIT` is defined (10MB), it's not enforced in the extractor.

**Risk / 风险**:
- DoS via large payload attacks
- Memory exhaustion
- Server crash

**Recommendation / 建议**:
```rust
const MAX_JSON_SIZE: usize = 10 * 1024 * 1024; // 10MB

let body = body_bytes.ok_or_else(|| {
    ExtractorError::Invalid("Request body is not available".to_string())
})?;

if body.len() > MAX_JSON_SIZE {
    return Err(ExtractorError::TooLarge);
}
```

**CVSS Score / CVSS 评分**: 7.5 (High)

---

### 🟠 HIGH-2: unwrap() in Security Code (安全代码中的 unwrap)

**Location / 位置**: Multiple locations in `nexus-security`

**Issue / 问题**:
```rust
// encoder.rs:237
let mut mac = Hmac::<Sha256>::new_from_slice(raw.as_bytes()).unwrap();

// auth.rs:454
let authenticated = result.unwrap();
```

**Risk / 风险**:
- Potential panic in authentication code
- Could leak information via panic messages
- Denial of service

**Recommendation / 建议**:
Replace all `unwrap()` with proper error handling in security-sensitive code.

**CVSS Score / CVSS 评分**: 6.8 (Medium)

---

### 🟠 HIGH-3: Timing Attack Vulnerability in Password Comparison (密码比较的时间攻击漏洞)

**Location / 位置**: `nexus-security/src/encoder.rs:109-111`

**Issue / 问题**:
```rust
// NoOpPasswordEncoder - Used for testing only but accessible!
impl PasswordEncoder for NoOpPasswordEncoder {
    fn matches(&self, raw: &str, encoded: &str) -> bool {
        raw == encoded  // NOT constant-time comparison!
    }
}
```

**Risk / 风险**:
- String comparison is not constant-time
- Vulnerable to timing attacks
- NoOpPasswordEncoder should NEVER be used in production

**Recommendation / 建议**:
1. Add compile-time feature flag:
```rust
#[cfg(feature = "testing_only")]
pub struct NoOpPasswordEncoder;
```

2. Use constant-time comparison:
```rust
use subtle::ConstantTimeEq;
raw.ct_eq(encoded.as_bytes()).into()
```

**CVSS Score / CVSS 评分**: 6.5 (Medium)

---

### 🟠 HIGH-4: Potential Username Enumeration (潜在的用户名枚举)

**Location / 位置**: `nexus-security/src/auth.rs:314-323`

**Issue / 问题**:
```rust
// Good: hide_user_not_found prevents enumeration
if self.hide_user_not_found {
    return Err(SecurityError::InvalidCredentials(
        "Invalid credentials".to_string(),
    ));
}
```

This is actually implemented correctly, but the default should be `true` and it should not be configurable.

**Risk / 风险**:
- If someone sets `hide_user_not_found = false`, usernames can be enumerated
- Valuable information for attackers

**Recommendation / 建议**:
Remove the option to disable this security feature:
```rust
// Always hide user not found - make it non-optional
let user = match self.user_service.load_user_by_username(username).await {
    Ok(u) => u,
    Err(_) => {
        // Always return the same error
        return Err(SecurityError::InvalidCredentials(
            "Invalid credentials".to_string(),
        ));
    }
};
```

**CVSS Score / CVSS 评分**: 5.3 (Medium)

---

### 🟠 HIGH-5: No Request Rate Limiting on Auth Endpoints (认证端点无请求速率限制)

**Issue / 问题**:
Authentication endpoints don't have built-in rate limiting.

**Risk / 飁险**:
- Brute force password attacks
- Credential stuffing
- DoS on authentication services

**Recommendation / 建议**:
```rust
use nexus_resilience::rate_limit::RateLimiter;

// Built-in rate limiter for auth
let auth_rate_limiter = RateLimiter::token_bucket(
    RateLimitConfig::new()
        .capacity(5)       // 5 attempts
        .refill_rate(1)   // 1 per minute
);
```

**CVSS Score / CVSS 评分**: 6.0 (Medium)

---

## Medium Severity Findings / 中危发现

### 🟡 MEDIUM-1: Missing Content-Type Validation (缺少 Content-Type 验证)

**Location / 位置**: `nexus-extractors/src/json.rs:110-118`

**Issue / 问题**:
```rust
if !content_type.starts_with("application/json")
    && !content_type.starts_with("application/")
    && !content_type.is_empty()
{
    return Err(...);
}
```

Allows any `application/*` content type, which is too permissive.

**Recommendation / 建议**:
```rust
const VALID_CONTENT_TYPES: &[&str] = &[
    "application/json",
    "application/json; charset=utf-8",
    "text/json",
];

if !VALID_CONTENT_TYPES.iter().any(|&ct| {
    content_type.to_lowercase().starts_with(ct)
}) {
    return Err(ExtractorError::InvalidContentType);
}
```

---

### 🟡 MEDIUM-2: No CSRF Protection (无 CSRF 保护)

**Issue / 问题**:
Framework doesn't include built-in CSRF protection for state-changing operations.

**Recommendation / 建议**:
Add CSRF token middleware:
```rust
pub struct CsrfMiddleware {
    token_length: usize,
    secure_cookie: bool,
}

impl Middleware for CsrfMiddleware {
    async fn call(&self, req: Request, next: Next) -> Result<Response, Error> {
        // Validate CSRF token for POST/PUT/DELETE/PATCH
        if matches!(req.method(), Method::POST | Method::PUT | Method::DELETE | Method::PATCH) {
            self.validate_token(req)?;
        }
        next.run(req).await
    }
}
```

---

### 🟡 MEDIUM-3: Insufficient Logging for Security Events (安全事件日志不足)

**Issue / 问题**:
Security events (login attempts, failures, suspicious activity) are not logged by default.

**Recommendation / 建议**:
```rust
pub struct SecurityAuditLogger {
    logger: Logger,
}

impl SecurityAuditLogger {
    pub fn log_login_attempt(&self, username: &str, success: bool, ip: &str) {
        if !success {
            // ALERT on failed login attempts
            self.logger.warn()
                .field("event", "auth_failed")
                .field("username", username)
                .field("ip", ip)
                .message("Authentication failed")
                .log();
        }
    }
}
```

---

### 🟡 MEDIUM-4: No HTTPS Enforcement (无 HTTPS 强制)

**Issue / 问题**:
Framework doesn't enforce HTTPS connections or HSTS headers.

**Recommendation / 建议**:
```rust
pub struct HstsMiddleware {
    max_age: Duration,
    include_subdomains: bool,
}

impl Middleware for HstsMiddleware {
    async fn call(&self, req: Request, next: Next) -> Result<Response, Error> {
        let mut response = next.run(req).await?;
        response.headers_mut().insert(
            "Strict-Transport-Security",
            format!("max-age={}, includeSubDomains", self.max_age.as_secs())
        );
        Ok(response)
    }
}
```

---

## Low Severity Findings / 低危发现

### 🟢 LOW-1: Generic Error Messages (通用错误消息)

**Issue / 问题**:
Some error messages are too generic, making debugging harder for legitimate users.

**Recommendation / 建议**:
Balance security with usability - log detailed errors server-side but return generic messages to clients.

---

### 🟢 LOW-2: Missing Security Headers (缺少安全头)

**Issue / 问题**:
Framework doesn't add recommended security headers by default.

**Recommendation / 建议**:
```rust
pub struct SecurityHeadersMiddleware;

impl Middleware for SecurityHeadersMiddleware {
    async fn call(&self, req: Request, next: Next) -> Result<Response, Error> {
        let mut response = next.run(req).await?;
        let headers = response.headers_mut();
        
        headers.insert("X-Content-Type-Options", "nosniff");
        headers.insert("X-Frame-Options", "DENY");
        headers.insert("X-XSS-Protection", "1; mode=block");
        headers.insert("Content-Security-Policy", "default-src 'self'");
        headers.insert("Referrer-Policy", "strict-origin-when-cross-origin");
        headers.insert("Permissions-Policy", "geolocation=(), microphone=()");
        
        Ok(response)
    }
}
```

---

## Compliance Assessment / 合规评估

### OWASP Top 10 (2021) Coverage / 覆盖情况

| Risk / 风险 | Status / 状态 | Notes / 说明 |
|-------------|---------------|-------------|
| A01:2021 – Broken Access Control | ⚠️ Partial | Authorization implemented, needs audit |
| A02:2021 – Cryptographic Failures | ❌ Critical | MD5 usage, weak PBKDF2 |
| A03:2021 – Injection | ✅ Good | Using prepared statements (data layer) |
| A04:2021 – Insecure Design | ⚠️ Partial | Security headers missing |
| A05:2021 – Security Misconfiguration | ⚠️ Partial | Debug modes need review |
| A06:2021 – Vulnerable Components | ✅ Good | Dependencies audited |
| A07:2021 – Auth Failures | ⚠️ Partial | No rate limiting on auth |
| A08:2021 – Data Integrity Failures | ✅ Good | No signature issues |
| A09:2021 – Logging Failures | ⚠️ Partial | Security logging incomplete |
| A10:2021 – SSRF | ✅ Good | No URL fetching in user input |

---

## Recommendations Summary / 建议摘要

### Immediate Actions (Next Sprint) / 立即行动（下个冲刺）

1. **Fix CRITICAL-1**: Remove MD5 fallback from password encoder
2. **Fix CRITICAL-2**: Fix PBKDF2 implementation or remove it
3. **Fix CRITICAL-3**: Replace MD5 with SHA-256 for remember me
4. **Implement HIGH-1**: Add request size limits
5. **Implement HIGH-5**: Add rate limiting to auth endpoints

### Short-term (Next Month) / 短期（下月）

1. Fix all `unwrap()` in security code
2. Add CSRF protection
3. Implement security audit logging
4. Add security headers middleware
5. Add HSTS enforcement

### Long-term (Next Quarter) / 长期（下季度）

1. Security testing suite
2. Penetration testing engagement
3. Security documentation
4. Compliance certification preparation

---

## Security Best Practices / 安全最佳实践

### For Users / 对于用户

1. **Always use BCrypt** for password hashing (default is good)
2. **Never use NoOpPasswordEncoder** in production
3. **Enable rate limiting** on all authentication endpoints
4. **Use HTTPS** in production
5. **Set `hide_user_not_found = true`** (default)
6. **Implement security headers** middleware
7. **Log security events** for audit
8. **Validate and sanitize** all user input

### Security Checklist / 安全检查清单

Before deploying to production:
- [ ] Review and update all cryptographic implementations
- [ ] Enable rate limiting on auth endpoints
- [ ] Configure security headers
- [ ] Set up security audit logging
- [ ] Review CORS configuration
- [ ] Test for common vulnerabilities (SQLi, XSS, CSRF)
- [ ] Review dependencies for known vulnerabilities
- [ ] Enable HTTPS only
- [ ] Review error messages for information leakage
- [ ] Configure session timeout appropriately

---

## Conclusion / 结论

The Nexus framework has a **solid security foundation** with proper password hashing (BCrypt), authentication, and authorization structures. However, there are **3 critical vulnerabilities** that must be addressed before production deployment.

**Key Strengths / 关键优势**:
- ✅ Strong authentication framework
- ✅ Authorization with roles and permissions
- ✅ BCrypt password hashing (when it works)
- ✅ Password hiding for user enumeration prevention
- ✅ Secure credential clearing after auth

**Key Weaknesses / 关键弱点**:
- ❌ MD5 fallback in password encoder
- ❌ Incorrect PBKDF2 implementation
- ❌ MD5 in remember me tokens
- ❌ Missing rate limiting on auth
- ❌ No built-in CSRF protection

**Overall Security Rating / 整体安全评级**: ⚠️ **B- (Good with Critical Issues)**

Once the critical issues are resolved, this would be an **A-** security framework.

---

**Report Generated**: 2026-01-25
**Next Audit Recommended**: After critical fixes are deployed
