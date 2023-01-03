# 第09章：错误处理

## 概述

rust 的可靠性依赖于编译时的提示与处理。

错误的分类：
- 可恢复错误：Result<T, E>
- 不可恢复错误：panic宏

当 panic! 宏执行时，程序会打印一个错误信息，展开（unwind）、清理调用栈，退出程序。

默认情况下，当 panic 发生时，程序展开调用栈或立即中止调用栈。如果想要将二进制文件更小，把设置从“展开”改为“中止”。

```ini
# Cargo.toml
[profile]
panic = 'abort'
```

panic 信息可能出现在自己写的代码中，也可能出现在我们调用的代码中，回了获得 panic! 更详细的回溯信息：
- 必须开启调试模式，不能带 --release；
- 设置 RUST_BACKTRACE 环境变量；

## Result 枚举

- 常见格式
```rust
enum Result <T, E> {
	Ok(T),
	Err(E),
}
```

T：操作成功情况下，Ok 变体里返回的数据类型；

E：操作失败情况下，Err 变体返回的错误类型；

## 匹配不同的错误

match 很有用，但很原始，一般是使用闭包来处理，通过 Result<T, E> 接收闭包作为参数，使用 match 实现。

### unwrap

unwrap 是 match 表达式的一种快捷写法，如果 Ok 返回 Ok 里面的值，如果 Err 调用 panic! 宏

### expect

与 unwrap 类似，但可以指定错误信息

## 传播错误

- 在函数中处理错误；
- 将错误返回给调用者；

### ？运算符传播错误

如果是 Ok，则 Ok 中的值就是表达式的结果，然后继续执行；
如果是 Err，则 Err 就是整个函数的返回值，类似于 return；

```rust
use std::io::{self, Read};
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
	let mut s = String::new();
	// let mut f = File::open("hello.txt")?;
	// f.read_to_string(&mut s)?;

	File::open("hello.txt")?.read_to_string(&mut s)?;
	Ok(s)
}

fn main() {
	let result = read_username_from_file();
}
```

### ? 运算符与 from 函数

- Trait std::convert::From 上的函数用于错误之间的转换；
- 被 ? 所应用的错误，会隐式的被 from 函数处理
- 当 ? 调用 from 函数时，它接收的错误类型会被转化为当前函数返回类型所定义的错误类型

主要用于针对不同错误原因，返回同一种错误类型的情况。

### ? 运算符与 main 函数

main 函数返回值是 ()，也可以是 Result<T, E>；Box<dyn Error> 是 tarit 对象，简单理解为任何可能的错误。


```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result <(), Box<dyn Error>> {
	let f = File::open("hello.txt")?;
	Ok(())
}

```

## 总体原则

在定义一个可能失败的函数时，优先考虑返回 Result，否则就 panic!

### 使用 panic! 的某些场景

- 演示某些概念：unwrap
- 原型代码：unwrap、expect
- 测试：unwrap，expect

### 指导性建议

- 损坏状态时指某些假设、保证、约定或者不可变性被打破，满足以下任一情况：
  - 非法的值、矛盾的值、空缺的值被传入代码
  - 不是预期能够偶尔发生的；
  - 在此之后会停止运行；
  - 没有好的方法来将这些信息进行编码；
- 当代码处于损坏状态时，最好使用 panic!

### 场景建议
- 调用你的代码，传入无意义的参数：panic!
- 调用外部不可控代码，返回非法状态，无法修复：panic!
- 如果失败是可预期的：Result
- 需要对值进行操作前，要进行合法性验证：panic!