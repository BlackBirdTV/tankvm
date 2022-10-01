// +----------------------------------------+
// |       TankVM Luis Weinzierl 2022       |
// +----------------------------------------+

use std::io::{Write, Read};

use terminal_keycode::{Decoder};
use crossterm::terminal;

use std::fs::File;

#[derive(Debug, Clone, PartialEq)]
enum Var {
    Str(String),
    Num(f64),
    Bool(bool),
    Ptr(usize),
    Byte(u8),
    Isize(isize),
    Usize(usize),
}

impl Var {
    fn get(&self, vars: &Vec<Var>) -> Self {
        match self {
            Var::Ptr(addr) => vars[*addr].get(vars).to_owned(),
            _ => self.to_owned()
        }
    }

    fn to_string(&self) -> String {
        match self {
            Var::Str(string) => string.to_owned(),
            Var::Num(number) => number.to_string(),
            Var::Bool(boolean) => boolean.to_string(),
            Var::Byte(byte) => byte.to_string(),
            _ => String::new()
        }
    }

    fn as_num(&self) -> f64 {
        match self {
            Var::Num(i) => *i,
            _ => panic!()
        }
    }

    fn as_usize(&self) -> usize {
        match self {
            Var::Usize(i) => *i,
            _ => panic!()
        }
    }

    fn as_isize(&self) -> isize {
        match self {
            Var::Isize(i) => *i,
            _ => panic!()
        }
    }

    fn as_pointer(&self) -> usize {
        match self {
            Var::Ptr(i) => *i,
            Var::Usize(i) => *i,
            _ => panic!()
        }
    }
}

#[derive(Clone, Debug)]
struct Inst {
    opcode: u8,
    args: Vec<Var>
}

fn main() {
    terminal::enable_raw_mode().unwrap();
    let mut stdin = std::io::stdin();
    let mut buf = vec![0];
    let mut decoder = Decoder::new();

    let mut vars = Vec::new();
    let file = File::open("test.tvm").expect("File System Error");
    let insts = to_insts(file);
    let mut i = 0;
    loop {
        if i >= insts.len() {
            break;
        }
        let inst = insts[i].clone();

        /*  
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | Opcode            | Args              | Description                                         |                
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 0                 | {any}             | Add a variable                                      |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 1                 | {ptr}{any}        | Set a variable                                      |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 2                 | {ptr}{num}        | Add to a variable                                   |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 3                 | {ptr}{num}        | Subtract from a variable                            |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 4                 | {ptr}{num}        | Multiply a variable                                 |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 5                 | {ptr}{num}        | Divide a variable                                   |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 6                 | {num}             | Go to Line                                          |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 7                 | {any}{any}{num}   | Jump if equal                                       |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 8                 | {any}{any}{num}   | Jump if not equal                                   |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 9                 | {num}{num}{num}   | Jump if greater                                     |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 10                | {num}{num}{num}   | Jump if less                                        |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 11                | {ptr}{uint}       | Add uint to ptr                                     |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 12                | {ptr}{uint}       | Remove uint from ptr                                |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 13                | {ptr}{uint}       | Multiply ptr by uint                                |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 14                | {ptr}{uint}       | Divide ptr by uint                                  |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 253               | {ptr}             | Read keycode from stdin and store it at the next 5  |
         *  |                   |                   | bytes like this: {len}{1}{2}{3}{4}                  |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 254               | {any}             | Print a variable                                    |
         *  +-------------------+-------------------+-----------------------------------------------------+
         *  | 255               |                   | Flush stdout                                        |
         *  +-------------------+-------------------+-----------------------------------------------------+
         */


        match inst.opcode {
            0 => vars.push(inst.args[0].get(&vars)),
            1 => {
                let i = inst.args[0].as_pointer();
                vars[i] = inst.args[1].get(&vars)
            }
            2 => {
                let i = inst.args[0].as_pointer();
                vars[i] = Var::Num(vars[i].get(&vars).as_num() + inst.args[1].get(&vars).as_num());
            }
            3 => {
                let i = inst.args[0].as_pointer();
                vars[i] = Var::Num(vars[i].get(&vars).as_num() - inst.args[1].get(&vars).as_num());
            }
            4 => {
                let i = inst.args[0].as_pointer();
                vars[i] = Var::Num(vars[i].get(&vars).as_num() * inst.args[1].get(&vars).as_num());
            }
            5 => {
                let i = inst.args[0].as_pointer();
                vars[i] = Var::Num(vars[i].get(&vars).as_num() / inst.args[1].get(&vars).as_num());
            },
            6 => {
                i = inst.args[0].as_usize()
            },
            7 => {
                let a = inst.args[0].get(&vars);
                let b = inst.args[1].get(&vars);
                if a == b {
                    i = inst.args[2].as_usize();
                }
            },
            8 => {
                let a = inst.args[0].get(&vars);
                let b = inst.args[1].get(&vars);
                if a != b {
                    i = inst.args[2].as_usize();
                }
            },
            9 => {
                let a = inst.args[0].get(&vars).as_num();
                let b = inst.args[1].get(&vars).as_num();
                if a > b {
                    i = inst.args[2].as_usize();
                }
            },
            10 => {
                let a = inst.args[0].get(&vars).as_num();
                let b = inst.args[1].get(&vars).as_num();
                if a < b {
                    i = inst.args[2].as_usize();
                }
            },
            253 => {
                let byte_ptr = inst.args[0].as_pointer();
                let mut buf: [u8; 4] = [0; 4];
                vars[byte_ptr] = Var::Isize(unsafe { libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, 4 as libc::size_t) });
                for (i, b) in buf.iter().enumerate() {
                    vars[byte_ptr + 1 + i] = Var::Byte(*b);
                }
            }
            254 => {
                print!("{}", inst.args[0].get(&vars).to_string())
            }
            255 => {
                std::io::stdout().flush().expect("Stdout Error");
            }
            _ => ()
        }
        i += 1;
    }
    terminal::disable_raw_mode().unwrap();
}

fn to_insts(inp: File) -> Vec<Inst> {
    let mut ret = Vec::new();
    let mut opcode = 0;
    let mut args = Vec::new();
    let mut bytes_left = None;
    let mut var_type = None;
    let mut var_buf = Vec::new();

    let mut first_char = true;
    for byte in inp.bytes() {
        // println!("before {}: {} | {:?} | {:?} | {:?} | {:?} | {}", byte, opcode, var_buf, args, var_type, bytes_left, first_char);
        let byte = byte.unwrap();
        if let Some(i) = bytes_left && i == 0 {
            let lcl_type = if let Some (res) = var_type {res} else {panic!()};
            args.push(match lcl_type {
                1 => Var::Str(String::from_utf8(var_buf.clone()).unwrap()),
                2 => Var::Num(f64::from_be_bytes([var_buf[0], var_buf[1], var_buf[2], var_buf[3], var_buf[4], var_buf[5], var_buf[6], var_buf[7]])),
                3 => Var::Bool(var_buf[0] == 1),
                4 => Var::Ptr(usize::from_be_bytes([var_buf[0], var_buf[1], var_buf[2], var_buf[3], var_buf[4], var_buf[5], var_buf[6], var_buf[7]])),
                5 => Var::Byte(var_buf[0]),
                6 => Var::Isize(isize::from_be_bytes([var_buf[0], var_buf[1], var_buf[2], var_buf[3], var_buf[4], var_buf[5], var_buf[6], var_buf[7]])),
                _ => Var::Usize(usize::from_be_bytes([var_buf[0], var_buf[1], var_buf[2], var_buf[3], var_buf[4], var_buf[5], var_buf[6], var_buf[7]]))
            });
            var_type = None;
            var_buf = Vec::new();
            bytes_left = None;
            if byte == 0 {
                ret.push(Inst {
                    opcode,
                    args: args.clone()
                });
                // println!("{:?}", ret);
                opcode = 0;
                first_char = true;
                args = Vec::new();
                continue;
            }
        }
        if first_char {
            opcode = byte;
            first_char = false;
        }
        else if let Some(i) = bytes_left && i > 0 {
            var_buf.push(byte);
            bytes_left = Some(i - 1);
        }
        else if var_type == None {
            var_type = Some(byte);
        }
        else if bytes_left == None {
            bytes_left = Some(byte);
        }
        // println!("after  {}: {} | {:?} | {:?} | {:?} | {:?} | {}", byte, opcode, var_buf, args, var_type, bytes_left, first_char);
    }

    ret
}