# 第03章 编程通用知识

## 3.1 变量与可变性

### 变量
- 声明变量使用 let 关键字
- 默认情况下，变量是不可变的（Imuutable）
- 声明变量时，变量前加入 mut 关键字，可以修改变量

### 常量
- 常量不可以使用 mut，且永远不可变
- 常量使用 const 关键字，且类型被标注
- 常量可以在任何作用域内声明，包括全局作用域
- 常量可以背绑定到表达式常量，无法绑定到函数的调用结果或者运行时才能计算出的值
- 常量命名规范，全大写字母，单词之间下划线隔开

举例：const MAX_POINTS: u32 = 100_000; 数字间的下划线是为了增加可读性

### 隐藏

隐藏，又叫 shadowing

- 可以使用相同名字声明新的变量，新的变量就会 shadow 之前声明的同名变量；
- 使用 let 声明同名变量，它的类型可与之前不同

```rust
fn test_shadowing() {
    let spaces = "    "; // 这个是 string 类型
    let spaces = spaces.len(); // 这个是 uint 类型
}
```

## 3.2 数据类型

### 标量与复合类型
- rust 编译器在编译时需要知道变量的类型，一般可以自行推导
- rust 编译器要求在可能存在多种类型时，必须添加标注

```rust
fn main() {
    let gusess: u32 = "32".parse().expect("Not a number");
    println!("{}", guess);
}
```

### 标量类型
- 整数类型：推荐 i32，i8 这种自带类型
  - 在调试模式（debug）下，整形溢出会报错
  - 在发布模式（release）下，整形溢出会环绕
  - 根据机器位数的 iszie，usize
- 浮点类型：f32（单精度），f64（双精度，默认）
- 布尔类型：true，false，占用1个字节大小
- 字符类型：char 类型
  - 字面值使用单引号
  - 占用 4 个字节
  - Unicode 标量值，范围 U+0000 到 U+D7FF，U+E000 到 U+10FFFF

## 3.3 复合类型

rust 提供了两种基础复合类型，元组（Tuple）和数组（Array），将多个值放到一个类型

### 元组

元组可以使用多个值，一旦创建不可修改，可以被解构（destructure）

```rust
fn main() {
    // create tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructure
    let (x, y, z) = tup;
    // get value in tuple
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
}
```

### 数组

- 数据存放在栈（stack）上，而不是堆（heap）上；
- 数组没有 Vector 灵活，Vector 长度可变；
- 数组的两种声明方法 [类型; 长度] 和 [首元素; 长度]；
- 数组索引超出，编译会通过，运行会报错

```rust
// declare1
let a [i32; 5] = {1, 2, 3, 4, 5};
// declar2
let a [3; 5]; 相当于 {3, 3, 3, 3, 3};
```

## 3.4 函数

- 关键字为 fn
- 命名惯例是 snake_case 风格
- 函数的参数：形参 parameters和实参 arguments，形参必须声明每个参数类型
- 函数的语句和表达式
  - 函数体由一系列语句组成，可选的由一个表达式结束；
  - 表达式会计算产生一个值，返回一个值
  - 语句是执行一些动作的指令，返回一个 tuple
  - 如果实在分不清楚，可以显示编写 return 增加可读性
- 注释与 C 语言相同

```rust
fn main() {
    println!("hello, world");
    another_function(4, 5);
}

fn another_function(x: i32, y: i32) -> i32 {
    let z = {
        let a = 1;
        a + 1
    };
    println!("another function, x: {}", x);
    println!("another function, y: {}", y);
    println!("another function, z: {}", z);

x + y + z
}
```

## 3.5 控制流

### 分支

if 表达式

- if 表达式允许根据条件执行不同的分支，条件必须是 bool
- if 表达式中，与条件相关的代码块就叫做分支（arm）
- 可选的，在后面加上一个 else / else if 表达式


```rust
fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }
}
```

多个 else if 可以用 match 重构

### 循环

loop，while，for

- loop：break 退出
- while：条件循环
- for：条件循环，推荐使用

```rust
fn main() {
    // 3, 2, 1
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
```