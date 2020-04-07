use wabt;
use wasmi::*;

fn main() {
    let wasm_binary_import = wabt::wat2wasm(
        r#"
        (module
            (memory (import "env" "memory") 17)

            (func (export "get_first_i32") (result i32)
                (i32.load (i32.const 0))
            )
        )
        "#,
    )
    .expect("failed to parse wat 2");

    {
        let module =
            wasmi::Module::from_buffer(&wasm_binary_import).expect("failed to load wasm 1");

        let imports = ImportsBuilder::new().with_resolver("env", &ResolveAll {});
        let instance = ModuleInstance::new(&module, &imports)
            .expect("failed to instantiate wasm module 2")
            .assert_no_start();
        println!(
            "import got {:?} wanted {:?}",
            instance
                .invoke_export("get_first_i32", &[], &mut wasmi::NopExternals)
                .expect("failed to execute get_first_i32 2"),
            Some(RuntimeValue::I32(1234))
        );
    }

    let wasm_binary_export = wabt::wat2wasm(
        r#"
        (module
            (memory (;0;) 17)
            (export "memory" (memory 0))

            (func (export "get_first_i32") (result i32)
                (i32.load (i32.const 0))
            )
        )
        "#,
    )
    .expect("failed to parse wat 1");

    {
        let module =
            wasmi::Module::from_buffer(&wasm_binary_export).expect("failed to load wasm 1");

        let imports = ImportsBuilder::new().with_resolver("env", &ResolveAll {});
        let instance = ModuleInstance::new(&module, &imports)
            .expect("failed to instantiate wasm module 1")
            .assert_no_start();

        let mem = instance
            .export_by_name("memory")
            .expect("Module expected to have 'memory' export")
            .as_memory()
            .cloned()
            .expect("'memory' export should be a memory");
        mem.set_value(0, 123 as i32)
            .expect("memory.set_value should not fail");

        println!(
            "export got {:?} wanted {:?}",
            instance
                .invoke_export("get_first_i32", &[], &mut wasmi::NopExternals)
                .expect("failed to execute get_first_i32 2"),
            Some(RuntimeValue::I32(123))
        );
    }

    {
        let wasm_binary_rust: Vec<u8> = include_bytes!(
            "./rust_contract/target/wasm32-unknown-unknown/release/wasm_example.wasm"
        )
        .iter()
        .cloned()
        .collect();
        let module = wasmi::Module::from_buffer(&wasm_binary_rust).expect("failed to load wasm 3");

        let imports = ImportsBuilder::new().with_resolver("env", &ResolveAll {});
        let instance = ModuleInstance::new(&module, &imports)
            .expect("failed to instantiate wasm module 3")
            .assert_no_start();
        let mem = instance
            .export_by_name("memory")
            .expect("Module expected to have 'memory' export")
            .as_memory()
            .cloned()
            .expect("'memory' export should be a memory");
        mem.set_value(1, 12345 as i32)
            .expect("memory.set_value should not fail");

        println!(
            "rust got {:?} wanted {:?}",
            instance
                .invoke_export("get_first_i32", &[], &mut wasmi::NopExternals)
                .expect("failed to execute get_first_i32 3"),
            Some(RuntimeValue::I32(12345))
        );
    }
}

struct ResolveAll {}

impl wasmi::ModuleImportResolver for ResolveAll {
    fn resolve_memory(
        &self,
        field_name: &str,
        descriptor: &MemoryDescriptor,
    ) -> Result<MemoryRef, wasmi::Error> {
        println!("Fetching memory from {}", field_name);
        if field_name == "memory" {
            let mem = MemoryInstance::alloc(
                memory_units::Pages(descriptor.initial() as usize),
                descriptor
                    .maximum()
                    .map(|x| memory_units::Pages(x as usize)),
            )?;

            mem.set_value(0, 1234 as i32)?;
            Ok(mem)
        } else {
            Err(wasmi::Error::Instantiation(
                "Memory imported under unknown name".to_owned(),
            ))
        }
    }
}
