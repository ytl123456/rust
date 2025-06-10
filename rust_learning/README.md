# 🦀 Rust学习之旅

欢迎来到Rust学习项目！这个项目包含了8周的系统性Rust学习内容，从基础语法到高级特性。

## 📚 学习计划概览

### 第1周：环境搭建与基础概念
- **目标**：熟悉Rust开发环境，掌握基本语法
- **项目**：
  - `hello_world/` - 增强版Hello World程序
  - `calculator/` - 简单计算器
- **核心概念**：变量、数据类型、函数、控制流

### 第2周：控制流与所有权入门
- **目标**：理解Rust独特的所有权系统
- **项目**：
  - `guessing_game/` - 猜数字游戏
  - `todo_list/` - 简单待办事项列表
- **核心概念**：所有权、引用与借用、Vec操作

### 第3周：所有权深入与数据结构
- **目标**：深入理解所有权，掌握结构体
- **项目**：
  - `student_manager/` - 学生成绩管理系统
- **核心概念**：结构体、方法、关联函数

### 第4周：枚举与模式匹配
- **目标**：掌握Rust强大的模式匹配功能
- **项目**：
  - `text_parser/` - 文本解析器
- **核心概念**：枚举、match、Option、Result

### 第5-8周：高级特性
- **包管理与模块系统**
- **集合与错误处理**
- **泛型与trait**
- **生命周期与智能指针**

## 🚀 快速开始

### 1. 环境准备
```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version
```

### 2. 运行项目
```bash
# 进入第1周目录
cd week1/hello_world

# 运行项目
cargo run

# 或者先构建再运行
cargo build
./target/debug/hello_world
```

## 📖 学习方法建议

### 每日学习流程
1. **理论学习** (30-45分钟)
   - 阅读相关章节
   - 理解核心概念

2. **代码实践** (45-60分钟)
   - 运行示例代码
   - 修改和实验

3. **项目练习** (30-45分钟)
   - 完成项目要求
   - 尝试扩展功能

4. **总结复习** (15分钟)
   - 记录学习心得
   - 整理遇到的问题

## 🎯 阶段性目标检验

### 第2周检验点
- 能独立完成猜数字游戏
- 理解基本的所有权规则

### 第4周检验点
- 能使用match处理复杂逻辑
- 熟练使用Option和Result

## 🛠️ 推荐工具

### 开发环境
- **VS Code** + rust-analyzer插件
- **CLion** + Rust插件

### 学习资源
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

## 🔗 有用链接

- [Rust官网](https://www.rust-lang.org/)
- [Rust用户论坛](https://users.rust-lang.org/)
- [Rust中文社区](https://rust.cc/)

---

祝你在Rust学习之旅中取得成功！记住：Rust的学习曲线虽然陡峭，但一旦掌握，你会体验到系统编程的乐趣和安全性的保障。🚀 