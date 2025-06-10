// 第4周 - 文本解析器
// 学习目标：枚举、模式匹配、Option、Result

use std::io;

// 定义Token类型的枚举
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Operator(char),
    Identifier(String),
    Keyword(String),
    Parenthesis(char),
    String(String),
    Unknown(char),
}

// 定义解析错误类型
#[derive(Debug)]
enum ParseError {
    InvalidNumber(String),
    UnexpectedCharacter(char),
    UnterminatedString,
    EmptyInput,
}

// 文本解析器结构体
struct TextParser {
    input: String,
    position: usize,
}

impl TextParser {
    // 创建新的解析器
    fn new(input: String) -> TextParser {
        TextParser { input, position: 0 }
    }
    
    // 解析文本为Token序列
    fn parse_tokens(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();
        
        if self.input.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            
            match ch {
                // 跳过空白字符
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                }
                // 数字
                '0'..='9' | '.' => {
                    match self.parse_number() {
                        Ok(token) => tokens.push(token),
                        Err(e) => return Err(e),
                    }
                }
                // 运算符
                '+' | '-' | '*' | '/' | '%' | '^' => {
                    tokens.push(Token::Operator(ch));
                    self.advance();
                }
                // 括号
                '(' | ')' => {
                    tokens.push(Token::Parenthesis(ch));
                    self.advance();
                }
                // 字符串
                '"' => {
                    match self.parse_string() {
                        Ok(token) => tokens.push(token),
                        Err(e) => return Err(e),
                    }
                }
                // 标识符和关键字
                'a'..='z' | 'A'..='Z' | '_' => {
                    tokens.push(self.parse_identifier());
                }
                // 未知字符
                _ => {
                    tokens.push(Token::Unknown(ch));
                    self.advance();
                }
            }
        }
        
        Ok(tokens)
    }
    
    // 获取当前字符
    fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }
    
    // 前进一个字符
    fn advance(&mut self) {
        self.position += 1;
    }
    
    // 解析数字
    fn parse_number(&mut self) -> Result<Token, ParseError> {
        let start = self.position;
        let mut has_dot = false;
        
        while self.position < self.input.len() {
            match self.current_char() {
                '0'..='9' => self.advance(),
                '.' => {
                    if has_dot {
                        break; // 第二个小数点，停止解析
                    }
                    has_dot = true;
                    self.advance();
                }
                _ => break,
            }
        }
        
        let number_str = &self.input[start..self.position];
        match number_str.parse::<f64>() {
            Ok(number) => Ok(Token::Number(number)),
            Err(_) => Err(ParseError::InvalidNumber(number_str.to_string())),
        }
    }
    
    // 解析字符串
    fn parse_string(&mut self) -> Result<Token, ParseError> {
        self.advance(); // 跳过开始的引号
        let start = self.position;
        
        while self.position < self.input.len() && self.current_char() != '"' {
            self.advance();
        }
        
        if self.position >= self.input.len() {
            return Err(ParseError::UnterminatedString);
        }
        
        let string_content = self.input[start..self.position].to_string();
        self.advance(); // 跳过结束的引号
        
        Ok(Token::String(string_content))
    }
    
    // 解析标识符和关键字
    fn parse_identifier(&mut self) -> Token {
        let start = self.position;
        
        while self.position < self.input.len() {
            match self.current_char() {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => self.advance(),
                _ => break,
            }
        }
        
        let identifier = self.input[start..self.position].to_string();
        
        // 检查是否为关键字
        match identifier.as_str() {
            "if" | "else" | "while" | "for" | "fn" | "let" | "const" | "return" => {
                Token::Keyword(identifier)
            }
            _ => Token::Identifier(identifier),
        }
    }
}

// 简单计算器
struct SimpleCalculator {
    tokens: Vec<Token>,
    position: usize,
}

impl SimpleCalculator {
    fn new(tokens: Vec<Token>) -> SimpleCalculator {
        SimpleCalculator { tokens, position: 0 }
    }
    
    // 计算简单表达式 (只支持 数字 运算符 数字)
    fn calculate(&mut self) -> Result<f64, String> {
        if self.tokens.len() < 3 {
            return Err("表达式至少需要三个部分：数字 运算符 数字".to_string());
        }
        
        // 获取第一个数字
        let num1 = match &self.tokens[0] {
            Token::Number(n) => *n,
            _ => return Err("第一个token应该是数字".to_string()),
        };
        
        // 获取运算符
        let operator = match &self.tokens[1] {
            Token::Operator(op) => *op,
            _ => return Err("第二个token应该是运算符".to_string()),
        };
        
        // 获取第二个数字
        let num2 = match &self.tokens[2] {
            Token::Number(n) => *n,
            _ => return Err("第三个token应该是数字".to_string()),
        };
        
        // 执行计算
        match operator {
            '+' => Ok(num1 + num2),
            '-' => Ok(num1 - num2),
            '*' => Ok(num1 * num2),
            '/' => {
                if num2 == 0.0 {
                    Err("除数不能为零".to_string())
                } else {
                    Ok(num1 / num2)
                }
            }
            '%' => Ok(num1 % num2),
            '^' => Ok(num1.powf(num2)),
            _ => Err(format!("不支持的运算符: {}", operator)),
        }
    }
}

fn main() {
    println!("📝 欢迎使用Rust文本解析器和计算器！");
    
    loop {
        show_menu();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("读取输入失败");
        
        match input.trim() {
            "1" => parse_text_demo(),
            "2" => simple_calculator_demo(),
            "3" => token_analysis_demo(),
            "4" => {
                println!("再见！👋");
                break;
            }
            _ => println!("❌ 无效选择，请重新输入！"),
        }
    }
}

fn show_menu() {
    println!("\n=== 文本解析器菜单 ===");
    println!("1. 文本解析演示");
    println!("2. 简单计算器");
    println!("3. Token分析");
    println!("4. 退出");
    println!("请选择操作 (1-4):");
}

fn parse_text_demo() {
    println!("请输入要解析的文本:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");
    
    let mut parser = TextParser::new(input.trim().to_string());
    
    match parser.parse_tokens() {
        Ok(tokens) => {
            println!("✅ 解析成功！找到 {} 个token:", tokens.len());
            for (i, token) in tokens.iter().enumerate() {
                match token {
                    Token::Number(n) => println!("{}: 数字 -> {}", i + 1, n),
                    Token::Operator(op) => println!("{}: 运算符 -> {}", i + 1, op),
                    Token::Identifier(id) => println!("{}: 标识符 -> {}", i + 1, id),
                    Token::Keyword(kw) => println!("{}: 关键字 -> {}", i + 1, kw),
                    Token::Parenthesis(p) => println!("{}: 括号 -> {}", i + 1, p),
                    Token::String(s) => println!("{}: 字符串 -> \"{}\"", i + 1, s),
                    Token::Unknown(c) => println!("{}: 未知字符 -> {}", i + 1, c),
                }
            }
        }
        Err(e) => {
            println!("❌ 解析失败:");
            match e {
                ParseError::InvalidNumber(s) => println!("无效数字: {}", s),
                ParseError::UnexpectedCharacter(c) => println!("意外字符: {}", c),
                ParseError::UnterminatedString => println!("未闭合的字符串"),
                ParseError::EmptyInput => println!("输入为空"),
            }
        }
    }
}

fn simple_calculator_demo() {
    println!("简单计算器 (格式: 数字 运算符 数字)");
    println!("支持的运算符: +, -, *, /, %, ^");
    println!("请输入表达式:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");
    
    let mut parser = TextParser::new(input.trim().to_string());
    
    match parser.parse_tokens() {
        Ok(tokens) => {
            let mut calculator = SimpleCalculator::new(tokens);
            match calculator.calculate() {
                Ok(result) => println!("✅ 计算结果: {}", result),
                Err(e) => println!("❌ 计算错误: {}", e),
            }
        }
        Err(e) => println!("❌ 解析错误: {:?}", e),
    }
}

fn token_analysis_demo() {
    let test_cases = vec![
        "123.45 + 67.89",
        "hello world",
        "if let x = 42",
        "\"这是一个字符串\"",
        "(a + b) * c",
        "unknown@#$",
    ];
    
    println!("📊 Token分析演示:");
    
    for (i, test) in test_cases.iter().enumerate() {
        println!("\n--- 测试案例 {} ---", i + 1);
        println!("输入: {}", test);
        
        let mut parser = TextParser::new(test.to_string());
        match parser.parse_tokens() {
            Ok(tokens) => {
                print!("Token: ");
                for token in tokens {
                    match token {
                        Token::Number(n) => print!("[数字:{}] ", n),
                        Token::Operator(op) => print!("[运算符:{}] ", op),
                        Token::Identifier(id) => print!("[标识符:{}] ", id),
                        Token::Keyword(kw) => print!("[关键字:{}] ", kw),
                        Token::Parenthesis(p) => print!("[括号:{}] ", p),
                        Token::String(s) => print!("[字符串:\"{}\"] ", s),
                        Token::Unknown(c) => print!("[未知:{}] ", c),
                    }
                }
                println!();
            }
            Err(e) => println!("解析失败: {:?}", e),
        }
    }
}

// 练习任务：
// 1. 添加更多数学函数支持 (sin, cos, sqrt等)
// 2. 支持变量赋值和使用
// 3. 支持复杂表达式解析
// 4. 添加语法高亮功能
