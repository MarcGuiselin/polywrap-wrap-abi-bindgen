use handlebars::handlebars_helper;
use serde_json::{Value};

handlebars_helper!(detect_keyword: |val: Value| {
    let type_val = val.as_str().unwrap();
    _detect_keyword(type_val)
});

pub fn _detect_keyword(type_val: &str) -> String {
    if _is_keyword(type_val) {
        return format!("_{}", type_val);
    }
    type_val.to_string()
}

fn _is_keyword(s: &str) -> bool {
    match s {
        // Python Keywords
        "False" | "None" | "True" | "and" | "as"
        | "assert" | "async" | "await" | "break" | "class"
        | "continue" | "def" | "del" | "elif" | "else"
        | "except" | "finally" | "for" | "from" | "global"
        | "if" | "import" | "in" | "is" | "lambda"
        | "nonlocal" | "not" | "or" | "pass" | "raise"
        | "return" | "try" | "while" | "with" | "yield"

        // Python Built-ins
        | "ArithmeticError" | "AssertionError" | "AttributeError" | "BaseException"
        | "BlockingIOError" | "BrokenPipeError" | "BufferError" | "BytesWarning"
        | "ChildProcessError" | "ConnectionAbortedError" | "ConnectionError"
        | "ConnectionRefusedError" | "ConnectionResetError" | "DeprecationWarning"
        | "EOFError" | "Ellipsis" | "EnvironmentError" | "Exception" | "False"
        | "FileExistsError" | "FileNotFoundError" | "FloatingPointError" | "FutureWarning"
        | "GeneratorExit" | "IOError" | "ImportError" | "ImportWarning"
        | "IndentationError" | "IndexError" | "InterruptedError" | "IsADirectoryError"
        | "KeyError" | "KeyboardInterrupt" | "LookupError" | "MemoryError"
        | "ModuleNotFoundError" | "NameError" | "None" | "NotADirectoryError"
        | "NotImplemented" | "NotImplementedError" | "OSError" | "OverflowError"
        | "PendingDeprecationWarning" | "PermissionError" | "ProcessLookupError"
        | "RecursionError" | "ReferenceError" | "ResourceWarning" | "RuntimeError"
        | "RuntimeWarning" | "StopAsyncIteration" | "StopIteration" | "SyntaxError"
        | "SyntaxWarning" | "SystemError" | "SystemExit" | "TabError" | "TimeoutError"
        | "True" | "TypeError" | "UnboundLocalError" | "UnicodeDecodeError"
        | "UnicodeEncodeError" | "UnicodeError" | "UnicodeTranslateError"
        | "UnicodeWarning" | "UserWarning" | "ValueError" | "Warning" | "ZeroDivisionError"
        | "__build_class__" | "__debug__" | "__doc__" | "__import__" | "__loader__"
        | "__name__" | "__package__" | "__spec__" | "abs" | "all" | "any" | "ascii"
        | "bin" | "bool" | "breakpoint" | "bytearray" | "bytes" | "callable" | "chr"
        | "classmethod" | "compile" | "complex" | "copyright" | "credits" | "delattr"
        | "dict" | "dir" | "divmod" | "enumerate" | "eval" | "exec" | "exit" | "filter"
        | "float" | "format" | "frozenset" | "getattr" | "globals"
        | "hasattr" | "hash" | "help" | "hex" | "id" | "input"
        | "int" | "isinstance" | "issubclass" | "iter" | "len"
        | "license" | "list" | "locals" | "map" | "max" | "memoryview"
        | "min" | "next" | "object" | "oct" | "open" | "ord"
        | "pow" | "print" | "property" | "quit" | "range" | "repr"
        | "reversed" | "round" | "set" | "setattr" | "slice"
        | "sorted" | "staticmethod" | "str" | "sum" | "super"
        | "tuple" | "type" | "vars" | "zip" => true,
        _ => false,
    }
}