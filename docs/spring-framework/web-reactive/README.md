# Spring Framework Reactive Web Documentation

This directory contains the Spring Framework Reactive Web documentation, converted from the official Spring Framework documentation.

## Overview

Spring WebFlux is the reactive-stack web framework in Spring Framework, added in version 5.0. It is fully non-blocking, supports Reactive Streams back pressure, and runs on servers such as Netty and Servlet containers.

## Documents

| File | Description |
|------|-------------|
| [01-webflux.md](01-webflux.md) | Spring WebFlux overview and main topics |
| [01a-webflux-overview.md](01a-webflux-overview.md) | Detailed overview of the reactive web framework |
| [01b-webflux-reactive-core.md](01b-webflux-reactive-core.md) | Reactive Core - HTTP/Reactive basics, codecs, path matching |
| [01c-webflux-functional.md](01c-webflux-functional.md) | Functional Endpoints - lightweight functional programming model |
| [02-webclient.md](02-webclient.md) | WebClient overview |
| [02a-webclient-configuration.md](02a-webclient-configuration.md) | WebClient configuration and builder |
| [02b-webclient-retrieve.md](02b-webclient-retrieve.md) | WebClient retrieve() method |
| [03-http-interface.md](03-http-interface.md) | HTTP Service Client |
| [04-rsocket.md](04-rsocket.md) | RSocket protocol support |
| [05-reactive-libraries.md](05-reactive-libraries.md) | Reactive Libraries support |

## Key Concepts

### Reactive Streams
- Spring WebFlux is built on Project Reactor
- Supports `Flux` (0..N) and `Mono` (0..1) types
- Back pressure support across network boundaries

### Server Support
- Netty (recommended)
- Servlet 3.1+ containers (Tomcat, Jetty)
- Undertow

### Client Support
- WebClient - reactive HTTP client
- HTTP Service Client - declarative HTTP interfaces
- RSocket - multiplexed, duplex communication

## Source

Original documentation: [Spring Framework Reference](https://docs.spring.io/spring-framework/reference/)
