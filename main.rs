use std::env;
use std::io::{Read, Write};
use std::fs::File;
use std::io::Result;
use std::process::Command;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Op {
    IncCell,
    DecCell,
    IncPointer,
    DecPointer,
    ReadChar,
    Write,
    BeginLoop,
    EndLoop,
}
use self::Op::*;

fn lex(input: &String) -> Vec<Op> {
    let mut operations = Vec::<Op>::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => operations.push(IncCell),
            '-' => operations.push(DecCell),
            '>' => operations.push(IncPointer),
            '<' => operations.push(DecPointer),
            ',' => operations.push(ReadChar),
            '.' => operations.push(Write),
            '[' => operations.push(BeginLoop),
            ']' => operations.push(EndLoop),
            _ => {}
        }
    }
    operations
}

fn transpile(ops: &[Op]) -> String {
    let mut out = String::from("#include \"stdio.h\"\nint main()\n{\nchar tape[20000] = {0};\nchar *ptr = tape;\n\n");
    for &operation in ops {
        match operation {
            IncCell => {
                out.push_str("++*ptr;\n");
            }
            DecCell => {
                out.push_str("--*ptr;\n");
            }
            IncPointer => {
                out.push_str("++ptr;\n");
            }
            DecPointer => {
                out.push_str("--ptr;\n");
            }
            ReadChar => {
                out.push_str("*ptr=getchar();\n");
            }
            Write => {
                out.push_str("putchar(*ptr);\n");
            }
            BeginLoop => {
                // Begin a loop at the current cell
                out.push_str("while (*ptr) {\n");
            }
            EndLoop => {
                // Close a loop
                out.push_str("}\n");
            }
        }
    }
    out.push_str("}\n");
    out
}

fn generate(filenameroot: &str, sourcecode: &String) -> std::io::Result<()> {
    let mut outfile = File::create(format!("{}.c", &filenameroot))?;
    outfile.write_all(&sourcecode.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No bf file selected.");
        std::process::exit(1);
    }
    let filename = &args[1];
    let namelen = filename.len();
    let filenameroot = &filename[..namelen-3];
    let mut sourcefile = File::open(filename).expect("No bf file selected");
    let mut source = String::new();
    sourcefile.read_to_string(&mut source).expect("failed to read");
    let operations = lex(&source);
    let c = transpile(&operations);
    generate(&filenameroot, &c);
}