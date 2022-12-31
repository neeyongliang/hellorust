# 第07章 Package, Crate, 和 Module

## Package，Crate，定义 Module

### rust 的模块系统

- Package（包）：Cargo 的特性，可以构建、测试、共享 crate
- Crate（单元包）：一个模块树，可以产生一个 library 或可执行文件
- Module（模块）、use：控制组织代码、作用域、私有路径
- Path（路径）：为 struct、function 或 module 等项命名的方式

### Package 与 Crate

- Crate 类型分为 binary 或 library
- Crate Root 是源代码文件，编译器从此构建 Crate 的根 Module
- Pacakge
  - Cargo.toml，描述如何构建这些 Crates
  - 只能包含 0-1 个 library crate
  - 可以包含任意多个 binary crate
  - 但必须至少包含一个 crate

### Module 

用来控制作用域和私有性

- Modeule 定义
  - 在一个 crate 内将代码分组
  - 增加可读性，易于服用
  - 控制项目（item）的私有性，public，private
- Module 建立
  - mod 关键字
  - 可嵌套
  - 可包含其他项（struct，enum，常量，trait，函数等）的定义

### 路径
- 绝对路径：从 crate root 开始，使用 crate 名或 字面值crate
- 相对路径：从当前模块开始，使用 self，super 或当前模块的标识符

pub 放在 struct 前，struct 公有，struct 字段私有；

pub 放在 enum 前，enum 公有，enum 变体也是公有；

## use

- use 可以将外部模块导入
- pub use 可以将导入的模块再共享出去
- use 配合 as 可以实现模块的本地重命名
- use 使用嵌套路径，可以简洁代码
- use 需要谨慎的使用通配符
  - 测试所有模块时使用
  - 预导入模块时使用

```rust
// 分别导入两个路径
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::collections, io};

// 第一个路径是第二个路径的父路径
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// 使用通配符
use std::collections::*;
```

## 拆分模块

模块名后面跟的不是分号，而是代码块，就可以将代码块拆分到模块名的同名文件中。