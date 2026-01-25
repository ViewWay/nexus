# WebClient

Spring WebFlux includes a reactive HTTP client called WebClient.

Spring WebFlux includes a client to perform HTTP requests. `WebClient` has a
functional, fluent API based on Reactor (see [Reactive Libraries](webflux-reactive-libraries.html))
which enables declarative composition of asynchronous logic without the need to deal with
threads or concurrency. It is fully non-blocking, supports streaming, and relies on
the same [codecs](webflux/reactive-spring.html#webflux-codecs) that are also used to encode and
decode request and response content on the server side.

`WebClient` needs an HTTP client library to perform requests. There is built-in
support for the following:

* 
[Reactor Netty](https://github.com/reactor/reactor-netty)

* 
[JDK HttpClient](https://docs.oracle.com/en/java/javase/17/docs/api/java.net.http/java/net/http/HttpClient.html)

* 
[Jetty Reactive HttpClient](https://github.com/jetty-project/jetty-reactive-httpclient)

* 
[Apache HttpComponents](https://hc.apache.org/index.html)

* 
Others can be plugged in via `ClientHttpConnector`.

Section Summary

 * 
 [Configuration](webflux-webclient/client-builder.html)

 * 
 [`retrieve()`](webflux-webclient/client-retrieve.html)

 * 
 [Exchange](webflux-webclient/client-exchange.html)

 * 
 [Request Body](webflux-webclient/client-body.html)

 * 
 [Filters](webflux-webclient/client-filter.html)

 * 
 [Attributes](webflux-webclient/client-attributes.html)

 * 
 [Context](webflux-webclient/client-context.html)

 * 
 [Synchronous Use](webflux-webclient/client-synchronous.html)

 * 
 [Testing](webflux-webclient/client-testing.html)