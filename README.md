# 

rodio 需要依赖下面两个库, Debian 13

libasound2-dev: 提供了 Rust 编译时需要的 C 语言头文件和底层接口，用于控制音频输出。

pkg-config: Rust 的构建脚本（build.rs）需要这个工具来定位系统库的安装路径。

播放完毕会自己结束运行