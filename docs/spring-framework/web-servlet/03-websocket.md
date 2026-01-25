# WebSockets (WebSocket 网络套接字)

本文档涵盖 Servlet 栈上的 WebSocket 消息支持，包括：
- 原始 WebSocket 交互
- 通过 SockJS 进行 WebSocket 模拟
- 通过 STOMP 作为 WebSocket 子协议的发布-订阅消息传递

---

## WebSocket 简介

### 什么是 WebSocket

WebSocket 协议（[RFC 6455](https://datatracker.ietf.org/doc/html/rfc6455)）提供了一种标准化的方式，在单个 TCP 连接上建立客户端和服务器之间的全双工、双向通信通道。

- WebSocket 是与 HTTP 不同的 TCP 协议
- 设计为在 HTTP 上工作，使用端口 80 和 443
- 允许重用现有的防火墙规则

### WebSocket 握手

WebSocket 交互以使用 HTTP `Upgrade` 头的 HTTP 请求开始，用于升级或切换到 WebSocket 协议。

**客户端请求示例：**

```http
GET /spring-websocket-portfolio/portfolio HTTP/1.1
Host: localhost:8080
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Key: Uc9l9TMkWGbHFD2qnFHltg==
Sec-WebSocket-Protocol: v10.stomp, v11.stomp
Sec-WebSocket-Version: 13
Origin: http://localhost:8080
```

**服务器响应示例：**

```http
HTTP/1.1 101 Switching Protocols
Upgrade: websocket
Connection: Upgrade
Sec-WebSocket-Accept: 1qVdfYHU9hPOl4JYYNXF623Gzn0=
Sec-WebSocket-Protocol: v10.stomp
```

成功握手后，HTTP 升级请求下的 TCP 套接字保持打开状态，供客户端和服务器继续发送和接收消息。

### HTTP 与 WebSocket 的区别

| 特性 | HTTP/REST | WebSocket |
|------|-----------|-----------|
| 架构 | 请求-响应模式 | 异步、事件驱动的消息传递 |
| URL | 多个 URL 表示不同资源 | 通常只有一个连接 URL |
| 通信方向 | 客户端发起请求 | 双向、全双工通信 |
| 语义 | 内置语义（方法、状态码） | 低级传输协议，无内置语义 |
| 消息路由 | 基于 URL、方法和头部 | 需要协商消息语义 |

在 HTTP 和 REST 中：
- 应用程序被建模为许多 URL
- 客户端访问这些 URL 进行交互，请求-响应样式
- 服务器根据 HTTP URL、方法和头部将请求路由到适当的处理程序

在 WebSocket 中：
- 通常只有一个 URL 用于初始连接
- 随后，所有应用程序消息在同一 TCP 连接上流动
- 指向完全不同的异步、事件驱动、消息传递架构

### 何时使用 WebSocket

**适合使用 WebSocket 的场景：**
- 低延迟 + 高频率 + 高消息量的组合
- 协作应用（如实时文档编辑）
- 游戏应用
- 金融应用（需要接近实时的更新）

**可能不需要 WebSocket 的场景：**
- 新闻、邮件和社交动态更新（每几分钟更新一次可能就足够了）
- 消息量相对较低的情况（如监控网络故障）
- HTTP 流或轮询可以提供有效的解决方案

> **注意**：在互联网上，您无法控制的限制性代理可能会阻止 WebSocket 交互，因为它们可能未配置为传递 `Upgrade` 头，或者它们关闭看起来处于空闲状态的长连接。

### Web 服务器配置

如果 WebSocket 服务器运行在 Web 服务器（例如 nginx）后面，您可能需要配置它将 WebSocket 升级请求传递到 WebSocket 服务器。

同样，如果应用程序在云环境中运行，请查看云提供商关于 WebSocket 支持的相关说明。

---

## Spring WebSocket 支持

Spring Framework 提供了以下 WebSocket 支持：

1. **WebSocket API** - 原始 WebSocket 交互
2. **SockJS Fallback** - WebSocket 模拟（当浏览器不支持 WebSocket 时）
3. **STOMP** - 作为 WebSocket 子协议的发布-订阅消息传递

### 章节内容

- [WebSocket API](https://docs.spring.io/spring-framework/reference/web/websocket/server.html) - 服务器端和客户端 WebSocket API
- [SockJS Fallback](https://docs.spring.io/spring-framework/reference/web/websocket/fallback.html) - 浏览器兼容性回退选项
- [STOMP](https://docs.spring.io/spring-framework/reference/web/websocket/stomp.html) - STOMP 协议支持

---

## WebSocket 配置

### 启用 WebSocket 支持

在 Spring MVC 中启用 WebSocket 支持：

```java
@Configuration
@EnableWebSocket
public class WebSocketConfig implements WebSocketConfigurer {

    @Override
    public void registerWebSocketHandlers(WebSocketHandlerRegistry registry) {
        registry.addHandler(myHandler(), "/myHandler")
                .setAllowedOrigins("*");
    }

    @Bean
    public WebSocketHandler myHandler() {
        return new MyWebSocketHandler();
    }
}
```

### WebSocket 处理器

```java
public class MyWebSocketHandler extends TextWebSocketHandler {

    @Override
    protected void handleTextMessage(WebSocketSession session, TextMessage message) {
        // 处理接收到的消息
        String payload = message.getPayload();
        // ...
        session.sendMessage(new TextMessage("Echo: " + payload));
    }

    @Override
    public void afterConnectionEstablished(WebSocketSession session) {
        // 连接建立后的处理
    }

    @Override
    public void afterConnectionClosed(WebSocketSession session, CloseStatus status) {
        // 连接关闭后的处理
    }
}
```

### SockJS 支持

当浏览器不支持 WebSocket 时，可以使用 SockJS 作为回退：

```java
@Configuration
@EnableWebSocket
public class WebSocketConfig implements WebSocketConfigurer {

    @Override
    public void registerWebSocketHandlers(WebSocketHandlerRegistry registry) {
        registry.addHandler(myHandler(), "/myHandler")
                .setAllowedOrigins("*")
                .withSockJS();  // 启用 SockJS 回退
    }

    @Bean
    public WebSocketHandler myHandler() {
        return new MyWebSocketHandler();
    }
}
```

---

## STOMP 协议

### 什么是 STOMP

STOMP（Simple Text Oriented Messaging Protocol）是一种简单的文本定向消息传递协议，提供了定义良好的消息语义。

### 启用 STOMP

```java
@Configuration
@EnableWebSocketMessageBroker
public class WebSocketConfig implements WebSocketMessageBrokerConfigurer {

    @Override
    public void configureMessageBroker(MessageBrokerRegistry config) {
        // 启用简单消息代理，用于向客户端发送消息
        config.enableSimpleBroker("/topic", "/queue");

        // 应用程序目的地前缀（用于客户端向服务器发送消息）
        config.setApplicationDestinationPrefixes("/app");
    }

    @Override
    public void registerStompEndpoints(StompEndpointRegistry registry) {
        // 注册 STOMP 端点
        registry.addEndpoint("/ws")
                .setAllowedOriginPatterns("*")
                .withSockJS();  // 启用 SockJS 回退
    }
}
```

### 消息处理控制器

```java
@Controller
public class ChatController {

    @MessageMapping("/chat.sendMessage")
    @SendTo("/topic/public")
    public ChatMessage sendMessage(@Payload ChatMessage chatMessage) {
        return chatMessage;
    }

    @MessageMapping("/chat.addUser")
    @SendTo("/topic/public")
    public ChatMessage addUser(@Payload ChatMessage chatMessage,
                               SimpMessageHeaderAccessor headerAccessor) {
        headerAccessor.getSessionAttributes().put("username", chatMessage.getSender());
        return chatMessage;
    }
}
```

### 客户端连接示例（JavaScript）

```javascript
const socket = new SockJS('/ws');
const stompClient = Stomp.over(socket);

stompClient.connect({}, function(frame) {
    console.log('Connected: ' + frame);

    // 订阅主题
    stompClient.subscribe('/topic/public', function(message) {
        const chatMessage = JSON.parse(message.body);
        // 处理接收到的消息
    });

    // 发送消息
    stompClient.send('/app/chat.addUser', {}, JSON.stringify({
        sender: 'username',
        type: 'JOIN'
    }));
});
```

---

## STOMP 消息流程

### 消息类型

1. **客户端发送消息** - 从客户端通过 WebSocket 连接发送
2. **服务端处理消息** - 带有 `@MessageMapping` 注解的方法处理
3. **广播消息** - 通过 `@SendTo` 或 `SimpMessagingTemplate` 广播到订阅者

### 消息目的地前缀

| 前缀 | 用途 |
|------|------|
| `/app` | 应用程序目的地（客户端 -> 服务器） |
| `/topic` | 发布-订阅目的地（服务器 -> 多个客户端） |
| `/queue` | 点对点目的地（服务器 -> 特定客户端） |
| `/user` | 用户专用目的地 |

### 示例：点对点消息

```java
@Controller
public class PrivateMessageController {

    @Autowired
    private SimpMessagingTemplate messagingTemplate;

    @MessageMapping("/chat.privateMessage")
    public void sendPrivateMessage(@Payload ChatMessage message) {
        // 发送消息给特定用户
        messagingTemplate.convertAndSendToUser(
            message.getRecipient(),
            "/queue/messages",
            message
        );
    }
}
```

客户端订阅：

```javascript
// 订阅用户专用队列
stompClient.subscribe('/user/queue/messages', function(message) {
    const chatMessage = JSON.parse(message.body);
    // 处理私有消息
});
```

---

## 安全性

### WebSocket 安全配置

```java
@Configuration
public class WebSocketSecurityConfig {

    @Bean
    public ChannelInterceptor authInterceptor() {
        return new ChannelInterceptor() {
            @Override
            public Message<?> preSend(Message<?> message, MessageChannel channel) {
                StompHeaderAccessor accessor =
                    StompHeaderAccessor.wrap(message);

                if (StompCommand.CONNECT.equals(accessor.getCommand())) {
                    // 验证用户
                    String user = accessor.getFirstNativeHeader("login");
                    String pass = accessor.getFirstNativeHeader("passcode");
                    // ... 验证逻辑
                }

                return message;
            }
        };
    }
}
```

### Token 认证

```javascript
// 连接时发送 token
const socket = new SockJS('/ws');
const stompClient = Stomp.over(socket);

const headers = {
    'Authorization': 'Bearer ' + token
};

stompClient.connect(headers, function(frame) {
    // 连接成功
});
```

---

## 参考资料

- [RFC 6455 - WebSocket Protocol](https://datatracker.ietf.org/doc/html/rfc6455)
- [Spring WebSocket Documentation](https://docs.spring.io/spring-framework/reference/web/websocket.html)
- [STOMP Protocol Specification](https://stomp.github.io/)
- [SockJS Client](https://github.com/sockjs/sockjs-client)
- [Stomp.js](https://github.com/stomp-js/stompjs)
