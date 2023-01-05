# 闭包与迭代器

## 闭包

### 什么是闭包

闭包：可以捕获其在环境的匿名函数。

闭包的特点：

- 匿名函数；
- 保存为变量、参数；
- 可以在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算；
- 可从其定义的作用域捕获值

### 闭包的类型推断

闭包的定义最终只会为参数、返回值推断出唯一具体的类型。

### 泛型，trait bound

可以用来实现延迟计算或者惰性计算。

### 闭包捕获环境变量

```rust
fn main() {
	let x = 4;
	// x 与 equal_to_x 闭包定义在同一作用域，x 被闭包捕获
	let equal_to_x = |z| z == x;
	let y = 4;
	assert_eq(equal_to_x(y));
}
```

函数不能捕获外部变量，而闭包可以，至少要实现以下三种 trait 之一：
- FnOnce：获取所有权，所有的闭包都实现了
- FnMute：可变借用，没有移动捕获变量的闭包，实现了 FnMut
- Fn：不可变借用，无需访问捕获变量的闭包实现了 Fn

### move 关键字

在参数列表前使用 move 关键字，可以强制闭包获取它所使用的的环境值的所有权。

在将闭包传递给新线程以移动数据的方式使其桂新线程所有时，此技术最有用。

```rust
fn main() {
	let x = vec![1, 2, 3];
	let equal_to_x = move |z| z == x;
	// 这里会报错，因为 x 所有权已经发生移动
	println!("cannot use x here: {:?}", x);
	let y = 4;
	assert_eq(equal_to_x(y));
}
```

## 迭代器

迭代器模式：对一系列项执行某些任务。

迭代器负责：遍历每个项，确定序列（遍历）合适完成。

rust 迭代器是惰性的，除非调用消费迭代器的方法，否则迭代器本身并没有任何效果。

### iterator trait

所有的迭代器都实现了 iterator trait

Iterator trait 定义与标准库，定义大致如下：

```rust
pub trait Iterator {
	type Item;
	fn next(&mut self) -> Option<Self::Item>
	// methods with default implementations elided
}
```

Iterator trait 仅要求实现一个方法：next方法，每次返回迭代器中的一项，返回结果包裹在 Some 里，迭代结束反复 None。

### 几个迭代方法

- iter方法：在不可变引用上创建迭代器；
- into_iter方法：创建的迭代器会获得所有权；
- iter_mut方法：迭代器可变的引用；

在 iterator trait 基础之上，可以实现很多其他的方法，包括消耗型适配器(如 sum)，适配型迭代器（如 map）。

```rust
fn iterator_sum() {
	let v1 = vec![1, 2, 3];
	let total: i32 = v1_iter.sum();

	let v2 = v1.iter().map(|x| x + 1).collect(); 
	let v3 = v1.iter().filter(|x| x > 1).collect();
}
```

### 自定义

主要还是实现 next 方法。