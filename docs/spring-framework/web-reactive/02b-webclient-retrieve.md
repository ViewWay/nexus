
The `retrieve()` method can be used to declare how to extract the response. For example:

* 
Java

* 
Kotlin

```java hljs
WebClient client = WebClient.create("https://example.org");

Mono> result = client.get()
 .uri("/persons/{id}", id).accept(MediaType.APPLICATION_JSON)
 .retrieve()
 .toEntity(Person.class);
```

```kotlin hljs
val client = WebClient.create("https://example.org")

val result = client.get()
 .uri("/persons/{id}", id).accept(MediaType.APPLICATION_JSON)
 .retrieve()
 .toEntity().awaitSingle()
```

Or to get only the body:

* 
Java

* 
Kotlin

```java hljs
WebClient client = WebClient.create("https://example.org");

Mono result = client.get()
 .uri("/persons/{id}", id).accept(MediaType.APPLICATION_JSON)
 .retrieve()
 .bodyToMono(Person.class);
```

```kotlin hljs
val client = WebClient.create("https://example.org")

val result = client.get()
 .uri("/persons/{id}", id).accept(MediaType.APPLICATION_JSON)
 .retrieve()
 .awaitBody()
```

To get a stream of decoded objects:

* 
Java

* 
Kotlin

```java hljs
Flux result = client.get()
 .uri("/quotes").accept(MediaType.TEXT_EVENT_STREAM)
 .retrieve()
 .bodyToFlux(Quote.class);
```

```kotlin hljs
val result = client.get()
 .uri("/quotes").accept(MediaType.TEXT_EVENT_STREAM)
 .retrieve()
 .bodyToFlow()
```

By default, 4xx or 5xx responses result in an `WebClientResponseException`, including
sub-classes for specific HTTP status codes. To customize the handling of error
responses, use `onStatus` handlers as follows:

* 
Java

* 
Kotlin

```java hljs
Mono result = client.get()
 .uri("/persons/{id}", id).accept(MediaType.APPLICATION_JSON)
 .retrieve()
 .onStatus(HttpStatusCode::is4xxClientError, response -> ...)
 .onStatus(HttpStatusCode::is5xxServerError, response -> ...)
 .bodyToMono(Person.class);
```

```kotlin hljs
val result = client.get()
 .uri("/persons/{id}", id).accept(MediaType.APPLICATION_JSON)
 .retrieve()
 .onStatus(HttpStatusCode::is4xxClientError) { ... }
 .onStatus(HttpStatusCode::is5xxServerError) { ... }
 .awaitBody()
```