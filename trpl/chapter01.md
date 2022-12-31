# 第01章 rust

## 安装
### 官网安装

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### 包管理安装

在 Debian，Ubuntu，Mint 下

```sh
sudo apt install cargo
```

在 CentOS，openEuler 下：

```sh
sudo dnf install cargo
```

## 镜像源

中国有很多大学都同步了 rust 源，包括 tuna，ustc，字节也退出了自己的 rust 源。

将以下内容放到 `$HOME/.cargo/config` 文件中。

```sh
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# 替换成你偏好的镜像源
replace-with = 'tuna'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

```

或

```sh
[source.crates-io]
# To use sparse index, change 'rsproxy' to 'rsproxy-sparse'
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

编辑 .zshrc 或 .bashrc，加入

```sh
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
```

## cargo

cargo 是 rust 的构建工具，以及包管理器。

### 生成模板

```sh
cargo new hello_cargo
```

Rust 中，代码包被称为 crate，模板工具会生成一系列文件：

- Cargo.toml：项目信息，依赖

```sh
[package]
name
version
authors
edition

[dependencies]
```
- src/：存放源代码目录
- Cargo.lock：这个是编译后生成的，用以重现构建过程，并加速构建。
- 初始化一个 Git 仓库

### 常用命令
- cargo build：构建，后面可以加入 --release 或 --debug
- cargo check：只检测不实际编译
- cargo run：运行
- cargo doc：构建本地文档，加入 --open 直接打开
