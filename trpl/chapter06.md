# 第06章 枚举

## 枚举类型

- 适合列出所有可能情况的类型。
- 可以在枚举中嵌入任意类型。
- 可以为枚举定义方法，使用 impl 关键字。

```rust
struct IpV4 {
	// ...
}

struct IpV6 {
	// ...
}

enum IpAddrType {
	V4(IpV4),
	V6(IpV6)
}

impl IpAddrType {
	fn bind(self: &IpAddrType) {
		// ...
	} 
}

```

## Option 枚举

Option<T> 的出现，是为了解决 rust 没有 NULL 的问题，如果想要使用 T 类型，首选得对 Option<T> 进行非空判断。

## match

- match 需要穷举所有可能
- 可以使用 _ 作为通配符，需要放到最后面

## if let

处理只关心一种匹配，忽略其他匹配的情况。

```rust
fn main() {
	let v = Some(0u8);
	// match 写法
	match v {
		Some(3) => println!("three"),
		_ => (),
	}

	// if let 写法
	if let Some(3) = v {
		println!("three");
	}
}
```

