
The `spring-web` module contains the following foundational support for reactive web
applications:

* 
For server request processing there are two levels of support.

[HttpHandler](#webflux-httphandler): Basic contract for HTTP request handling with
non-blocking I/O and Reactive Streams back pressure, along with adapters for Reactor Netty,
Tomcat, Jetty, and any Servlet container.

* 
[`WebHandler` API](#webflux-web-handler-api): Slightly higher level, general-purpose web API for
request handling, on top of which concrete programming models such as annotated
controllers and functional endpoints are built.

* 
For the client side, there is a basic `ClientHttpConnector` contract to perform HTTP
requests with non-blocking I/O and Reactive Streams back pressure, along with adapters for
[Reactor Netty](https://github.com/reactor/reactor-netty), reactive
[Jetty HttpClient](https://github.com/jetty-project/jetty-reactive-httpclient)
and [Apache HttpComponents](https://hc.apache.org/).
The higher level [WebClient](../webflux-webclient.html) used in applications
builds on this basic contract.

* 
For client and server, [codecs](#webflux-codecs) for serialization and
deserialization of HTTP request and response content.

## [](#webflux-httphandler)`HttpHandler`

[HttpHandler](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/http/server/reactive/HttpHandler.html)
is a simple contract with a single method to handle a request and a response. It is
intentionally minimal, and its main and only purpose is to be a minimal abstraction
over different HTTP server APIs.

The following table describes the supported server APIs:

Server name
Server API used
Reactive Streams support

Netty

Netty API

[Reactor Netty](https://github.com/reactor/reactor-netty)

Tomcat

Servlet non-blocking I/O; Tomcat API to read and write ByteBuffers vs byte[]

spring-web: Servlet non-blocking I/O to Reactive Streams bridge

Jetty

Servlet non-blocking I/O; Jetty API to write ByteBuffers vs byte[]

spring-web: Servlet non-blocking I/O to Reactive Streams bridge

Servlet container

Servlet non-blocking I/O

spring-web: Servlet non-blocking I/O to Reactive Streams bridge

The following table describes server dependencies (also see
[supported versions](https://github.com/spring-projects/spring-framework/wiki/What%27s-New-in-the-Spring-Framework)):

Server name
Group id
Artifact name

Reactor Netty

io.projectreactor.netty

reactor-netty

Tomcat

org.apache.tomcat.embed

tomcat-embed-core

Jetty

org.eclipse.jetty

jetty-server, jetty-servlet

The code snippets below show using the `HttpHandler` adapters with each server API.

**Reactor Netty**

* 
Java

* 
Kotlin

```java hljs
HttpHandler handler = ...
ReactorHttpHandlerAdapter adapter = new ReactorHttpHandlerAdapter(handler);
HttpServer.create().host(host).port(port).handle(adapter).bindNow();
```

```kotlin hljs
val handler: HttpHandler = ...
val adapter = ReactorHttpHandlerAdapter(handler)
HttpServer.create().host(host).port(port).handle(adapter).bindNow()
```

**Tomcat**

* 
Java

* 
Kotlin

```java hljs
HttpHandler handler = ...
Servlet servlet = new TomcatHttpHandlerAdapter(handler);

Tomcat server = new Tomcat();
File base = new File(System.getProperty("java.io.tmpdir"));
Context rootContext = server.addContext("", base.getAbsolutePath());
Tomcat.addServlet(rootContext, "main", servlet);
rootContext.addServletMappingDecoded("/", "main");
server.setHost(host);
server.setPort(port);
server.start();
```

```kotlin hljs
val handler: HttpHandler = ...
val servlet = TomcatHttpHandlerAdapter(handler)

val server = Tomcat()
val base = File(System.getProperty("java.io.tmpdir"))
val rootContext = server.addContext("", base.absolutePath)
Tomcat.addServlet(rootContext, "main", servlet)
rootContext.addServletMappingDecoded("/", "main")
server.host = host
server.setPort(port)
server.start()
```

**Jetty**

* 
Java

* 
Kotlin

```java hljs
HttpHandler handler = ...
JettyCoreHttpHandlerAdapter adapter = new JettyCoreHttpHandlerAdapter(handler);

Server server = new Server();
server.setHandler(adapter);

ServerConnector connector = new ServerConnector(server);
connector.setHost(host);
connector.setPort(port);
server.addConnector(connector);

server.start();
```

```kotlin hljs
val handler: HttpHandler = ...
val adapter = JettyCoreHttpHandlerAdapter(handler)

val server = Server()
server.setHandler(adapter)

val connector = ServerConnector(server)
connector.host = host
connector.port = port
server.addConnector(connector)

server.start()
```

**

In Spring Framework 6.2, `JettyHttpHandlerAdapter` was deprecated in favor of
`JettyCoreHttpHandlerAdapter`, which integrates directly with Jetty 12 APIs
without a Servlet layer.

To deploy as a WAR to a Servlet container instead, use
[`AbstractReactiveWebInitializer`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/web/server/adapter/AbstractReactiveWebInitializer.html),
to adapt `HttpHandler` to a `Servlet` via `ServletHttpHandlerAdapter`.

## [](#webflux-web-handler-api)`WebHandler` API

The `org.springframework.web.server` package builds on the
[`HttpHandler`](#webflux-httphandler) contract
to provide a general-purpose web API for processing requests through a chain of multiple
[`WebExceptionHandler`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/web/server/WebExceptionHandler.html), multiple
[`WebFilter`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/web/server/WebFilter.html), and a single
[`WebHandler`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/web/server/WebHandler.html) component. The chain can
be put together with `WebHttpHandlerBuilder` by simply pointing to a Spring
`ApplicationContext` where components are
[auto-detected](#webflux-web-handler-api-special-beans), and/or by registering components
with the builder.

While `HttpHandler` has a simple goal to abstract the use of different HTTP servers, the
`WebHandler` API aims to provide a broader set of features commonly used in web applications
such as:

* 
User session with attributes.

* 
Request attributes.

* 
Resolved `Locale` or `Principal` for the request.

* 
Access to parsed and cached form data.

* 
Abstractions for multipart data.

* 
and more..

### [](#webflux-web-handler-api-special-beans)Special bean types

The table below lists the components that `WebHttpHandlerBuilder` can auto-detect in a
Spring ApplicationContext, or that can be registered directly with it:

Bean name
Bean type
Count
Description

`WebExceptionHandler`

0..N

Provide handling for exceptions from the chain of `WebFilter` instances and the target
 `WebHandler`. For more details, see [Exceptions](#webflux-exception-handler).

`WebFilter`

0..N

Apply interception style logic to before and after the rest of the filter chain and
 the target `WebHandler`. For more details, see [Filters](#webflux-filters).

`webHandler`

`WebHandler`

1

The handler for the request.

`webSessionManager`

`WebSessionManager`

0..1

The manager for `WebSession` instances exposed through a method on `ServerWebExchange`.
 `DefaultWebSessionManager` by default.

`serverCodecConfigurer`

`ServerCodecConfigurer`

0..1

For access to `HttpMessageReader` instances for parsing form data and multipart data that is then
 exposed through methods on `ServerWebExchange`. `ServerCodecConfigurer.create()` by default.

`localeContextResolver`

`LocaleContextResolver`

0..1

The resolver for `LocaleContext` exposed through a method on `ServerWebExchange`.
 `AcceptHeaderLocaleContextResolver` by default.

`forwardedHeaderTransformer`

`ForwardedHeaderTransformer`

0..1

For processing forwarded type headers, either by extracting and removing them or by removing them only.
 Not used by default.

### [](#webflux-form-data)Form Data

`ServerWebExchange` exposes the following method for accessing form data:

* 
Java

* 
Kotlin

```java hljs
Mono> getFormData();
```

```Kotlin hljs
suspend fun getFormData(): MultiValueMap
```

The `DefaultServerWebExchange` uses the configured `HttpMessageReader` to parse form data
(`application/x-www-form-urlencoded`) into a `MultiValueMap`. By default,
`FormHttpMessageReader` is configured for use by the `ServerCodecConfigurer` bean
(see the [Web Handler API](#webflux-web-handler-api)).

### [](#webflux-multipart)Multipart Data

[See equivalent in the Servlet stack](../webmvc/mvc-servlet/multipart.html)

`ServerWebExchange` exposes the following method for accessing multipart data:

* 
Java

* 
Kotlin

```java hljs
Mono> getMultipartData();
```

```Kotlin hljs
suspend fun getMultipartData(): MultiValueMap
```

The `DefaultServerWebExchange` uses the configured
`HttpMessageReader>` to parse `multipart/form-data`,
`multipart/mixed`, and `multipart/related` content into a `MultiValueMap`.
By default, this is the `DefaultPartHttpMessageReader`, which does not have any third-party
dependencies.
Alternatively, the `SynchronossPartHttpMessageReader` can be used, which is based on the
[Synchronoss NIO Multipart](https://github.com/synchronoss/nio-multipart) library.
Both are configured through the `ServerCodecConfigurer` bean
(see the [Web Handler API](#webflux-web-handler-api)).

To parse multipart data in streaming fashion, you can use the `Flux` returned from the
`PartEventHttpMessageReader` instead of using `@RequestPart`, as that implies `Map`-like access
to individual parts by name and, hence, requires parsing multipart data in full.
By contrast, you can use `@RequestBody` to decode the content to `Flux` without
collecting to a `MultiValueMap`.

### [](#webflux-forwarded-headers)Forwarded Headers

[See equivalent in the Servlet stack](../webmvc/filters.html#filters-forwarded-headers)

As a request goes through proxies such as load balancers the host, port, and
scheme may change, and that makes it a challenge to create links that point to the correct
host, port, and scheme from a client perspective.

[RFC 7239](https://datatracker.ietf.org/doc/html/rfc7239) defines the `Forwarded` HTTP header
that proxies can use to provide information about the original request.

### [](#forwarded-headers-non-standard)Non-standard Headers

There are other non-standard headers, too, including `X-Forwarded-Host`, `X-Forwarded-Port`,
`X-Forwarded-Proto`, `X-Forwarded-Ssl`, `X-Forwarded-Prefix`, and `X-Forwarded-For`.

#### [](#x-forwarded-host)X-Forwarded-Host

While not standard, [`X-Forwarded-Host: `](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-Host)
is a de-facto standard header that is used to communicate the original host to a
downstream server. For example, if a request of `[example.com/resource](https://example.com/resource)` is sent to
a proxy which forwards the request to `[localhost:8080/resource](http://localhost:8080/resource)`, then a header of
`X-Forwarded-Host: example.com` can be sent to inform the server that the original host was `example.com`.

#### [](#x-forwarded-port)X-Forwarded-Port

While not standard, `X-Forwarded-Port: ` is a de-facto standard header that is used to
communicate the original port to a downstream server. For example, if a request of
`[example.com/resource](https://example.com/resource)` is sent to a proxy which forwards the request to
`[localhost:8080/resource](http://localhost:8080/resource)`, then a header of `X-Forwarded-Port: 443` can be sent
to inform the server that the original port was `443`.

#### [](#x-forwarded-proto)X-Forwarded-Proto

While not standard, [`X-Forwarded-Proto: (https|http)`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-Proto)
is a de-facto standard header that is used to communicate the original protocol (for example, https / http)
to a downstream server. For example, if a request of `[example.com/resource](https://example.com/resource)` is sent to
a proxy which forwards the request to `[localhost:8080/resource](http://localhost:8080/resource)`, then a header of
`X-Forwarded-Proto: https` can be sent to inform the server that the original protocol was `https`.

#### [](#x-forwarded-ssl)X-Forwarded-Ssl

While not standard, `X-Forwarded-Ssl: (on|off)` is a de-facto standard header that is used to communicate the
original protocol (for example, https / https) to a downstream server. For example, if a request of
`[example.com/resource](https://example.com/resource)` is sent to a proxy which forwards the request to
`[localhost:8080/resource](http://localhost:8080/resource)`, then a header of `X-Forwarded-Ssl: on` to inform the server that the
original protocol was `https`.

#### [](#x-forwarded-prefix)X-Forwarded-Prefix

While not standard, [`X-Forwarded-Prefix: `](https://microsoft.github.io/reverse-proxy/articles/transforms.html#defaults)
is a de-facto standard header that is used to communicate the original URL path prefix to a
downstream server.

Use of `X-Forwarded-Prefix` can vary by deployment scenario, and needs to be flexible to
allow replacing, removing, or prepending the path prefix of the target server.

*Scenario 1: Override path prefix*

```
https://example.com/api/{path} -> http://localhost:8080/app1/{path}
```

The prefix is the start of the path before the capture group `{path}`. For the proxy,
the prefix is `/api` while for the server the prefix is `/app1`. In this case, the proxy
can send `X-Forwarded-Prefix: /api` to have the original prefix `/api` override the
server prefix `/app1`.

*Scenario 2: Remove path prefix*

At times, an application may want to have the prefix removed. For example, consider the
following proxy to server mapping:

```
https://app1.example.com/{path} -> http://localhost:8080/app1/{path}
https://app2.example.com/{path} -> http://localhost:8080/app2/{path}
```

The proxy has no prefix, while applications `app1` and `app2` have path prefixes
`/app1` and `/app2` respectively. The proxy can send `X-Forwarded-Prefix: ` to
have the empty prefix override server prefixes `/app1` and `/app2`.

**

A common case for this deployment scenario is where licenses are paid per
production application server, and it is preferable to deploy multiple applications per
server to reduce fees. Another reason is to run more applications on the same server in
order to share the resources required by the server to run.

In these scenarios, applications need a non-empty context root because there are multiple
applications on the same server. However, this should not be visible in URL paths of
the public API where applications may use different subdomains that provides benefits
such as:

* 
Added security, for example, same origin policy

* 
Independent scaling of applications (different domain points to different IP address)

*Scenario 3: Insert path prefix*

In other cases, it may be necessary to prepend a prefix. For example, consider the
following proxy to server mapping:

```
https://example.com/api/app1/{path} -> http://localhost:8080/app1/{path}
```

In this case, the proxy has a prefix of `/api/app1` and the server has a prefix of
`/app1`. The proxy can send `X-Forwarded-Prefix: /api/app1` to have the original prefix
`/api/app1` override the server prefix `/app1`.

#### [](#x-forwarded-for)X-Forwarded-For

[`X-Forwarded-For: `](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/X-Forwarded-For)
is a de-facto standard header that is used to communicate the original `InetSocketAddress` of the client to a
downstream server. For example, if a request is sent by a client at `[fd00:fefe:1::4]` to a proxy at
`192.168.0.1`, the "remote address" information contained in the HTTP request will reflect the actual address of the
client, not the proxy.

### [](#webflux-forwarded-headers-transformer)ForwardedHeaderTransformer

`ForwardedHeaderTransformer` is a component that modifies the host, port, and scheme of
the request, based on forwarded headers, and then removes those headers. If you declare
it as a bean with the name `forwardedHeaderTransformer`, it will be
[detected](#webflux-web-handler-api-special-beans) and used.

### [](#webflux-forwarded-headers-security)Security Considerations

There are security considerations for forwarded headers since an application cannot know
if the headers were added by a proxy, as intended, or by a malicious client. This is why
a proxy at the boundary of trust should be configured to remove untrusted forwarded traffic coming
from the outside. You can also configure the `ForwardedHeaderTransformer` with
`removeOnly=true`, in which case it removes but does not use the headers.

## [](#webflux-filters)Filters

[See equivalent in the Servlet stack](../webmvc/filters.html)

In the [`WebHandler` API](#webflux-web-handler-api), you can use a `WebFilter` to apply interception-style
logic before and after the rest of the processing chain of filters and the target
`WebHandler`. When using the [WebFlux Config](dispatcher-handler.html#webflux-framework-config), registering a `WebFilter` is as simple
as declaring it as a Spring bean and (optionally) expressing precedence by using `@Order` on
the bean declaration or by implementing `Ordered`.

### [](#webflux-filters-cors)CORS

[See equivalent in the Servlet stack](../webmvc/filters.html#filters-cors)

Spring WebFlux provides fine-grained support for CORS configuration through annotations on
controllers. However, when you use it with Spring Security, we advise relying on the built-in
`CorsFilter`, which must be ordered ahead of Spring Security’s chain of filters.

See the section on [CORS](../webflux-cors.html) and the [CORS `WebFilter`](../webflux-cors.html#webflux-cors-webfilter) for more details.

### [](#filters.url-handler)URL Handler

[See equivalent in the Servlet stack](../webmvc/filters.html#filters.url-handler)

You may want your controller endpoints to match routes with or without a trailing slash in the URL path.
For example, both "GET /home" and "GET /home/" should be handled by a controller method annotated with `@GetMapping("/home")`.

Adding trailing slash variants to all mapping declarations is not the best way to handle this use case.
The `UrlHandlerFilter` web filter has been designed for this purpose. It can be configured to:

* 
respond with an HTTP redirect status when receiving URLs with trailing slashes, sending browsers to the non-trailing slash URL variant.

* 
mutate the request to act as if the request was sent without a trailing slash and continue the processing of the request.

Here is how you can instantiate and configure a `UrlHandlerFilter` for a blog application:

* 
Java

* 
Kotlin

```java hljs
UrlHandlerFilter urlHandlerFilter = UrlHandlerFilter
 // will HTTP 308 redirect "/blog/my-blog-post/" -> "/blog/my-blog-post"
 .trailingSlashHandler("/blog/**").redirect(HttpStatus.PERMANENT_REDIRECT)
 // will mutate the request to "/admin/user/account/" and make it as "/admin/user/account"
 .trailingSlashHandler("/admin/**").mutateRequest()
 .build();
```

```kotlin hljs
val urlHandlerFilter = UrlHandlerFilter
 // will HTTP 308 redirect "/blog/my-blog-post/" -> "/blog/my-blog-post"
 .trailingSlashHandler("/blog/**").redirect(HttpStatus.PERMANENT_REDIRECT)
 // will mutate the request to "/admin/user/account/" and make it as "/admin/user/account"
 .trailingSlashHandler("/admin/**").mutateRequest()
 .build()
```

## [](#webflux-exception-handler)Exceptions

[See equivalent in the Servlet stack](../webmvc/mvc-servlet/exceptionhandlers.html#mvc-ann-customer-servlet-container-error-page)

In the [`WebHandler` API](#webflux-web-handler-api), you can use a `WebExceptionHandler` to handle
exceptions from the chain of `WebFilter` instances and the target `WebHandler`. When using the
[WebFlux Config](dispatcher-handler.html#webflux-framework-config), registering a `WebExceptionHandler` is as simple as declaring it as a
Spring bean and (optionally) expressing precedence by using `@Order` on the bean declaration or
by implementing `Ordered`.

The following table describes the available `WebExceptionHandler` implementations:

Exception Handler
Description

`ResponseStatusExceptionHandler`

Provides handling for exceptions of type
 [`ResponseStatusException`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/web/server/ResponseStatusException.html)
 by setting the response to the HTTP status code of the exception.

`WebFluxResponseStatusExceptionHandler`

Extension of `ResponseStatusExceptionHandler` that can also determine the HTTP status
 code of a `@ResponseStatus` annotation on any exception.

 This handler is declared in the [WebFlux Config](dispatcher-handler.html#webflux-framework-config).

## [](#webflux-codecs)Codecs

[See equivalent in the Servlet stack](../webmvc/message-converters.html#message-converters)

The `spring-web` and `spring-core` modules provide support for serializing and
deserializing byte content to and from higher level objects through non-blocking I/O with
Reactive Streams back pressure. The following describes this support:

* 
[`Encoder`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/core/codec/Encoder.html) and
[`Decoder`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/core/codec/Decoder.html) are low level contracts to
encode and decode content independent of HTTP.

* 
[`HttpMessageReader`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/http/codec/HttpMessageReader.html) and
[`HttpMessageWriter`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/http/codec/HttpMessageWriter.html) are contracts
to encode and decode HTTP message content.

* 
An `Encoder` can be wrapped with `EncoderHttpMessageWriter` to adapt it for use in a web
application, while a `Decoder` can be wrapped with `DecoderHttpMessageReader`.

* 
[`DataBuffer`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/core/io/buffer/DataBuffer.html) abstracts different
byte buffer representations (for example, Netty `ByteBuf`, `java.nio.ByteBuffer`, etc.) and is
what all codecs work on. See [Data Buffers and Codecs](../../core/databuffer-codec.html) in the
"Spring Core" section for more on this topic.

The `spring-core` module provides `byte[]`, `ByteBuffer`, `DataBuffer`, `Resource`, and
`String` encoder and decoder implementations. The `spring-web` module provides Jackson
JSON, Jackson Smile, JAXB2, Protocol Buffers and other encoders and decoders along with
web-only HTTP message reader and writer implementations for form data, multipart content,
server-sent events, and others.

`ClientCodecConfigurer` and `ServerCodecConfigurer` are typically used to configure and
customize the codecs to use in an application. See the section on configuring
[HTTP message codecs](config.html#webflux-config-message-codecs).

### [](#webflux-codecs-jackson)Jackson JSON

JSON and binary JSON ([Smile](https://github.com/FasterXML/smile-format-specification)) are
both supported when the Jackson library is present.

The `JacksonJsonDecoder` works as follows:

* 
Jackson’s asynchronous, non-blocking parser is used to aggregate a stream of byte chunks
into `TokenBuffer`'s each representing a JSON object.

* 
Each `TokenBuffer` is passed to Jackson’s `JsonMapper` to create a higher level object.

* 
When decoding to a single-value publisher (for example, `Mono`), there is one `TokenBuffer`.

* 
When decoding to a multi-value publisher (for example, `Flux`), each `TokenBuffer` is passed to
the `JsonMapper` as soon as enough bytes are received for a fully formed object. The
input content can be a JSON array, or any
[line-delimited JSON](https://en.wikipedia.org/wiki/JSON_streaming) format such as NDJSON,
JSON Lines, or JSON Text Sequences.

The `JacksonJsonEncoder` works as follows:

* 
For a single value publisher (for example, `Mono`), simply serialize it through the
`JsonMapper`.

* 
For a multi-value publisher with `application/json`, by default collect the values with
`Flux#collectToList()` and then serialize the resulting collection.

* 
For a multi-value publisher with a streaming media type such as
`application/x-ndjson` or `application/stream+x-jackson-smile`, encode, write, and
flush each value individually using a
[line-delimited JSON](https://en.wikipedia.org/wiki/JSON_streaming) format. Other
streaming media types may be registered with the encoder.

* 
For SSE the `JacksonJsonEncoder` is invoked per event and the output is flushed to ensure
delivery without delay.

**

By default both `JacksonJsonEncoder` and `JacksonJsonDecoder` do not support elements of type
`String`. Instead the default assumption is that a string or a sequence of strings
represent serialized JSON content, to be rendered by the `CharSequenceEncoder`. If what
you need is to render a JSON array from `Flux`, use `Flux#collectToList()` and
encode a `Mono>`.

### [](#webflux-codecs-forms)Form Data

`FormHttpMessageReader` and `FormHttpMessageWriter` support decoding and encoding
`application/x-www-form-urlencoded` content.

On the server side where form content often needs to be accessed from multiple places,
`ServerWebExchange` provides a dedicated `getFormData()` method that parses the content
through `FormHttpMessageReader` and then caches the result for repeated access.
See [Form Data](#webflux-form-data) in the
[`WebHandler` API](#webflux-web-handler-api) section.

Once `getFormData()` is used, the original raw content can no longer be read from the
request body. For this reason, applications are expected to go through `ServerWebExchange`
consistently for access to the cached form data versus reading from the raw request body.

### [](#webflux-codecs-multipart)Multipart

`MultipartHttpMessageReader` and `MultipartHttpMessageWriter` support decoding and
encoding "multipart/form-data", "multipart/mixed", and "multipart/related" content.
In turn `MultipartHttpMessageReader` delegates to another `HttpMessageReader`
for the actual parsing to a `Flux` and then simply collects the parts into a `MultiValueMap`.
By default, the `DefaultPartHttpMessageReader` is used, but this can be changed through the
`ServerCodecConfigurer`.
For more information about the `DefaultPartHttpMessageReader`, refer to the
[javadoc of `DefaultPartHttpMessageReader`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/http/codec/multipart/DefaultPartHttpMessageReader.html).

On the server side where multipart form content may need to be accessed from multiple
places, `ServerWebExchange` provides a dedicated `getMultipartData()` method that parses
the content through `MultipartHttpMessageReader` and then caches the result for repeated access.
See [Multipart Data](#webflux-multipart) in the
[`WebHandler` API](#webflux-web-handler-api) section.

Once `getMultipartData()` is used, the original raw content can no longer be read from the
request body. For this reason applications have to consistently use `getMultipartData()`
for repeated, map-like access to parts, or otherwise rely on the
`SynchronossPartHttpMessageReader` for a one-time access to `Flux`.

### [](#webflux-codecs-protobuf)Protocol Buffers

`ProtobufEncoder` and `ProtobufDecoder` supporting decoding and encoding "application/x-protobuf", "application/octet-stream"
and "application/vnd.google.protobuf" content for `com.google.protobuf.Message` types. They also support stream of values
if content is received/sent with the "delimited" parameter along the content type (like "application/x-protobuf;delimited=true").
This requires the "com.google.protobuf:protobuf-java" library, version 3.29 and higher.

The `ProtobufJsonDecoder` and `ProtobufJsonEncoder` variants support reading and writing JSON documents to and from Protobuf messages.
They require the "com.google.protobuf:protobuf-java-util" dependency. Note, the JSON variants do not support reading stream of messages,
see the [javadoc of `ProtobufJsonDecoder`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/http/codec/protobuf/ProtobufJsonDecoder.html) for more details.

### [](#webflux-codecs-gson)Google Gson

Applications can use the `GsonEncoder` and `GsonDecoder` to serialize and deserialize JSON documents thanks to the [Google Gson](https://google.github.io/gson/) library .
This codec supports both JSON media types and the NDJSON format for streaming.

**

`Gson` does not support non-blocking parsing, so the `GsonDecoder` does not support deserializing
to `Flux` types. For example, if this decoder is used for deserializing a JSON stream or even a list of elements
as a `Flux`, an `UnsupportedOperationException` will be thrown at runtime.
Applications should instead focus on deserializing bounded collections and use `Mono>` as target types.

### [](#webflux-codecs-limits)Limits

`Decoder` and `HttpMessageReader` implementations that buffer some or all of the input
stream can be configured with a limit on the maximum number of bytes to buffer in memory.
In some cases buffering occurs because input is aggregated and represented as a single
object — for example, a controller method with `@RequestBody byte[]`,
`x-www-form-urlencoded` data, and so on. Buffering can also occur with streaming, when
splitting the input stream — for example, delimited text, a stream of JSON objects, and
so on. For those streaming cases, the limit applies to the number of bytes associated
with one object in the stream.

To configure buffer sizes, you can check if a given `Decoder` or `HttpMessageReader`
exposes a `maxInMemorySize` property and if so the Javadoc will have details about default
values. On the server side, `ServerCodecConfigurer` provides a single place from where to
set all codecs, see [HTTP message codecs](config.html#webflux-config-message-codecs). On the client side, the limit for
all codecs can be changed in
[WebClient.Builder](../webflux-webclient/client-builder.html#webflux-client-builder-maxinmemorysize).

For [Multipart parsing](#webflux-codecs-multipart) the `maxInMemorySize` property limits
the size of non-file parts. For file parts, it determines the threshold at which the part
is written to disk. For file parts written to disk, there is an additional
`maxDiskUsagePerPart` property to limit the amount of disk space per part. There is also
a `maxParts` property to limit the overall number of parts in a multipart request.
To configure all three in WebFlux, you’ll need to supply a pre-configured instance of
`MultipartHttpMessageReader` to `ServerCodecConfigurer`.

### [](#webflux-codecs-streaming)Streaming

[See equivalent in the Servlet stack](../webmvc/mvc-ann-async.html#mvc-ann-async-http-streaming)

When streaming to the HTTP response (for example, `text/event-stream`,
`application/x-ndjson`), it is important to send data periodically, in order to
reliably detect a disconnected client sooner rather than later. Such a send could be a
comment-only, empty SSE event or any other "no-op" data that would effectively serve as
a heartbeat.

### [](#webflux-codecs-buffers)`DataBuffer`

`DataBuffer` is the representation for a byte buffer in WebFlux. The Spring Core part of
this reference has more on that in the section on
[Data Buffers and Codecs](../../core/databuffer-codec.html). The key point to understand is that on some
servers like Netty, byte buffers are pooled and reference counted, and must be released
when consumed to avoid memory leaks.

WebFlux applications generally do not need to be concerned with such issues, unless they
consume or produce data buffers directly, as opposed to relying on codecs to convert to
and from higher level objects, or unless they choose to create custom codecs. For such
cases please review the information in [Data Buffers and Codecs](../../core/databuffer-codec.html),
especially the section on [Using DataBuffer](../../core/databuffer-codec.html#databuffers-using).

## [](#webflux-logging)Logging

[See equivalent in the Servlet stack](../webmvc/mvc-servlet/logging.html)

`DEBUG` level logging in Spring WebFlux is designed to be compact, minimal, and
human-friendly. It focuses on high value bits of information that are useful over and
over again vs others that are useful only when debugging a specific issue.

`TRACE` level logging generally follows the same principles as `DEBUG` (and for example also
should not be a firehose) but can be used for debugging any issue. In addition, some log
messages may show a different level of detail at `TRACE` vs `DEBUG`.

Good logging comes from the experience of using the logs. If you spot anything that does
not meet the stated goals, please let us know.

### [](#webflux-logging-id)Log Id

In WebFlux, a single request can be run over multiple threads and the thread ID
is not useful for correlating log messages that belong to a specific request. This is why
WebFlux log messages are prefixed with a request-specific ID by default.

On the server side, the log ID is stored in the `ServerWebExchange` attribute
([`LOG_ID_ATTRIBUTE`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/web/server/ServerWebExchange.html#LOG_ID_ATTRIBUTE)),
while a fully formatted prefix based on that ID is available from
`ServerWebExchange#getLogPrefix()`. On the `WebClient` side, the log ID is stored in the
`ClientRequest` attribute
([`LOG_ID_ATTRIBUTE`](https://docs.spring.io/spring-framework/docs/7.0.3/javadoc-api/org/springframework/web/reactive/function/client/ClientRequest.html#LOG_ID_ATTRIBUTE))
,while a fully formatted prefix is available from `ClientRequest#logPrefix()`.

### [](#webflux-logging-sensitive-data)Sensitive Data

[See equivalent in the Servlet stack](../webmvc/mvc-servlet/logging.html#mvc-logging-sensitive-data)

`DEBUG` and `TRACE` logging can log sensitive information. This is why form parameters and
headers are masked by default and you must explicitly enable their logging in full.

The following example shows how to do so for server-side requests:

* 
Java

* 
Kotlin

```java hljs
@Configuration
class MyConfig implements WebFluxConfigurer {

 @Override
 public void configureHttpMessageCodecs(ServerCodecConfigurer configurer) {
 configurer.defaultCodecs().enableLoggingRequestDetails(true);
 }
}
```

```kotlin hljs
@Configuration
class MyConfig : WebFluxConfigurer {

 override fun configureHttpMessageCodecs(configurer: ServerCodecConfigurer) {
 configurer.defaultCodecs().enableLoggingRequestDetails(true)
 }
}
```

The following example shows how to do so for client-side requests:

* 
Java

* 
Kotlin

```java hljs
Consumer consumer = configurer ->
 configurer.defaultCodecs().enableLoggingRequestDetails(true);

WebClient webClient = WebClient.builder()
 .exchangeStrategies(strategies -> strategies.codecs(consumer))
 .build();
```

```kotlin hljs
val consumer: (ClientCodecConfigurer) -> Unit = { configurer -> configurer.defaultCodecs().enableLoggingRequestDetails(true) }

val webClient = WebClient.builder()
 .exchangeStrategies({ strategies -> strategies.codecs(consumer) })
 .build()
```

### [](#webflux-logging-appenders)Appenders

Logging libraries such as SLF4J and Log4J 2 provide asynchronous loggers that avoid
blocking. While those have their own drawbacks such as potentially dropping messages
that could not be queued for logging, they are the best available options currently
for use in a reactive, non-blocking application.

### [](#webflux-codecs-custom)Custom codecs

Applications can register custom codecs for supporting additional media types,
or specific behaviors that are not supported by the default codecs.

Some configuration options expressed by developers are enforced on default codecs.
Custom codecs might want to get a chance to align with those preferences,
like [enforcing buffering limits](#webflux-codecs-limits)
or [logging sensitive data](#webflux-logging-sensitive-data).

The following example shows how to do so for client-side requests:

* 
Java

* 
Kotlin

```java hljs
WebClient webClient = WebClient.builder()
 .codecs(configurer -> {
 CustomDecoder decoder = new CustomDecoder();
 configurer.customCodecs().registerWithDefaultConfig(decoder);
 })
 .build();
```

```kotlin hljs
val webClient = WebClient.builder()
 .codecs({ configurer ->
 val decoder = CustomDecoder()
 configurer.customCodecs().registerWithDefaultConfig(decoder)
 })
 .build()
```