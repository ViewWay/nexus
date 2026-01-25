# Upgrading Spring Boot / 升级 Spring Boot

Source: https://docs.springframework.org.cn/spring-boot/upgrading.html

---

## English

Instructions for upgrading from earlier versions of Spring Boot are available on the project wiki. Follow the links in the release notes section to find the version you are upgrading to.

Upgrade instructions are always the first item in the release notes. If you are multiple versions behind, please make sure to look at the release notes for the versions you skipped as well.

## Upgrading from 1.x

If you are upgrading from Spring Boot `1.x`, check out the migration guide on the project wiki which provides detailed upgrade instructions. You should also review the release notes for a list of "new and noteworthy" features for each release.

## Upgrading to a New Feature Version

When upgrading to a new feature version, some properties may have been renamed or removed. Spring Boot provides a way to analyze your application's environment and print diagnostics at startup, as well as temporarily migrate properties for you at runtime. To enable this feature, add the following dependency to your project:

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-properties-migrator</artifactId>
    <scope>runtime</scope>
</dependency>
```

|  |  |
| --- | --- |
|  | Properties that are added late to the environment (for example when using `@PropertySource`) will not be considered. |
|  |  |
| --- | --- |
|  | Make sure to remove the module from your project's dependencies once the migration is complete. |

## Upgrading Spring Boot CLI

To upgrade an existing CLI installation, use the appropriate package manager command (for example, `brew upgrade`). If you manually installed the CLI, follow the standard instructions, remembering to update your `PATH` environment variable to remove any older references.

---

## 中文 / Chinese

项目 wiki 上提供了从早期版本的 Spring Boot 升级的说明。请按照发行说明部分中的链接找到您要升级到的版本。

升级说明始终是发行说明中的第一项。如果您落后多个版本，请确保您也查看已跳过的版本的发布说明。

## 从 1.x 升级

如果您从 Spring Boot 的 `1.x` 版本升级，请查看项目 Wiki 上的迁移指南，该指南提供了详细的升级说明。还可以查看发行说明，了解每个版本的"新增和值得关注的"功能列表。

## 升级到新的功能版本

升级到新的功能版本时，某些属性可能已被重命名或删除。Spring Boot 提供了一种方法来分析应用程序的环境并在启动时打印诊断信息，还可以为您临时迁移运行时的属性。要启用此功能，请将以下依赖项添加到您的项目中：

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-properties-migrator</artifactId>
    <scope>runtime</scope>
</dependency>
```

|  |  |
| --- | --- |
|  | 在环境中后期添加的属性（例如，使用 `@PropertySource` 时）将不被考虑。 |
|  |  |
| --- | --- |
|  | 迁移完成后，请确保从项目的依赖项中删除此模块。 |

## 升级 Spring Boot CLI

要升级现有的 CLI 安装，请使用相应的包管理器命令（例如，`brew upgrade`）。如果您手动安装了 CLI，请按照标准说明操作，记住更新您的 `PATH` 环境变量以删除任何旧的引用。
