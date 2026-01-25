# Kotlin

Spring Framework provides first-class support for Kotlin, letting developers write Spring applications with Kotlin code and idioms. The reference documentation contains dedicated sections that cover Kotlin's support:

- Requirements
- Extensions
- Null-safety
- Classes and interfaces
- Annotations
- Bean Definition DSL
- Web
- Coroutines
- Spring Projects in Kotlin
- Getting Started

## Requirements

Spring Framework supports Kotlin 1.7+.

### Kotlin Gradle Plugin

To use Kotlin, you need to add the Kotlin Gradle plugin to your build:

```groovy
// build.gradle.kts
plugins {
    kotlin("jvm") version "1.9.0"
}
```

### Dependencies

Spring Framework requires the following Kotlin dependencies:

- `kotlin-stdlib`: Core Kotlin standard library
- `kotlin-reflect`: Kotlin reflection library (required for Spring's reflection-based features)

```groovy
// build.gradle.kts
dependencies {
    implementation("org.jetbrains.kotlin:kotlin-stdlib")
    implementation("org.jetbrains.kotlin:kotlin-reflect")
}
```

### Jackson Kotlin Module

If you're using Jackson for JSON serialization, you should add the Jackson Kotlin module for better Kotlin support:

```groovy
// build.gradle.kts
dependencies {
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin")
}
```

## Extensions

Spring Framework provides Kotlin extensions for existing Spring APIs. These extensions leverage Kotlin's extension functions to provide more idiomatic APIs for Kotlin developers.

### Reified Type Parameters

Java generics have type erasure, which means you cannot access generic type information at runtime. Kotlin's reified type parameters allow you to access generic type information at runtime when using inline functions.

Spring provides extension functions that use reified type parameters to simplify API usage:

```kotlin
// Example with reified type parameters
class UserRepository {

    @Autowired
    lateinit var entityManager: EntityManager

    fun findUserById(id: Long): User? {
        // Reified type parameter allows accessing User type at runtime
        return entityManager.find(User::class.java, id)
    }
}
```

### Kotlin-specific Extensions

Spring Framework includes many Kotlin-specific extensions that provide more concise and idiomatic APIs:

```kotlin
// RestTemplate extensions
val restTemplate = RestTemplate()
val users: List<User> = restTemplate.getForObject(URI("https://api.example.com/users"))

// WebClient extensions (Spring WebFlux)
val webClient = WebClient.create("https://api.example.com")
val users: Flux<User> = webClient.get().uri("/users").retrieve().bodyToFlow()
```

## Null-safety

Spring Framework provides null-safety annotations to help Kotlin developers write safer code. These annotations are used by the Kotlin compiler to provide null-safety guarantees.

### JSR-305 Annotations

Spring Framework uses JSR-305 annotations (`@NonNull`, `@Nullable`, etc.) to declare nullability of API elements. Kotlin recognizes these annotations and provides null-safety checks based on them.

```kotlin
import org.springframework.lang.NonNull
import org.springframework.lang.Nullable

class UserService {

    // Compiler will enforce non-null return value
    @NonNull
    fun getUser(id: Long): User {
        // Implementation must return non-null User
        return userRepository.findById(id).orElseThrow()
    }

    // Kotlin compiler will enforce null check
    fun printUser(@Nullable user: User?) {
        // Must handle null case
        user?.let { println(it.name) }
    }
}
```

### Spring's Nullability Annotations

Spring Framework provides its own nullability annotations in the `org.springframework.lang` package:

- `@NonNull`: The value cannot be null
- `@Nullable`: The value can be null

```kotlin
import org.springframework.lang.NonNull
import org.springframework.lang.Nullable

@Component
class UserService(@NonNull val repository: UserRepository) {

    fun findUser(@NonNull id: Long): @Nullable User? {
        return repository.findById(id).orElse(null)
    }
}
```

## Classes and Interfaces

Spring Framework's support for Kotlin includes special handling for Kotlin classes and interfaces.

### Primary Constructor Instantiation

Kotlin's primary constructor is preferred over secondary constructors. Spring Framework automatically detects and uses the primary constructor for dependency injection:

```kotlin
// Preferred - using primary constructor
@Service
class UserService(private val userRepository: UserRepository) {
    // Spring will inject UserRepository through the primary constructor
}
```

### Immutable Classes

Kotlin data classes are immutable by default when all properties are declared as `val`. Spring Framework supports immutable classes for dependency injection and configuration:

```kotlin
// Immutable data class
data class User(
    val id: Long,
    val name: String,
    val email: String
)

@Service
class UserService {

    // Immutable service class
    fun createUser(name: String, email: String): User {
        return User(
            id = generateId(),
            name = name,
            email = email
        )
    }
}
```

### Parameter Names

Kotlin preserves parameter names in bytecode by default (using the `-java-parameters` compiler flag). Spring Framework can use these parameter names for autowiring by name:

```kotlin
@Repository
class UserRepository(private val dataSource: DataSource) {
    // Spring will use the parameter name "dataSource" for autowiring
}
```

## Annotations

Kotlin's annotation syntax differs slightly from Java. Spring Framework provides support for Kotlin's annotation syntax.

### Required Parameters Based on Nullability

Spring Framework uses nullability annotations to determine if annotation parameters are required:

```kotlin
import org.springframework.web.bind.annotation.*

@RestController
@RequestMapping("/api/users")
class UserController {

    // Required parameter (non-null)
    @GetMapping("/{id}")
    fun getUser(@PathVariable id: Long): User {
        return userService.findById(id)
    }

    // Optional parameter (nullable)
    @GetMapping
    fun searchUsers(
        @RequestParam(required = false) name: String?
    ): List<User> {
        return if (name != null) {
            userService.searchByName(name)
        } else {
            userService.findAll()
        }
    }

    // Required parameter with default value
    @GetMapping("/paginated")
    fun getUsersPaginated(
        @RequestParam(defaultValue = "0") page: Int,
        @RequestParam(defaultValue = "10") size: Int
    ): Page<User> {
        return userService.findAllPaginated(page, size)
    }
}
```

### Annotation Use-Site Targets

Kotlin allows specifying annotation use-site targets to clarify what the annotation applies to:

```kotlin`
class Example {

    // Annotate the property (not the getter)
    @get:Autowired
    val userService: UserService

    // Annotate the constructor parameter
    @field:NotBlank
    val username: String
}
```

## Bean Definition DSL

Spring Framework provides a functional bean definition DSL for Kotlin that allows you to register beans using a Kotlin-friendly API.

### Functional Bean Registration

The Kotlin Bean Definition DSL provides a functional way to register beans:

```kotlin
import org.springframework.context.support.beans

// Define beans using the DSL
val beans = beans {
    bean<UserRepository>()
    bean<UserService>()
    bean<UserController>()
    bean {
        RestTemplate()
    }
}

// Use with GenericApplicationContext
fun main() {
    val context = GenericApplicationContext().apply {
        beans.initialize(this)
        refresh()
    }

    val userService = context.getBean(UserService::class.java)
}
```

### Bean Configuration

The DSL supports bean configuration with lambdas:

```kotlin
beans {
    // Simple bean
    bean<UserRepository>()

    // Bean with configuration
    bean<UserService> {
        UserService(ref())
    }

    // Bean with properties
    bean<DataSource> {
        HikariDataSource().apply {
            jdbcUrl = "jdbc:mysql://localhost:3306/mydb"
            username = "user"
            password = "password"
        }
    }

    // Bean with profile
    bean("datasource", profile = arrayOf("production")) {
        HikariDataSource().apply {
            jdbcUrl = "jdbc:mysql://prod-db:3306/mydb"
            username = "prod-user"
            password = env["DB_PASSWORD"]
        }
    }
}
```

### Bean Dependencies

The DSL provides several ways to reference other beans:

```kotlin
beans {
    bean<UserRepository>()

    // Using ref() to reference another bean
    bean<UserService> {
        UserService(ref())
    }

    // Using by type
    bean<UserController> {
        UserController(UserService(ref()))
    }

    // Using by name
    bean("userController") {
        UserController(ref<UserService>())
    }
}
```

## Web

Spring Framework provides Kotlin-specific extensions and DSLs for web development.

### Router DSL

Spring WebFlux provides a functional routing DSL for Kotlin:

```kotlin
import org.springframework.web.servlet.function.router
import org.springframework.web.servlet.function.ServerResponse

@Configuration
class RouterConfig {

    @Bean
    fun routes(userHandler: UserHandler) = router {
        val repository = ref<UserRepository>()

        // GET /api/users
        GET("/api/users") {
            ServerResponse.ok().body(userService.findAll())
        }

        // GET /api/users/{id}
        GET("/api/users/{id}") {
            val id = it.pathVariable("id").toLong()
            val user = userService.findById(id)
            ServerResponse.ok().body(user)
        }

        // POST /api/users
        POST("/api/users") {
            val user = it.body(toMono<User>())
            val saved = userService.save(user)
            ServerResponse.ok().body(saved)
        }

        // Nested routes
        "/api/users".nest {
            GET("/") { /*...*/ }
            POST("/") { /*...*/ }
            "/{id}".nest {
                GET("/") { /*...*/ }
                PUT("/") { /*...*/ }
                DELETE("/") { /*...*/ }
            }
        }
    }
}
```

### MockMvc DSL

Spring MVC Test provides a Kotlin DSL for MockMvc:

```kotlin
import org.springframework.test.web.servlet.MockMvc
import org.springframework.test.web.servlet.setup.MockMvcBuilders
import org.springframework.test.web.servlet.get

@SpringBootTest
class UserControllerTest {

    @Autowired
    lateinit var mockMvc: MockMvc

    @Test
    fun `should get user by id`() {
        mockMvc.get("/api/users/1")
            .andExpect {
                status { isOk() }
                content { contentType(MediaType.APPLICATION_JSON) }
                jsonPath("$.name") { value("John Doe") }
            }
    }

    @Test
    fun `should create user`() {
        mockMvc.post("/api/users") {
            contentType = MediaType.APPLICATION_JSON
            content = """{"name":"John","email":"john@example.com"}"""
        }
        .andExpect {
            status { isCreated() }
            header { exists("Location") }
        }
    }
}
```

### Script Templates

Spring Framework supports Kotlin script templates for view rendering:

```kotlin
@Configuration
class WebConfig {

    @Bean
    fun kotlinScriptTemplateResolver(): ScriptTemplateConfigurer {
        return ScriptTemplateConfigurer().apply {
            engineName = "kotlin"
            scripts = arrayOf("scripts/render.kt")
            renderFunction = "render"
            isSharedEngine = false
        }
    }
}
```

### Kotlin Multiplatform Serialization

Spring Framework supports Kotlin multiplatform serialization:

```kotlin
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class User(
    val id: Long,
    val name: String,
    val email: String
)

@RestController
class UserController {

    private val json = Json { ignoreUnknownKeys = true }

    @GetMapping("/api/users")
    fun getUsers(): List<User> {
        return userService.findAll()
    }

    @PostMapping("/api/users")
    fun createUser(@RequestBody user: User): User {
        return userService.save(user)
    }
}
```

## Coroutines

Spring Framework provides comprehensive support for Kotlin coroutines in both Spring MVC and WebFlux.

### Suspend Functions

Spring MVC and WebFlux support Kotlin suspend functions, allowing you to write asynchronous code in a sequential style:

```kotlin
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.toList

@RestController
@RequestMapping("/api/users")
class UserController(private val userService: UserService) {

    // Suspend function in Spring MVC
    @GetMapping("/{id}")
    suspend fun getUser(@PathVariable id: Long): User {
        return delay(100) // Simulate async operation
        return userService.findById(id)
    }

    @GetMapping
    suspend fun getAllUsers(): List<User> {
        return userService.findAll()
    }
}

@Service
class UserService(private val userRepository: UserRepository) {

    suspend fun findById(id: Long): User = withContext(Dispatchers.IO) {
        userRepository.findById(id) ?: throw UserNotFoundException(id)
    }

    suspend fun findAll(): List<User> = withContext(Dispatchers.IO) {
        userRepository.findAll()
    }
}
```

### Flow

Spring Framework supports Kotlin Flow for streaming data:

```kotlin
import kotlinx.coroutines.flow.Flow

@RestController
@RequestMapping("/api/users")
class UserController(private val userService: UserService) {

    // Return Flow for streaming response (WebFlux)
    @GetMapping(produces = [MediaType.TEXT_EVENT_STREAM_VALUE])
    fun streamUsers(): Flow<User> {
        return userService.streamAllUsers()
    }

    @GetMapping("/export")
    suspend fun exportUsers(): Flow<User> {
        return userService.findAllAsFlow()
    }
}

@Service
class UserService(private val userRepository: UserRepository) {

    fun streamAllUsers(): Flow<User> = userRepository.streamAll()
}
```

### Deferred

Spring Framework supports Kotlin Deferred for async computation:

```kotlin
import kotlinx.coroutines.Deferred
import kotlinx.coroutines.async

@RestController
class UserController(
    private val userService: UserService,
    private val orderService: OrderService
) {

    @GetMapping("/users/{id}/details")
    suspend fun getUserDetails(@PathVariable id: Long): UserDetails {
        return coroutineScope {
            // Parallel execution
            val userDeferred: Deferred<User> = async { userService.findById(id) }
            val ordersDeferred: Deferred<List<Order>> = async { orderService.findByUserId(id) }

            // Wait for both results
            val user = userDeferred.await()
            val orders = ordersDeferred.await()

            UserDetails(user, orders)
        }
    }
}
```

### Coroutine Scope

Spring Framework provides coroutine scope management:

```kotlin
@Service
class UserService(
    private val userRepository: UserRepository,
    private val coroutineScope: CoroutineScope
) {

    @PostConstruct
    fun init() {
        // Launch background task
        coroutineScope.launch {
            while (isActive) {
                refreshCache()
                delay(60_000)
            }
        }
    }

    @PreDestroy
    fun cleanup() {
        coroutineScope.cancel()
    }
}
```

## Spring Projects in Kotlin

Various Spring projects provide Kotlin-specific features and support.

### Kotlin Spring Plugin

The `kotlin-spring` compiler plugin provides special support for Spring:

```groovy
// build.gradle.kts
plugins {
    kotlin("plugin.spring") version "1.9.0"
}
```

The plugin provides:

- **Open class by default**: Kotlin classes are `final` by default. The `kotlin-spring` plugin automatically makes classes annotated with Spring annotations `open` for proxying.

- **All-open classes**: Classes annotated with `@Configuration`, `@Controller`, `@Service`, `@Repository`, `@Component`, etc., are automatically made `open`.

### Final by Default

Kotlin classes are `final` by default, which means they cannot be extended. This can cause issues with Spring's proxy-based AOP. The `kotlin-spring` plugin addresses this by automatically opening Spring-annotated classes.

If you prefer not to use the plugin, you can manually mark classes as `open`:

```kotlin
// Manual approach - mark class as open
open class UserService {
    open fun findByUsername(username: String): User? {
        // Implementation
    }
}

// Using kotlin-spring plugin - class is automatically open
@Service
class UserService {
    fun findByUsername(username: String): User? {
        // Implementation
    }
}
```

### Immutable Classes for Persistence

Spring Data supports Kotlin immutable data classes for persistence:

```kotlin
import org.springframework.data.annotation.Id
import org.springframework.data.relational.core.mapping.Table

@Table("users")
data class User(
    @Id val id: Long? = null,
    val name: String,
    val email: String
)

interface UserRepository : R2dbcRepository<User, Long> {
    fun findByEmail(email: String): Flow<User>
}
```

### Dependency Injection

Spring Framework's dependency injection works seamlessly with Kotlin:

```kotlin
@Service
class UserService(
    private val userRepository: UserRepository,
    private val eventPublisher: ApplicationEventPublisher
) {
    // Spring automatically injects dependencies through constructor
}

@Component
class UserEventHandler(
    private val emailService: EmailService
) : ApplicationListener<UserCreatedEvent> {

    override fun onApplicationEvent(event: UserCreatedEvent) {
        emailService.sendWelcomeEmail(event.user)
    }
}
```

### Testing

Spring Test provides Kotlin-specific extensions for testing:

```kotlin
import org.junit.jupiter.api.Test
import org.springframework.boot.test.context.SpringBootTest
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.test.web.servlet.MockMvc
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc

@SpringBootTest
@AutoConfigureMockMvc
class UserControllerTest @Autowired constructor(
    private val mockMvc: MockMvc,
    private val userService: UserService
) {

    @Test
    fun `should return user when user exists`() {
        // Given
        val user = User(1L, "John", "john@example.com")
        every { userService.findById(1L) } returns user

        // When & Then
        mockMvc.get("/api/users/1")
            .andExpect {
                status { isOk() }
                jsonPath("$.name") { value("John") }
            }
    }

    @Test
    fun `should return 404 when user does not exist`() {
        every { userService.findById(1L) } returns null

        mockMvc.get("/api/users/1")
            .andExpect {
                status { isNotFound() }
            }
    }
}
```

Mockk integration with Spring Boot Test:

```kotlin
import io.mockk.every
import io.mockk.mockk
import org.junit.jupiter.api.Test
import org.springframework.boot.test.context.TestConfiguration
import org.springframework.context.annotation.Bean

@TestConfiguration
class TestConfig {

    @Bean
    fun testUserService() = mockk<UserService>()

    @Bean
    fun testEmailService() = mockk<EmailService>(relaxed = true)
}
```

## Getting Started

This section provides guidance on getting started with Spring and Kotlin.

### Start Spring Initializr

The easiest way to create a Spring Boot Kotlin project is to use [start.spring.io](https://start.spring.io/):

1. Navigate to [start.spring.io](https://start.spring.io/)
2. Choose Kotlin as the language
3. Select Spring Boot version
4. Add dependencies (e.g., Spring Web, Spring Data JPA)
5. Generate and download the project

### Web Stack Selection

When creating a new Spring Boot Kotlin project for web development, you need to choose between:

- **Spring MVC**: Traditional servlet-based web framework
  - Blocking I/O
  - Easier to integrate with existing servlet-based technologies
  - Simpler programming model

- **Spring WebFlux**: Reactive web framework
  - Non-blocking I/O
  - Better for high-concurrency scenarios
  - Supports Kotlin coroutines natively

```kotlin
// Spring MVC Example
@RestController
class UserController(private val userService: UserService) {

    @GetMapping("/users/{id}")
    fun getUser(@PathVariable id: Long): User {
        return userService.findById(id)
    }
}

// Spring WebFlux Example
@RestController
class UserController(private val userService: UserService) {

    @GetMapping("/users/{id}")
    suspend fun getUser(@PathVariable id: Long): User {
        return userService.findById(id)
    }

    @GetMapping("/users", produces = [TEXT_EVENT_STREAM_VALUE])
    fun streamUsers(): Flow<User> {
        return userService.streamAllUsers()
    }
}
```

### Recommended Dependencies

For a typical Spring Boot Kotlin application, include these dependencies:

```groovy
// build.gradle.kts
dependencies {
    // Spring Boot starters
    implementation("org.springframework.boot:spring-boot-starter-web")
    implementation("org.springframework.boot:spring-boot-starter-data-jpa")

    // Kotlin standard library
    implementation("org.jetbrains.kotlin:kotlin-reflect")
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin")

    // Development tools
    developmentOnly("org.springframework.boot:spring-boot-devtools")

    // Testing
    testImplementation("org.springframework.boot:spring-boot-starter-test")
    testImplementation("io.mockk:mockk:1.13.5")
}
```

### Project Structure

A typical Spring Boot Kotlin project structure:

```
src/
├── main/
│   ├── kotlin/
│   │   └── com/example/demo/
│   │       ├── DemoApplication.kt
│   │       ├── controller/
│   │       │   └── UserController.kt
│   │       ├── service/
│   │       │   └── UserService.kt
│   │       ├── repository/
│   │       │   └── UserRepository.kt
│   │       └── model/
│   │           └── User.kt
│   └── resources/
│       ├── application.properties
│       └── application-dev.properties
└── test/
    └── kotlin/
        └── com/example/demo/
            └── UserControllerTest.kt
```

### Sample Application

```kotlin
package com.example.demo

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.context.annotation.Bean
import org.springframework.web.servlet.config.annotation.CorsRegistry
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer

@SpringBootApplication
class DemoApplication {

    @Bean
    fun corsConfigurer(): WebMvcConfigurer {
        return object : WebMvcConfigurer {
            override fun addCorsMappings(registry: CorsRegistry) {
                registry.addMapping("/**").allowedOrigins("*")
            }
        }
    }
}

fun main(args: Array<String>) {
    runApplication<DemoApplication>(*args)
}
```

## Resources

For more information on using Spring with Kotlin:

- [Kotlin Language Reference](https://kotlinlang.org/docs/reference/)
- [Spring Boot Kotlin Documentation](https://spring.io/guides/tutorials/spring-boot-kotlin/)
- [Kotlin Slack - Spring Channel](https://kotlinlang.slack.com/messages/spring/)
