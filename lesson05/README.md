# 第五课

请搜索相关文档，实现：一个简单的声明宏并理解其代码结构，和编译过程。

---

运行代码

```sh
cargo run --bin lesson05
```

找了个场景：Excel 基本函数

sum：跟普遍使用的例子基本一致的，加减乘除都可以，貌似问题不大；

if：关键词冲突，有特殊语法；

xlookup：有可选参数、有默认值，不是很好处理，更比较适合用结构体配合字段 Option 来处理。



代码结构：

- 考虑到简单，目前尝试的例子都用的 expr，还没有用其它标记。
- 声明式宏是基于模式匹配，一般用于适合有几种写法的场景，例如：
  - 空列表 vec![]
  - 带初始值和重复长度的列表 vec![23;10]
  - 指定值 vec![1,2,3]
- 目前理解到有一个很有优势的场景，一般函数搞不定：参数数量不固定，但是可能类型基本一致，只是不断重复
- 会被提前展开为 Rust 代码再进行编译，所以没有性能损耗
- 但是可读性和代码维护难度大幅度提升，绝对不适合用作复杂业务逻辑，仅限于特定场景
- 也不好编辑和调试，仅仅比写纯文本好非常少的一些些而已



结构大概理解为下面注释这样：

```rust

/// =XLOOKUP(lookup_value, lookup_array, return_array, [if_not_found], [match_mode], [search_mode])
macro_rules! xlookup { // 这里是宏名称的声明，用的时候就是 xlookup! 也就是加了一个感叹号
    ($v:expr,$la:expr,$ra:expr,$d:expr,$mm:expr,$sm:expr) => {  // 这是其中一个声明模式，例如这就是 (v, la, ra ,d, mm, sm)，如果改其它形式的括号也是可以的比如 [] {} 再其它的符号试过了，没发现更多的符号支持
        {
            // todo mm sm
            let mut res = $d;  // 参数声明前面要带$，用的时候也要带$
            for (idx, item) in $la.iter().enumerate() {  // 其它时候跟写普通代码一样，但编辑器就是不会有任何类型提示
                if $v == *item {
                    res = $ra[idx];
                    break;
                }
            };
            res
        }
    };
  () => {};  // 写完一个模式，有需要的话，还可以继续声明其它模式，所以非常适合用来支持多种写法的那种情况
  // 但是不适合这里这个函数的意义，d mm sm 都是可选参数，以及逗号隔开单独给sm参数也是可以的，所以我感觉并不适合模式匹配
}
```



编译过程：

TokenStream 是统一的对接口，其实就是编译器对代码文本的统一抽象类型建模，先要认得它们。

声明宏 => TokenStream(宏层面) => expand(宏展开) => TokenStream(Rust代码层面)

相比于普通 Rust 代码：Rust 代码 => TokenStream(Rust代码层面)

前面多了一层宏展开过程，外加宏层面的TokenStream。

本质是一种 TokenStream => 到另外一种 TokenStream 的处理（词法分析级别）。

声明宏的展开可以理解为“模式匹配”，类似于正则匹配的这种类比概念。

有张图这个人说的比较清晰了：https://www.cnblogs.com/gaozejie/p/16950786.html