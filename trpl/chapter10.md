# 10 泛型，trait，和生命周期

## 泛型

- 提高代码复用能力
- 具体类或其他属性的抽象替代
- 类型参数：短字符，CamelCase，T 一般表示 type 的缩写

```rust
fn largest<T>(list: &[T]) -> T {...}
```

### 函数中的泛型
- 泛型函数包含参数类型，返回类型
- 结构体中使用泛型
- 枚举中使用泛型
  - 可以让枚举的变体持有泛型数据类型：Option<T>，Result<T, E>
- 方法的定义中使用泛型
  - 在 T 放在 impl 关键字后，表示在类型 T 上实现方法，如 impl<T> Point<PointType1> ；
  - 只针对具体类型实现方法，如 impl PointType1<i32>

```rust
// x, y 必须是相同类型
struct PointType1<T> {
	x: T,
	y: T,
}

// x, y 可以是不同类型
struct PointType2<T, U> {
	x: T,
	y: U,
}

// 表示某个值可能存在
enum Option<T> {
	Some(T),
	None
}

enum Result<T, E> {
	Ok(T),
	Err(E),
}

// 所有 PointType1 都有 x 方法
impl<T> PointType1<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

// 只有 PointType1<i32> 类型才有 x1 方法
impl PointType1<T> {
	fn x1(&self) -> i32 {
		&self.x
	}
}

```

## Trait
- Trait：告诉编译器某种类型具有且可以与其他类型共享的功能
- Trait：抽象的定义共享行为
- Trait bounds（约束）：泛型类型参数指定为实现了特定行为的类型
- Trait：类似于其他语言中的 interface，但有不同的地方

### 定义一个 Trait

Trait 是把方法签名放在一起，来定义实现某种目的所必须的一组行为。
- 关键字为 trait
- 只有方法签名，没有具体实现；
- 可以有多个方法，每个方法签名占一行，以；结尾

```rust
pub trait Summary {
	fn summarize(&self) -> String;
	fn get_date(&self) -> String;
}
```

### 在类型上实现 trait

- 与为类型实现方法类似
- 不同之处：
  - 格式为 impl SomeTrait for SomeType {...}
  - 在 impl 块中，需要对方法签名进行具体实现

来自 trait 的东西，只有 trait 在作用域中才可以使用。

### 实现 trait 的约束

1. 可以在某个类型上实现某个 trait 的前提是这个类型或这个 trait 是在本地 crate 中定义的；
2. 无法为外部类型来实现外部的 trait

### 默认实现

1. 可以在定义时添加对 trait 的默认实现
2. 可以在使用的类型上也可以覆盖默认实现

### trait 作为参数

- 使用 impl Trait 语法，适用于简单的情况
- 使用 Trait bound 语法，适用于通用类型
- 使用 + 连接多个 trait
- 使用 Trait bound 的 where 子语句

```rust
// 接收所有实现了 Summary 这个 trait 的类型
pub fn notify1(item: impl Summary) {
	// ...
}

pub fn notify2<T: Summary>(item: T) {
	// ...
} 

pub fn notify3<T: Summary + Display>(item: T) {
	// ...
}

pub fn notify4(item: impl Summary + Display) {
	// ...
}

pub fn notify5<T, U>(a: T, b: U) -> String
where
	T: Summary + Display, // 要求 T 实现了 Summary 和 Display 的 Trait
	U: Clone + Debug, // 要求 U 实现了 Clone 和 Debug 的 Trait
{
	// ...
}
```

### trait 作为返回类型

- impl Trait 语法：返回类型只能为确定的一种类型
- 使用 Trait Bound 有条件返回类型
  1. 在 impl 上使用 Trait Bound 可以为”实现了某种特定 Trait 的类型“来实现方法
  2. 也可以为实现了其他 Trait 的任意类型有条件的实现某个 Trait
  3. 为满足 Trait Bound 的所有类型上实现 Trait 叫做覆盖实现
```rust
// 针对所有实现了 Display 的类型 T 实现 ToString Trait
impl <T: fmt::Display> ToString for T {
	// ...
}
```

## 生命周期

rust 的每个引用都有自己的生命周期，引用在生命周期内保持有效的作用域，大多数生命周期都是隐式的，可推导的。

当生命周期可能以不同的方式相互关联时，需要手动标注生命周期。生命周期的主要目标，就是避免悬垂引用。

### 生命周期的语法标注

```rust
// ‘a 生命周期表示 x, y 中生命周期较小的变量的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}
```

- 生命周期的标注不会改变生命周期的长度；
- 当指定了泛型的声明周期参数，函数可以接收带有任何生命周期的引用；
- 生命周期的标注，描述了多个应用的生命周期间的关系，但不影响生命周期；

标注语法：

- 以‘ 开头，通常全小写且非常短，很多人使用 ’a
- 在引用符号 & 后，使用空格将标注和引用类型分开

```rust
&i32    // 一个引用
&‘a i32 // 带有显式生命周期的引用
&’a mut i32 // 带有显示生命周期的可变引用
```

### 深入理解生命周期

指定生命周期参数的方式依赖于函数所做的事情，从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期匹配。

结构体类型中，可以使用自包含类型（如 i32），可以使用引用类型，但要对每个引用增加生命周期。

### 生命周期的省略规则

在 rust 引用分析中所编入的模式成为生命周期的省略规则。生命周期的省略规则不能覆盖的情况，需要手动进行标注。

出现在函数方法的参数中，称之为输入生命周期；出现在函数返回值中，称之为输出生命周期。

一般三个规则：
1. 每个引用类型的参数都有自己的生命周期；
2. 如果只有 1 个输入生命周期，那么该生命周期被赋给所有输出生命周期参数；
3. 如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self（是方法），那么 self 的生命周期会被赋给所有输出的生命周期参数 。

说明：1用于输入生命周期，2，3用于输出生命周期，如果3条规则使用完成后仍然无法确定，编译器就会报错，这些规则也适用于 fn 定义和 impl 块。

```rust
fn first_world(s: &str) -> &str {}
// 规则1：每个引用类型都有自己的生命周期
fn first_world<'a>(s: &'a str) -> &str{}
// 规则2：如果只有1个输入生命周期，该周期会赋给输出
fn first_world<'a>(s: &'a str) -> &'a str{}

fn longest(x: &str, y: &str) -> &str{}
// 规则1
fn longest<'a, 'b>(x: &'a, y: &'b) -> &str {}
// 规则3：不是方法，没有 self 参数
// 编译器会报错
```

### 方法定义中的生命周期标注

- 在 struct 上使用生命周期实现方法，语法和泛型参数一样，依赖于生命周期的参数是否和字段、方法的参数或返回值有关；
- 在 struct 字段的生命周期名，在 impl 后声明，在 struct 后使用，这些声明周期是 struct 的一部分；

```rust
struct ImportantExcerpt<'a> {
	part: &'a str,
}

// 这两个不可省略
impl<'a> ImportantExcerpt<'a> {
	fn level(&self) -> i32 {
		3
	}

	// 两个参数，第一个为 self，会把 self 的生命周期传递给返回值
	fn announce_and_return_part(&self, announcement: &str) -> &str {
		self.part
	}
}
```

### 静态生命周期

- 'static 是整个程序的生命周期

```rust
use std::fmt::Display;

// 本例使用了泛型，trait bound，生命周期
// 函数接收三个参数，a, b, 以及所有实现了 Display trait 的类型 T
// 函数执行打印 T 的值并返回较长的那个变量
fn longest_with_an_announcement<'a, T>(x: &'a, y: &'a, ann: T) -> &'a str
where：
	T: Display,
{
	println!("Announcemnet! {}", ann);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}
```