# 电子邮件

本节描述了如何使用 Spring Framework 发送电子邮件。

## 库依赖

为了使用 Spring Framework 的电子邮件支持，您需要在应用程序的类路径中包含以下 JAR 包：

- The Jakarta Mail library

此库可在网上免费获取——例如，在 Maven Central 中为 `com.sun.mail:jakarta.mail`。请确保使用最新的 2.x 版本（使用 `jakarta.mail` 包命名空间），而不是 Jakarta Mail 1.6.x（使用 `javax.mail` 包命名空间）。

Spring Framework 提供了一个有用的工具库来发送电子邮件，它使您无需关注底层邮件系统的具体细节，并代表客户端负责低级资源处理。

`org.springframework.mail` 包是 Spring Framework 电子邮件支持的根级别包。发送电子邮件的核心接口是 `MailSender` 接口。一个封装简单邮件属性（如 `from` 和 `to` 以及许多其他属性）的简单值对象是 `SimpleMailMessage` 类。此包还包含一个受检异常层次结构，它在较低级别的邮件系统异常之上提供了更高级别的抽象，其根异常是 `MailException`。

`org.springframework.mail.javamail.JavaMailSender` 接口为 `MailSender` 接口（它继承自该接口）添加了专门的 JavaMail 特性，例如 MIME 消息支持。`JavaMailSender` 还提供了一个名为 `org.springframework.mail.javamail.MimeMessagePreparator` 的回调接口，用于准备 `MimeMessage`。

## 用法

### `MailSender` 和 `SimpleMailMessage` 的基本用法

以下示例展示了如何使用 `MailSender` 和 `SimpleMailMessage` 发送电子邮件：

```java
public class SimpleOrderManager implements OrderManager {

    private MailSender mailSender;
    private SimpleMailMessage templateMessage;

    public void setMailSender(MailSender mailSender) {
        this.mailSender = mailSender;
    }

    public void setTemplateMessage(SimpleMailMessage templateMessage) {
        this.templateMessage = templateMessage;
    }

    @Override
    public void placeOrder(Order order) {
        // Do the business calculations...
        // Call the collaborators to persist the order...

        // Create a thread-safe "copy" of the template message and customize it
        SimpleMailMessage msg = new SimpleMailMessage(this.templateMessage);
        msg.setTo(order.getCustomer().getEmailAddress());
        msg.setText(
            "Dear " + order.getCustomer().getFirstName()
                + order.getCustomer().getLastName()
                + ", thank you for placing order. Your order number is "
                + order.getOrderNumber());
        try {
            this.mailSender.send(msg);
        }
        catch (MailException ex) {
            // simply log it and go on...
            System.err.println(ex.getMessage());
        }
    }
}
```

以下示例显示了前面代码的 Bean 定义：

```java
@Bean
JavaMailSender mailSender() {
    JavaMailSenderImpl mailSender = new JavaMailSenderImpl();
    mailSender.setHost("mail.mycompany.example");
    return mailSender;
}

@Bean // this is a template message that we can pre-load with default state
SimpleMailMessage templateMessage() {
    SimpleMailMessage message = new SimpleMailMessage();
    message.setFrom("[email protected]");
    message.setSubject("Your order");
    return message;
}

@Bean
SimpleOrderManager orderManager(JavaMailSender mailSender, SimpleMailMessage templateMessage) {
    SimpleOrderManager orderManager = new SimpleOrderManager();
    orderManager.setMailSender(mailSender);
    orderManager.setTemplateMessage(templateMessage);
    return orderManager;
}
```

### 使用 `JavaMailSender` 和 `MimeMessagePreparator`

本节描述了 `OrderManager` 的另一种实现，它使用了 `MimeMessagePreparator` 回调接口：

```java
import jakarta.mail.Message;
import jakarta.mail.MessagingException;
import jakarta.mail.internet.InternetAddress;
import jakarta.mail.internet.MimeMessage;
import org.springframework.mail.MailException;
import org.springframework.mail.javamail.JavaMailSender;
import org.springframework.mail.javamail.MimeMessagePreparator;

public class SimpleOrderManager implements OrderManager {

    private JavaMailSender mailSender;

    public void setMailSender(JavaMailSender mailSender) {
        this.mailSender = mailSender;
    }

    public void placeOrder(final Order order) {
        // Do the business calculations...
        // Call the collaborators to persist the order...

        MimeMessagePreparator preparator = new MimeMessagePreparator() {
            public void prepare(MimeMessage mimeMessage) throws Exception {
                mimeMessage.setRecipient(Message.RecipientType.TO,
                    new InternetAddress(order.getCustomer().getEmailAddress()));
                mimeMessage.setFrom(new InternetAddress("[email protected]"));
                mimeMessage.setText("Dear " + order.getCustomer().getFirstName() + " " +
                    order.getCustomer().getLastName() + ", thanks for your order. " +
                    "Your order number is " + order.getOrderNumber() + ".");
            }
        };

        try {
            this.mailSender.send(preparator);
        }
        catch (MailException ex) {
            // simply log it and go on...
            System.err.println(ex.getMessage());
        }
    }
}
```

> 邮件代码是一个横切关注点，很有可能被重构为一个自定义的 Spring AOP 切面，然后可以在 `OrderManager` 目标上的适当连接点运行。

## 使用 JavaMail 的 `MimeMessageHelper`

在处理 JavaMail 消息时，一个非常方便的类是 `org.springframework.mail.javamail.MimeMessageHelper`，它使您无需使用冗长的 JavaMail API。使用 `MimeMessageHelper`，创建 `MimeMessage` 非常容易：

```java
// of course you would use DI in any real-world cases
JavaMailSenderImpl sender = new JavaMailSenderImpl();
sender.setHost("mail.host.com");

MimeMessage message = sender.createMimeMessage();
MimeMessageHelper helper = new MimeMessageHelper(message);
helper.setTo("[email protected]");
helper.setText("Thank you for ordering!");

sender.send(message);
```

### 发送附件和内嵌资源

Multipart 电子邮件允许包含附件和内嵌资源。内嵌资源的示例包括您希望在消息中使用但不希望显示为附件的图像或样式表。

#### 附件

以下示例展示了如何使用 `MimeMessageHelper` 发送带有单个 JPEG 图像附件的电子邮件：

```java
JavaMailSenderImpl sender = new JavaMailSenderImpl();
sender.setHost("mail.host.com");

MimeMessage message = sender.createMimeMessage();

// use the true flag to indicate you need a multipart message
MimeMessageHelper helper = new MimeMessageHelper(message, true);
helper.setTo("[email protected]");

helper.setText("Check out this image!");

// let's attach the infamous windows Sample file (this time copied to c:/)
FileSystemResource file = new FileSystemResource(new File("c:/Sample.jpg"));
helper.addAttachment("CoolImage.jpg", file);

sender.send(message);
```

#### 内嵌资源

以下示例展示了如何使用 `MimeMessageHelper` 发送带有内嵌图像的电子邮件：

```java
JavaMailSenderImpl sender = new JavaMailSenderImpl();
sender.setHost("mail.host.com");

MimeMessage message = sender.createMimeMessage();

// use the true flag to indicate you need a multipart message
MimeMessageHelper helper = new MimeMessageHelper(message, true);
helper.setTo("[email protected]");

// use the true flag to indicate the text included is HTML
helper.setText("<html><body><img src='cid:identifier1234'></body></html>", true);

// let's include the infamous windows Sample file (this time copied to c:/)
FileSystemResource res = new FileSystemResource(new File("c:/Sample.jpg"));
helper.addInline("identifier1234", res);

sender.send(message);
```

> 内嵌资源是使用指定的 `Content-ID`（在上述示例中为 `identifier1234`）添加到 `MimeMessage` 中的。添加文本和资源的顺序非常重要。务必先添加文本，然后再添加资源。如果顺序颠倒，将不起作用。

### 使用模板库创建电子邮件内容

前面章节示例中的代码使用诸如 `message.setText(..)` 之类的方法调用显式创建了电子邮件消息的内容。对于简单情况来说，这很好，并且在上述示例的上下文也是可以接受的，其目的是向您展示 API 的基础知识。

然而，在典型的企业应用程序中，开发人员通常不会出于多种原因使用前面所示的方法来创建电子邮件消息的内容：

- 在 Java 代码中创建基于 HTML 的电子邮件内容既繁琐又容易出错。
- 显示逻辑和业务逻辑之间没有明确的分离。
- 更改电子邮件内容的显示结构需要编写 Java 代码、重新编译、重新部署等等。

通常，解决这些问题的方法是使用模板库（例如 FreeMarker）来定义电子邮件内容的显示结构。这样，您的代码只需负责创建要在电子邮件模板中渲染的数据并发送电子邮件。当您的电子邮件内容变得稍微复杂时，这绝对是一种最佳实践，并且借助 Spring Framework 对 FreeMarker 的支持类，实现起来非常容易。
