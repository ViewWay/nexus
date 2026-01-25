# Spring WebFlux

The reactive-stack web framework in Spring Framework.

The original web framework included in the Spring Framework, Spring Web MVC, was
purpose-built for the Servlet API and Servlet containers. The reactive-stack web framework,
Spring WebFlux, was added later in version 5.0. It is fully non-blocking, supports
[Reactive Streams](https://www.reactive-streams.org/) back pressure, and runs on such servers as
Netty, and Servlet containers.

Both web frameworks mirror the names of their source modules
([spring-webmvc](https://github.com/spring-projects/spring-framework/tree/main/spring-webmvc) and
[spring-webflux](https://github.com/spring-projects/spring-framework/tree/main/spring-webflux)) and co-exist side by side in the
Spring Framework. Each module is optional. Applications can use one or the other module or,
in some cases, both — for example, Spring MVC controllers with the reactive `WebClient`.

Section Summary

 * 
 [Overview](webflux/new-framework.html)

 * 
 [Reactive Core](webflux/reactive-spring.html)

 * 
 [`DispatcherHandler`](webflux/dispatcher-handler.html)

 * 
 [Annotated Controllers](webflux/controller.html)

 * 
 [Functional Endpoints](webflux-functional.html)

 * 
 [URI Links](webflux/uri-building.html)

 * 
 [Range Requests](webflux/range.html)

 * 
 [CORS](webflux-cors.html)

 * 
 [API Versioning](webflux-versioning.html)

 * 
 [Error Responses](webflux/ann-rest-exceptions.html)

 * 
 [Web Security](webflux/security.html)

 * 
 [HTTP Caching](webflux/caching.html)

 * 
 [View Technologies](webflux-view.html)

 * 
 [WebFlux Config](webflux/config.html)

 * 
 [HTTP/2](webflux/http2.html)