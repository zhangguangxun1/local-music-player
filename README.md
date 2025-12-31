# Local music player

我以为 [Slint](https://docs.slint.dev/latest/docs/slint/) 这种原生支持打包后文件会很小, 比如只有 5-6M, 实际上还是有 15M 左右, 并没有那么轻量

## 引用

其实好些语法都不太清楚, 还参考或者引用了这些项目或者资源:

[zeedle](https://github.com/Jordan-Haidee/zeedle)

[Font Awesome](https://fontawesome.com/)

## Linux

常见的几个 Linux 发行版编译信息

### Debian 13

如果要在 Debian 13 系统下编译, rodio 需要依赖下面两个库:

libasound2-dev: 提供了 Rust 编译时需要的 C 语言头文件和底层接口，用于控制音频输出

pkg-config: Rust 的构建脚本（build.rs）需要这个工具来定位系统库的安装路径

类似这样的错误

```
error: linker `cc` not found
  |
  = note: No such file or directory (os error 2)
error: could not compile `quote` (build script) due to 1 previous error
```

系统缺少C编译器（如gcc），导致Rust项目无法找到链接器cc

build-essential包 包含gcc、g++和make等必要工具，可解决链接器缺失问题

```
sudo apt install build-essential
```

Debian 除了这么贵的显卡毫无用武之地感觉没太多别的毛病

### Fedora 43

其它系统根据编译报错情况酌情安装缺失的库, 比如 Fedora 缺失的是:

Rust 在编译 alsa-sys 插件时需要连接到系统底层的音频接口（C 语言库）, alsa-lib-devel 包含了 Rust 编译过程中需要的头文件和 .pc 文件（pkg-config 配置文件）, 就需要安装

```
sudo dnf install alsa-lib-devel
```

感觉 Fedora bug 好多好多, 简直无法接受那种

### Manjaro

Manjaro 类似基础组件在 base-devel

```
sudo pacman -Sy base-devel
```

还可能有一些底层 Rust rfd 库的依赖是, 烦的抠脚, 每个系统都 "百花齐放" 的.

```
[2025-12-31T07:38:51Z ERROR rfd::backend::xdg_desktop_portal] Failed to open zenity dialog: No such file or directory (os error 2)
```

类似这种就需要安装 zenity

```
sudo pacman -Sy zenity
```

每个平台的基础库名称都可能不一样, 如果遇到编译中途失败, 可以把错误信息直接复制给 Ai, 同时告诉他自己的系统一般都能给出缺失基础库的名称, 安装即可

Manjaro 也是毛病一大堆多的很, 控制台一直闪烁要瞎了

总体上感觉 Debian 是最省心最干净的, 但是 Debian 驱动不了显卡, 总是总 软件渲染, 一台电脑一半的价钱都在显卡上, 居然用不上谁受得了啊

## Mac

目前 Mac 上编译部分 struct 在 Mac 环境提示 Send Sync 等实现缺失, 需要增加如下不安全的实现 

```
// 条件编译: 只在 macOS Inter x86_64 上编译, Mac 平台需要标识这两个不安全的实现, 否则编译器检查不通过
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
unsafe impl Send for Player {}

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
unsafe impl Sync for Player {}
```

### 开发

Mac OS 上 slint 插件可能没有处理好平台兼容, 尤其是 Inter x86_64 这些老的苹果电脑, 需要自己手动安装 slint-lsp 来实时预览 slint 界面

```
cargo install slint-viewer
cargo install slint-lsp
```

然后配置 IDE 实际去请求本地自己编译完成的 slint-lsp 程序

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
