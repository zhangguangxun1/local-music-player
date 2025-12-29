# Local music player

我以为 [Slint](https://docs.slint.dev/latest/docs/slint/) 这种原生支持打包后文件会很小, 比如只有 5-6M, 实际上还是有 15M 左右, 并没有那么轻量

## 引用

其实好些语法都不太清楚, 还参考或者引用了这些项目或者资源:

[zeedle](https://github.com/Jordan-Haidee/zeedle)

[Font Awesome](https://fontawesome.com/)

## Debian 13

如果要在 Debian 13 系统下编译, rodio 需要依赖下面两个库:  

libasound2-dev: 提供了 Rust 编译时需要的 C 语言头文件和底层接口，用于控制音频输出

pkg-config: Rust 的构建脚本（build.rs）需要这个工具来定位系统库的安装路径

## Mac

目前 Mac 上编译部分 struct 在 Mac 环境提示 Send Sync 等实现缺失, 需要增加如下不安全的实现 

```
// 条件编译：只在 macOS 上编译, Mac 平台需要标识这两个不安全的实现, 否则编译器检查不通过
#[cfg(target_os = "macos")]
unsafe impl Send for Player {}

#[cfg(target_os = "macos")]
unsafe impl Sync for Player {}
```

### 中文渲染乱码

Mac Inter x86_64 平台默认的 slint 渲染引擎无法关联系统内置的中文字体, 引入第三方字体又会造成软件包编译体积过大故不考虑

开发中需要预览请在入口文件手动导入

```slint
import "/System/Library/Fonts/Hiragino Sans GB.ttc";
import "/System/Library/Fonts/Supplemental/Songti.ttc";
```

指定 Mac inter x86_64 平台字体默认字体导入, 如果是 m1-... 等可能需要确认字体路径或者本身不存在这个编译问题

其它平台就默认按编译器找到的中文字体即可

Mac 核心中文字体 通常位于 `/System/Library/Fonts` 目录下面, 针对中文字体选择 `冬青黑体` 文件名 `Hiragino Sans GB.ttc` 引入字体名称 `Hiragino Sans GB`

补充中文字体, 通常位于 `/System/Library/Fonts/Supplemental` 目录下面, 针对中文字体选择 `宋体-简` 文件名 `Songti.ttc` 引入字体名称 `Songti SC`

如果某个平台版本这两个字体都不存在, 就需要人为编码支持了.
