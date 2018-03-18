mod expressions;
mod function;
mod statement;

use std::io;
use compile::errors::*;
use compile::Scope;

pub trait Compile {
    fn compile<O>(&self, output: &mut O, scope: &mut Scope) -> Result<()> where O: io::Write;
}


pub fn write_epilogue<O>(output: &mut O) -> io::Result<()> where O: io::Write {
    output.write_all(b"movl %ebp, %esp
pop %ebp
ret\n")
}