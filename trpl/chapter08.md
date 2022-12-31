# 第08章 常用的集合

## Vector

Vector 由标准库提供，可以存储多个相同类型的值，这些值在内存中是连续分配的。

### 创建 Vector

- 使用 Vector::new()
- 使用指定初始值创建

### 更新 Vector

可以使用 push 方法

### 删除 Vector

离开作用域之后，会被清理掉

### 读取 Vector

- 索引，超出范围会引发 panic
- 使用 get 方法

```sh
let v: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3]

v.push(4);
v.push(5);
v.push(6);

let third: &i32 = &v[2];

match v.get(2) {
	Some(third) => println!("The third element is {}", third);
	None => println!("There is no third element"),
}

```

## String

### 创建 String
- Vector 操作可以用于 String
- String::new() 创建一个空字符串
- 一般是使用初始值来创建字符串

### 更新 String

- push_str：把一个字符串切片附加到 String
- push：把一个字符附加到 String
- +：连接字符串

```rust
let v = "hello, world".to_string();
let mut u = String::from("hello, rust");
u.push_str("!");
```

String 不允许索引，String 内部存储 UTF-8 编码，必须按照字符边界进行切割。

## HashMap

创建 K，V 键值对，通过 K 进行检索

### 创建 HashMap

创建 HashMap::new()，不常用且不在预加载中，数据存储在 heap 上。HashMap 键与键之间，值与值之间必须是同一种类型。

也可以使用 collect 创建，但使用场景局限。

```rust
use std::collections::HashMap;

fn main() {
	let teams = vec![String::from("Blue"), String::from("Yello")];
	let intial_scores = vec![ 10, 50 ];
	let scores: HashMap<_, _> = 
		teams.iter().zip(intial_scores.iter()).collect();
}
```

### HashMap 和所有权

- 对于实现了 Copy trait 的类型，值会被复制到 HashMap 中
- 对于拥有所有权的值，值会被移动，所有权会转移给 HashMap
- 如果将值的引用插入到 HashMap 中，被引用的值必须保持有效
  - HashMap 有效期内，被引用的值必须保持有效

### 访问 HashMap 的值

- get 方法：参数 K，返回 Option<&V>
- for 循环 for (k, v) in SomeHashMap {}

### 更新 HashMap

使用 entry(K).or_insert(V) 的形式

### Hash 函数

默认情况下， HashMap 使用加密功能强大的 Hash 函数，可以抵御拒绝服务攻击，但速度不是最快。

可以指定不同的 Hasher 来切换到另一个实现了 BuildHasher trait 的类型。


