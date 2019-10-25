use rapidus::{
    parser::{Error as ParserError, Parser},
    vm::{codegen::Error as CodegenError, error::RuntimeError, jsvalue::value::Value, vm::VM},
};
use std::option::NoneError;

pub type ScriptResult = Result<Value, ScriptError>;

#[derive(Debug)]
pub enum ScriptError {
    ParserError(ParserError),
    CodegenError(CodegenError),
    RuntimeError(RuntimeError),
    StackEmpty,
}

impl From<ParserError> for ScriptError {
    fn from(from: ParserError) -> Self {
        ScriptError::ParserError(from)
    }
}
impl From<CodegenError> for ScriptError {
    fn from(from: CodegenError) -> Self {
        ScriptError::CodegenError(from)
    }
}
impl From<RuntimeError> for ScriptError {
    fn from(from: RuntimeError) -> Self {
        ScriptError::RuntimeError(from)
    }
}
impl From<NoneError> for ScriptError {
    fn from(from: NoneError) -> Self {
        ScriptError::StackEmpty
    }
}

pub struct Isolate {
    vm: VM,
}

impl Isolate {
    pub fn new() -> Self {
        Isolate { vm: VM::new() }
    }

    pub fn execute_script(&mut self, src: &str) -> ScriptResult {
        let mut parser = Parser::new("test", src.to_owned() + ";undefined;");
        let node = parser.parse_all()?;

        let func_info = self.vm.compile(&node, true)?;
        self.vm.run_global(func_info)?;

        let val: Value = self.vm
            .current_context
            .stack
            .pop()
            .unwrap_or(Value::undefined().into())
            .into();

        Ok(val)
    }

    pub fn value(&self, name: &str) -> Value {
        self.vm.current_context.this.get_property(name)
    }
}
