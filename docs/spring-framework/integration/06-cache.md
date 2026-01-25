# 缓存抽象

自 3.1 版本以来，Spring Framework 提供了对现有 Spring 应用程序透明添加缓存的支持。与事务支持类似，缓存抽象允许一致地使用各种缓存解决方案，同时对代码的影响最小。

在 Spring Framework 4.1 中，缓存抽象得到了显著扩展，增加了对 JSR-107 注解和更多自定义选项的支持。

## 章节摘要

- 理解缓存抽象
- 基于声明式注解的缓存
- JCache (JSR-107) 注解
- 基于声明式 XML 的缓存
- 配置缓存存储
- 接入不同的后端缓存
- 如何设置 TTL/TTI/逐出策略/XXX 功能？

## 理解缓存抽象

Spring 的缓存抽象的核心思想是将缓存应用到 Java 方法上，从而根据方法参数缓存方法执行的结果。这样，当使用相同参数再次调用该方法时，可以直接从缓存中返回结果，而无需实际执行方法。

缓存抽象应用于方法上，重点关注根据参数缓存结果。这意味着：
- 对于给定的参数，方法只执行一次
- 后续调用使用相同参数时，直接从缓存返回结果
- 方法执行逻辑对外部透明

### 核心抽象

缓存抽象围绕两个核心接口构建：

1. **`Cache`** - 缓存抽象，表示缓存存储
2. **`CacheManager`** - 缓存管理器，用于管理和访问缓存

```java
public interface Cache {
    String getName();
    Object getNativeCache();
    ValueWrapper get(Object key);
    <T> T get(Object key, Class<T> type);
    <T> T get(Object key, Callable<T> valueLoader);
    void put(Object key, Object value);
    ValueWrapper putIfAbsent(Object key, Object value);
    void evict(Object key);
    boolean evictIfPresent(Object key);
    void clear();
}
```

```java
public interface CacheManager {
    Cache getCache(String name);
    Collection<String> getCacheNames();
}
```

## 基于声明式注解的缓存

Spring 提供了一组注解，可以声明性地定义缓存行为。

### 启用缓存注解

要启用缓存注解支持，需要在配置类上添加 `@EnableCaching` 注解：

```java
@Configuration
@EnableCaching
public class CacheConfig {
}
```

### `@Cacheable` 注解

`@Cacheable` 注解用于标记方法的返回值应该被缓存：

```java
@Cacheable("books")
public Book findBook(String isbn) {
    // 方法实现
}
```

### `@CachePut` 注解

`@CachePut` 注解用于更新缓存，但不会跳过方法执行：

```java
@CachePut("books")
public Book updateBook(Book book) {
    // 更新书籍
    return book;
}
```

### `@CacheEvict` 注解

`@CacheEvict` 注解用于从缓存中移除数据：

```java
@CacheEvict("books")
public void deleteBook(String isbn) {
    // 删除书籍
}
```

清空整个缓存：

```java
@CacheEvict(value = "books", allEntries = true)
public void deleteAllBooks() {
    // 删除所有书籍
}
```

### `@Caching` 注解

`@Caching` 注解用于组合多个缓存注解：

```java
@Caching(
    cacheable = @Cacheable("books"),
    put = @CachePut(value = "bookSummaries", key = "#result.isbn")
)
public Book findBookWithSummary(String isbn) {
    // 方法实现
}
```

### `@CacheConfig` 注解

`@CacheConfig` 是类级别的注解，用于共享缓存相关的设置：

```java
@Service
@CacheConfig(cacheNames = "books")
public class BookService {

    @Cacheable(key = "#isbn")
    public Book findBook(String isbn) {
        // 方法实现
    }

    @CacheEvict(key = "#book.isbn")
    public void updateBook(Book book) {
        // 更新书籍
    }
}
```

## JCache (JSR-107) 注解

Spring 还支持 JSR-107 (JCache) 注解：

- `@CacheResult` - 类似于 `@Cacheable`
- `@CachePut` - 更新缓存
- `@CacheRemove` - 从缓存中移除条目
- `@CacheRemoveAll` - 清空缓存

```java
@CacheResult(cacheName = "books")
public Book findBook(String isbn) {
    // 方法实现
}
```

## 基于声明式 XML 的缓存

```xml
<beans xmlns="http://www.springframework.org/schema/beans"
       xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
       xmlns:cache="http://www.springframework.org/schema/cache"
       xsi:schemaLocation="
           http://www.springframework.org/schema/beans
           https://www.springframework.org/schema/beans/spring-beans.xsd
           http://www.springframework.org/schema/cache
           https://www.springframework.org/schema/cache/spring-cache.xsd">

    <cache:annotation-driven/>

    <bean id="cacheManager" class="org.springframework.cache.concurrent.ConcurrentMapCacheManager"/>

</beans>
```

## 配置缓存存储

### ConcurrentMapCacheManager

基于内存的简单缓存管理器：

```java
@Bean
public CacheManager cacheManager() {
    return new ConcurrentMapCacheManager("books", "authors");
}
```

### CaffeineCacheManager

基于 Caffeine 的高性能缓存：

```java
@Bean
public CacheManager cacheManager() {
    CaffeineCacheManager cacheManager = new CaffeineCacheManager();
    cacheManager.setCaffeine(Caffeine.newBuilder()
        .expireAfterWrite(10, TimeUnit.MINUTES)
        .maximumSize(1000));
    return cacheManager;
}
```

### RedisCacheManager

基于 Redis 的分布式缓存：

```java
@Bean
public RedisCacheManager cacheManager(RedisConnectionFactory factory) {
    return RedisCacheManager.builder(factory)
        .cacheDefaults(RedisCacheConfiguration.defaultCacheConfig()
            .entryTtl(Duration.ofMinutes(10)))
        .build();
}
```

## 接入不同的后端缓存

Spring 的缓存抽象支持多种缓存后端：

- **简单缓存**: ConcurrentMapCacheManager、Caffeine
- **分布式缓存**: Redis、Hazelcast
- **兼容 JSR-107**: 任何 JSR-107 兼容的实现

### 自定义 CacheManager

```java
@Bean
public CacheManager cacheManager() {
    SimpleCacheManager cacheManager = new SimpleCacheManager();
    cacheManager.setCaches(Arrays.asList(
        new ConcurrentMapCache("books"),
        new ConcurrentMapCache("authors")
    ));
    return cacheManager;
}
```

## 条件缓存

可以使用 `condition` 参数来定义缓存的条件：

```java
@Cacheable(value = "books", condition = "#isbn.length() > 10")
public Book findBook(String isbn) {
    // 方法实现
}
```

使用 `unless` 参数基于返回值排除缓存：

```java
@Cacheable(value = "books", unless = "#result.price > 100")
public Book findBook(String isbn) {
    // 方法实现
}
```

## 自定义 Key 生成

可以使用 SpEL 表达式自定义缓存键：

```java
@Cacheable(value = "books", key = "#isbn")
public Book findBook(String isbn) {
    // 方法实现
}

@Cacheable(value = "books", key = "#isbn + '-' + #edition")
public Book findBookEdition(String isbn, int edition) {
    // 方法实现
}
```

自定义 KeyGenerator：

```java
@Configuration
@EnableCaching
public class CacheConfig {

    @Bean
    public CacheManager cacheManager() {
        return new ConcurrentMapCacheManager("books");
    }

    @Bean
    public KeyGenerator customKeyGenerator() {
        return (target, method, params) -> {
            StringBuilder sb = new StringBuilder();
            sb.append(target.getClass().getSimpleName());
            sb.append(".");
            sb.append(method.getName());
            sb.append(".");
            for (Object param : params) {
                sb.append(param.toString());
            }
            return sb.toString();
        };
    }
}
```

## 同步缓存

从 Spring 4.3 开始，支持同步缓存以防止并发未命中时的"惊群效应"：

```java
@Cacheable(value = "books", sync = true)
public Book findBook(String isbn) {
    // 方法实现
}
```
