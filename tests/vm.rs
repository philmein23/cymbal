extern crate cymbal;

#[cfg(test)]
mod vm_tests {
    use cymbal::lexer::Lexer;
    use cymbal::parser::Parser;
    use cymbal::compiler::Compiler;
    use cymbal::vm::Vm;

    #[test]
    fn integer() {
        test_vm(vec![
            ("1", "1"),
            ("2", "2"),
            ("1 + 2", "3"),
        ]);
    }

    fn test_vm(tests: Vec<(&str, &str)>) {
        for (input, expected) in tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            let mut compiler = Compiler::new();
            match compiler.compile(&program) {
                Err(err) => {
                    panic!("error on compile for `{}`: {}", input, err);
                }
                _ => {}
            }
            let bytecode = compiler.bytecode();
            let mut vm = Vm::new(bytecode);
            match vm.run() {
                Err(err) => {
                    panic!("error on vm for `{}`: {}", input, err);
                }
                _ => {}
            }
            if let Some(obj) = vm.last_popped_stack_elem() {
                assert_eq!(&obj.to_string(), expected, "for `{}` {:?}", input, vm);
            } else {
                panic!("no stack top on vm for `{} {:?}`", input, vm);
            }
        }
    }
}
