# 编写自动化测试

## 编写和运行测试

测试也是函数，用于检测其他代码是否与预期一致。测试通常被用来：
- 准备（arange）数据、状态
- 运行（acet）被测试的代码
- 断言（assert）结果

测试函数需要使用 test 属性（attribute）进行标注：
- Attribute 是一段 rust 代码的元数据
- 在函数上加入 #[test] 就可以把代码变为测试函数

### 运行测试

cargo test

出现 panic 则测试失败

## 断言

- 使用 assert 判断
- 使用 assert_eq! 和 assert_ne! 测试相等性
  - 以上三个宏都可以在最后一个参数加入自定义信息
- 使用 should_panic 属性检查发生了恐慌

```rust
assert(xxx != xx, "new message");
assert_eq!("aaa", "bbb", "aaa is not eq bbb");
#[should_panic]
// 其他地方抛出的恐慌必须包含 less panic information 字符串，否则就不会通过
#[should_panic(expected = "less panic information")]
```

## 使用 Result<T, E>

- 返回 Ok(())，成功；返回 Err(err), 失败。
- 不要与 #[should_panic] 混用

```rust
#[cfg(test)]

mod tests {
	// 这个模块叫 tests，但可以放正常的代码

	// 这里才是真正的测试代码
	#[test]
	fn it_works() -> Result<(), String> {
		if 2 + 2 == 4 {
			Ok(())
		} else {
			Err(String::from("two plus two does not equal four"))
		}
	}
}
```

## 控制测试的运行方式

cargo test 不加参数，执行的是默认行为。

- 默认行为：并行执行所有测，捕获（不显示输出），读取与测试相关的输出更容易。
- 命令行参数：
  - cargo test --help，所有参数
  - cargo test -- --help，所有可以放在两个 - 之后的参数

举例： cargo test -- test-threads=1 以单线程顺序执行所有测试

### 显示函数的输出

- 运行成功，标准输出会被捕获；
- 运行失败，标准输出会被打印；

### 按名称运行测试子集
- 单个测试直接指定测试名
- 多个测试指定测试名的一部分，比如公共的开头（模块名也可）

### 忽略某些测试

- 增加 #\[ignore\] 属性，可以标记被忽略的测试
- 执行 cargo test -- --ignored 可以单独运行被忽略的测试

### 测试的组织

测试分为单元测试和集成测试。

单元测试专注于功能点，一次对一个模块进行隔离测试；集成测试在库外部，只能测试 public 接口，可能在测试中使用多个模块。

### 单元测试
- 使用 cfg：configuration（配置）标记
- 告诉 rust 只有在指定配置下才编译和运行
- 例如 #[cfg(test)] 只有在 cargo test 下才会编译运行

### 集成测试
- 创建集成测试 tests 目录
- tests 目录下的每个测试文件都是一个单独的 crate
  - 需要导入被测试crate
- 单独运行测试文件 cargo test --test 文件名

## 针对 binary crate 的集成测试
如果项目只包含 src/main.rs 没有 src/lib.rs，则不能在 tests 目录下建立集成测试。