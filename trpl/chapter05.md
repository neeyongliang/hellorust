# 第05章 结构体

## struct 定义

- struct 关键字
- 花括号
- 字段（Feild）

```rust
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}
```

## struct 初始化

- 初始化阶段所有字段都需要初始化，如果某个字段式可变的，那整个结构体就是可变的。
- 如果字段名和字段变量名一致时，可以简写。
- 基于某个已有的 struct，初始化 struct时，先写不同的步伐，再加入 ..old_struct 即可。
- 空 struct 一般来用于实现特定的 trait。

## struct 调试 

- 如果要打印调试结构体，需要实现 Display 或 Debug 的 trait
- 增加 `#[derive(Debug)]` 来修饰自定结构体
- 使用 `{:?}` 或 `{:#?}` 来打印结构体

## struct 方法
- 方法与函数类似：fn 关键字，名称，参数，返回值
- 方法与函数不同：
  - 方法是在 struct (或 enum，trait 对象)的上下文中定义；
  - 方法的第一个参数是 self，表示调用这个方法的实例

如果 impl 块中，第一个参数不是 self，这种函数不是方法，叫做关联函数，每个 struct 允许拥有多个 impl 块。