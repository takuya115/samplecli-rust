use clap::Clap;
use core::panic;
use std::fs::File;
use std::io::{ BufRead, BufReader, stdin };

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Takuya",
    about = "Super awesome sample RPN caculator"
)]

struct Opts {
    #[clap(short, long)]
    verbose: bool,
    #[clap(name = "FILE")]
    formula_file: Option<String>
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        
        run(reader, opts.verbose);
    } else {
        // ファイルを指定しなかった場合
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    let calc = RpnCalcurator::new(verbose);
    
    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

// 構造体定義
struct RpnCalcurator(bool);

impl RpnCalcurator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            // i32型に変換できる(=Ok(x)がtrueの場合)
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                // i32型に変換できない場合 = 記号 or 異常な値
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                
                // 演算子が取得できた場合は該当する演算を実施、値をresに入れる
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "%" => x % y,
                    _ => panic!("invalid token"),
                };
                stack.push(res);
            }
            
            // "-v"オプションが指定されている場合は、この時点でトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax")
        }
    }
}
