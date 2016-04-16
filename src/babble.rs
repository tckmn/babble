extern crate num;
use self::num::rational::BigRational as Rational;
use self::num::bigint::BigInt;
use self::num::traits::ToPrimitive;

// adapted from @Shepmaster's code here:
// http://stackoverflow.com/a/27590832/1223693
use std::io::prelude::*;
macro_rules! warn(
    ($($arg:tt)*) => { {
        write!(&mut ::std::io::stderr(), "warn: ").unwrap();
        writeln!(&mut ::std::io::stderr(), $($arg)*).unwrap();
    } }
);

macro_rules! rint(
    ($x:expr) => (
        Rational::from_integer(BigInt::from($x))
    )
);
macro_rules! rfloat(
    ($x:expr) => (
        Rational::from_float($x).unwrap()
    )
);

pub struct Babble {
    primary: usize, secondary: usize, result: usize, vars: [Value; 26]
}

#[derive(Clone)]
enum Value {
    Num(Rational), Arr(Vec<Value>), Block(String)
}
impl Value {
    fn num(f: f64) -> Value {
        Value::Num(rfloat!(f))
    }
}

impl Babble {
    pub fn new() -> Babble {
        Babble {
            primary: 0, secondary: 1, result: 2,
            vars: [Value::num(0.0), Value::num(0.0), Value::num(0.0),
                   Value::num(0.0), Value::num(0.0), Value::num(0.0),
                   Value::num(0.0), Value::num(0.0), Value::num(0.0),
                   Value::num(0.0), Value::num(0.0), Value::num(0.0),
                   Value::num(0.0), Value::num(0.0), Value::num(0.0),
                   Value::num(0.0), Value::num(0.0), Value::num(0.0),
                   Value::num(0.0), Value::num(0.0), Value::num(0.0),
                   Value::num(0.0), Value::num(0.0), Value::num(0.0),
                   Value::num(0.0), Value::num(0.0)]
        }
    }

    pub fn run(&mut self, code: String) {
        let (mut stdout, mut stdin) = (::std::io::stdout(), ::std::io::stdin());
        self.run_with_io(code, &mut stdout, &mut stdin);
    }

    pub fn run_with_io(&mut self, code: String,
                       stdout: &mut Write, stdin: &mut Read) {
        for token in Babble::tokenize(code) {
            token(self, stdout, stdin)
        }
    }

    fn tokenize(code: String) -> Vec<Box<Fn(&mut Babble, &mut Write, &mut Read)>> {
        let mut tokens = Vec::new();
        let mut code_iter = BabbleCodeIterator::new(code);
        while let Some(token) = Babble::parse(&mut code_iter) {
            tokens.push(token);
        }
        tokens
    }

    fn parse(mut code: &mut BabbleCodeIterator)
            -> Option<Box<Fn(&mut Babble, &mut Write, &mut Read)>> {
        // this is the top-level parsing function. In normal parsing mode, we
        // simply grab three letters and go from there
        let cmd: String = code.take(3).collect();

        // check for primary, secondary, or result variable setting commands
        if cmd.starts_with("PV") {
            let pv = Babble::letter_idx(cmd.chars().last().unwrap());
            return Some(box move |this, _, _| {
                this.primary = pv;
            });
        } else if cmd.starts_with("SV") {
            let sv = Babble::letter_idx(cmd.chars().last().unwrap());
            return Some(box move |this, _, _| {
                this.secondary = sv;
            });
        } else if cmd.starts_with("RV") {
            let rv = Babble::letter_idx(cmd.chars().last().unwrap());
            return Some(box move |this, _, _| {
                this.result = rv;
            });
        }

        match &cmd[..] {
            // literals .......................................................

            // array / string literal
            "ARR" => Babble::parse_literal_array(&mut code),

            // small number literals
            "ZRO" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(0.0)
            }),
            "ONE" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(1.0)
            }),
            "TWO" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(2.0)
            }),
            "TRE" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(3.0)
            }),
            "FOR" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(4.0)
            }),
            "FIV" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(5.0)
            }),
            "SIX" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(6.0)
            }),
            "SVN" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(7.0)
            }),
            "EGT" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(8.0)
            }),
            "NIN" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(9.0)
            }),
            "TEN" => Some(box |this, _, _| {
                this.vars[this.primary] = Value::num(10.0)
            }),

            // math ...........................................................

            // basic arithmetic
            "ADD" => Some(box |this, _, _| {
                this.vars[this.result] = Value::Num(
                    match this.vars[this.primary] {
                        Value::Num(ref n) => n.clone(),
                        _ => rint!(0)
                    } + match this.vars[this.secondary] {
                        Value::Num(ref n) => n.clone(),
                        _ => rint!(0)
                    });
            }),
            "SUB" => Some(box |this, _, _| {
                this.vars[this.result] = Value::Num(
                    match this.vars[this.primary] {
                        Value::Num(ref n) => n.clone(),
                        _ => rint!(0)
                    } - match this.vars[this.secondary] {
                        Value::Num(ref n) => n.clone(),
                        _ => rint!(0)
                    });
            }),

            // I/O ............................................................

            // stdin/stdout
            "PUT" => Some(box |this, stdout, _| {
                match this.vars[this.primary] {
                    Value::Num(ref n) => print!("{}", n),
                    Value::Arr(ref a) => for v in a { match v {
                        &Value::Num(ref n) => {
                            let mut val = n.clone();
                            while val != rint!(0) {
                                let byte = (val.clone() % rint!(256))
                                    .to_integer().to_u8().unwrap();
                                stdout.write(&[byte]).unwrap();
                                val = (val / rint!(256)).floor();
                            }
                        },
                        _ => warn!("PUT called on array with ignored \
                                   non-Num element")
                    } },
                    Value::Block(_) => warn!("PUT called on block ignored")
                }
            }),

            // if we run out of chars or if the function is unknown, ignore

            _ => None
        }
    }

    fn parse_literal_array(mut code: &mut BabbleCodeIterator)
            -> Option<Box<Fn(&mut Babble, &mut Write, &mut Read)>> {
        let mut arr: Vec<Value> = Vec::new();

        'loopy: loop {
            let ch = if let Some(x) = code.next() { x } else { return None };
            if ch == 'Z' {
                match if let Some(x) = code.next() { x } else { return None } {
                    // ZE: [e]nd array literal
                    'E' => break 'loopy,
                    // ZL; [l]owercase letter
                    'L' => {
                        let ch3 = if let Some(x) = code.next() { x }
                                  else { return None };
                        arr.push(Value::num(ch3 as u8 as f64 + 32.0));
                    },
                    // ZT: [t]wo base 25 digits
                    'T' => {
                        let ch3 = if let Some(x) = code.next() { x }
                                  else { return None };
                        let ch4 = if let Some(x) = code.next() { x }
                                  else { return None };
                        arr.push(Value::num((Babble::letter_idx(ch3) * 25 +
                                             Babble::letter_idx(ch4)) as f64));
                    },
                    // ZZ: a literal Z character
                    'Z' => arr.push(Value::num('Z' as u8 as f64)),
                    _ => {}
                }
            } else {
                arr.push(Value::num(ch as u8 as f64));
            }
        }

        Some(box move |this, _, _| {
            this.vars[this.primary] = Value::Arr(arr.to_owned());
        })
    }

    // convert an uppercase character to its index in the alphabet, 0-indexed
    // A=0, B=1, etc.
    fn letter_idx(ch: char) -> usize {
        (ch as usize) - 65
    }
}

struct BabbleCodeIterator { code: Vec<char> }
impl BabbleCodeIterator {
    fn new(code: String) -> BabbleCodeIterator {
        BabbleCodeIterator {
            code: code.chars().rev().filter(|x| x.is_uppercase()).collect()
        }
    }
}
impl Iterator for BabbleCodeIterator {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.code.pop()
    }
}
