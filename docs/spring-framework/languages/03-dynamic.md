# Dynamic Language Support

Spring provides comprehensive support for using classes and objects defined through dynamic languages (such as Groovy). This support allows you to write any number of classes in supported dynamic languages and have the Spring container transparently instantiate, configure, and dependency inject the resulting objects.

Spring's scripting support is primarily focused on Groovy and BeanShell. In addition to these specific supported languages, JSR-223 scripting mechanism is also supported for integration with any JSR-223 compliant language provider (starting from Spring 4.2), such as JRuby.

## First Example

This chapter describes in detail the dynamic language support. Before diving into all aspects of dynamic language support, let's look at a quick example of a bean defined using a dynamic language. The first bean uses Groovy as the dynamic language. (The basis of this example is taken from the Spring test suite. If you want to see equivalent examples for other supported languages, please consult the source code.)

The following example shows the `Messenger` interface that the Groovy bean will implement. Note that this interface is defined in pure Java. Dependent objects that have a reference to `Messenger` injected are unaware that the underlying implementation is a Groovy script. The following listing shows the `Messenger` interface:

```java
package org.springframework.scripting;

public interface Messenger {
    String getMessage();
}
```

The following example defines a class that depends on the `Messenger` interface:

```java
package org.springframework.scripting;

public class DefaultBookingService implements BookingService {
    private Messenger messenger;

    public void setMessenger(Messenger messenger) {
        this.messenger = messenger;
    }

    public void processBooking() {
        // use the injected Messenger object...
    }
}
```

The following example implements the `Messenger` interface using Groovy:

```groovy
package org.springframework.scripting.groovy

// Import the Messenger interface (written in Java) that is to be implemented
import org.springframework.scripting.Messenger

// Define the implementation in Groovy in file 'Messenger.groovy'
class GroovyMessenger implements Messenger {
    String message
}
```

Finally, the following example shows the bean definition that injects the Groovy-defined `Messenger` implementation into an instance of the `DefaultBookingService` class:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<beans xmlns="http://www.springframework.org/schema/beans" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:lang="http://www.springframework.org/schema/lang"
    xsi:schemaLocation="
        http://www.springframework.org/schema/beans https://www.springframework.org/schema/beans/spring-beans.xsd
        http://www.springframework.org/schema/lang https://www.springframework.org/schema/lang/spring-lang.xsd">

    <!-- this is the bean definition for the Groovy-backed Messenger implementation -->
    <lang:groovy id="messenger" script-source="classpath:Messenger.groovy">
        <lang:property name="message" value="I Can Do The Frug" />
    </lang:groovy>

    <!-- an otherwise normal bean that will be injected by the Groovy-backed Messenger -->
    <bean id="bookingService" class="x.y.DefaultBookingService">
        <property name="messenger" ref="messenger" />
    </bean>

</beans>
```

The `bookingService` bean (a `DefaultBookingService`) can now use its private member variable `messenger` as usual, since the `Messenger` instance injected into it is a `Messenger` instance. There is nothing special here - just plain Java and plain Groovy.

## Defining Beans Backed by Dynamic Languages

This section describes in detail how to define Spring-managed beans in any supported dynamic language.

### Common Concepts

Using beans backed by dynamic languages involves the following steps:

1. Write tests for the dynamic language source code (this is a matter of course).
2. Then write the dynamic language source code itself.
3. Define the beans backed by dynamic languages in XML configuration using the corresponding `<lang:language/>` element (you can programmatically define such beans using the Spring API, but you need to consult the source code for operational guidelines, as this chapter does not cover such advanced configuration). Note that this is an iterative step. Each dynamic language source file requires at least one bean definition (although multiple bean definitions can reference the same source file).

#### The `<lang:language/>` Element

The last step in the previous list involves defining bean definitions backed by dynamic languages, one for each bean you want to configure (which is no different from normal JavaBean configuration). However, instead of specifying the fully qualified class name of the class to be instantiated and configured by the container, you can use the `<lang:language/>` element to define beans backed by dynamic languages.

Each supported language has a corresponding `<lang:language/>` element:

- `<lang:groovy/>` (Groovy)
- `<lang:bsh/>` (BeanShell)
- `<lang:std/>` (JSR-223, for example with JRuby)

The exact attributes and child elements available for configuration depend on which language the bean is defined with (the language-specific sections later in this chapter provide details).

#### Refreshable Beans

One of the most compelling value-adds of Spring's dynamic language support (perhaps the only one) is the "refreshable bean" feature.

A refreshable bean is a bean backed by a dynamic language. With a small amount of configuration, a bean backed by a dynamic language can monitor changes to its underlying source file resources and reload itself when the dynamic language source file changes (for example, when you edit and save file changes on the file system).

This allows you to deploy any number of dynamic language source files as part of your application, configure the Spring container to create beans backed by the dynamic language source files (using the mechanisms described in this chapter), and then (as requirements change or due to some other external factors) edit the dynamic language source files and have any changes reflected in the beans backed by the changed dynamic language source files. There is no need to shut down the running application (or redeploy in the case of a web application). Beans backed by the modified dynamic language source pick up the new state and logic from the changed dynamic language source file.

This feature is disabled by default.

To turn on the refreshable bean feature, you only need to specify an additional attribute on the `<lang:language/>` element of the bean definition. Using the earlier example, the following example shows what changes we would make to our Spring XML configuration to implement a refreshable bean:

```xml
<beans>
    <!-- this bean is now 'refreshable' due to the presence of the 'refresh-check-delay' attribute -->
    <lang:groovy id="messenger"
            refresh-check-delay="5000" <!-- switches refreshing on with 5 seconds between checks -->
            script-source="classpath:Messenger.groovy">
        <lang:property name="message" value="I Can Do The Frug" />
    </lang:groovy>

    <bean id="bookingService" class="x.y.DefaultBookingService">
        <property name="messenger" ref="messenger" />
    </bean>
</beans>
```

The `refresh-check-delay` attribute defined on the `messenger` bean definition indicates how many milliseconds after any changes to the underlying dynamic language source file the bean will be refreshed. You can turn off the refresh behavior by assigning a negative value to the `refresh-check-delay` attribute. Remember that, by default, the refresh behavior is disabled.

#### Inline Dynamic Language Source Files

Dynamic language support can also handle dynamic language source files embedded directly in Spring bean definitions. More specifically, the `<lang:inline-script/>` element allows you to define dynamic language source immediately inside the Spring configuration file:

```xml
<lang:groovy id="messenger">
    <lang:inline-script>
        package org.springframework.scripting.groovy

        import org.springframework.scripting.Messenger

        class GroovyMessenger implements Messenger {
            String message
        }
    </lang:inline-script>
    <lang:property name="message" value="I Can Do The Frug" />
</lang:groovy>
```

#### Understanding Constructor Injection in Dynamic Language Backed Beans

One very important thing to note about Spring's dynamic language support is that you cannot currently supply constructor arguments for beans backed by dynamic languages (and therefore, beans backed by dynamic languages do not support constructor injection).

## Groovy Beans

This section describes how to use beans defined in Groovy within Spring.

From the Groovy homepage:

"Groovy is an agile dynamic language for the Java 2 Platform that has many of the features that people like in languages such as Python, Ruby and Smalltalk, making them available to Java developers using a Java-like syntax."

```groovy
package org.springframework.scripting.groovy

// from the file 'calculator.groovy'
class GroovyCalculator implements Calculator {
    int add(int x, int y) {
        x + y
    }
}
```

```xml
<!-- from the file 'beans.xml' -->
<beans>
    <lang:groovy id="calculator" script-source="classpath:calculator.groovy"/>
</beans>
```

### Customizing Groovy Objects with Callbacks

The `GroovyObjectCustomizer` interface is a callback interface that lets you add additional creation logic during the creation of a Groovy-backed bean. For example, an implementation of this interface could call any necessary initialization methods, set some default property values, or specify a custom `MetaClass`:

```java
public interface GroovyObjectCustomizer {
    void customize(GroovyObject goo);
}
```

## BeanShell Beans

This section introduces how to use BeanShell beans in Spring.

From the BeanShell homepage:

"BeanShell is a small, free, embeddable Java source interpreter with dynamic language features, written in Java. BeanShell dynamically runs standard Java syntax and extends it with common scripting conveniences such as loose types, commands, and method closures like those in Perl and JavaScript."

Unlike Groovy, BeanShell-based bean definitions require some (small) additional configuration. Spring's implementation of BeanShell dynamic language support is interesting because Spring creates a JDK dynamic proxy that implements all the interfaces specified in the `script-interfaces` attribute value of the `<lang:bsh>` element.

```xml
<lang:bsh id="messageService" script-source="classpath:BshMessenger.bsh"
    script-interfaces="org.springframework.scripting.Messenger">
    <lang:property name="message" value="Hello World!" />
</lang:bsh>
```

## Scenarios

### Scripting Spring MVC Controllers

One class of beans that can benefit from using dynamic language-backed beans is Spring MVC controllers. In a pure Spring MVC application, the navigation flow of the web application is largely determined by the code encapsulated in Spring MVC controllers.

```groovy
package org.springframework.showcase.fortune.web

import org.springframework.showcase.fortune.service.FortuneService
import org.springframework.showcase.fortune.domain.Fortune
import org.springframework.web.servlet.ModelAndView
import org.springframework.web.servlet.mvc.Controller

import jakarta.servlet.http.HttpServletRequest
import jakarta.servlet.http.HttpServletResponse

// from the file '/WEB-INF/groovy/FortuneController.groovy'
class FortuneController implements Controller {
    @Property FortuneService fortuneService

    ModelAndView handleRequest(HttpServletRequest request,
            HttpServletResponse httpServletResponse) {
        return new ModelAndView("tell", "fortune", this.fortuneService.tellFortune())
    }
}
```

```xml
<lang:groovy id="fortune"
        refresh-check-delay="3000"
        script-source="/WEB-INF/groovy/FortuneController.groovy">
    <lang:property name="fortuneService" ref="fortuneService"/>
</lang:groovy>
```

### Scripting Validators

Another area of Spring application development that can benefit from the flexibility provided by dynamic language-backed beans is validation. Using a loosely-typed dynamic language (possibly with inline regular expression support) to express complex validation logic may be easier than using regular Java.

```groovy
import org.springframework.validation.Validator
import org.springframework.validation.Errors
import org.springframework.beans.TestBean

class TestBeanValidator implements Validator {
    boolean supports(Class clazz) {
        return TestBean.class.isAssignableFrom(clazz)
    }

    void validate(Object bean, Errors errors) {
        if(bean.name?.trim()?.size() > 0) {
            return
        }
        errors.reject("whitespace", "Cannot be composed wholly of whitespace.")
    }
}
```

## Additional Details

### AOP - Advising Scripted Beans

You can use the Spring AOP framework to advise scripted beans. The Spring AOP framework is not actually aware that a bean being advised might be a scripted bean, so all of the AOP use cases and features that you use (or intend to use) apply to scripted beans. When advising scripted beans, you cannot use class-based proxies. You must use interface-based proxies.

### Scoping

Scripted beans can be scoped just like any other bean. The `scope` attribute on the various `<lang:language/>` elements allows you to control the scope of the underlying scripted bean, just as you would with a regular bean. (The default scope is singleton, the same as for "regular" beans).

```xml
<?xml version="1.0" encoding="UTF-8"?>
<beans xmlns="http://www.springframework.org/schema/beans" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:lang="http://www.springframework.org/schema/lang"
    xsi:schemaLocation="
        http://www.springframework.org/schema/beans https://www.springframework.org/schema/beans/spring-beans.xsd
        http://www.springframework.org/schema/lang https://www.springframework.org/schema/lang/spring-lang.xsd">

    <lang:groovy id="messenger" script-source="classpath:Messenger.groovy" scope="prototype">
        <lang:property name="message" value="I Can Do The RoboCop" />
    </lang:groovy>

    <bean id="bookingService" class="x.y.DefaultBookingService">
        <property name="messenger" ref="messenger" />
    </bean>

</beans>
```

## More Resources

The following links point to more resources for the various dynamic languages referenced in this chapter:

- Groovy homepage
- BeanShell homepage
- JRuby homepage
