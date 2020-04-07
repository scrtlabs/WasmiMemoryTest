use wabt;
use wasmi;

fn main() {
    let wasm_binary_export: Vec<u8> = wabt::wat2wasm(
        r#"
        (module
            (memory (;0;) 17)
            (export "memory" (memory 0))

            (func (export "get_first_i32") (result i32)
                i32.load 0
            )
        )
        "#,
    )
    .expect("failed to parse wat 1");

    let wasm_binary_import: Vec<u8> = wabt::wat2wasm(
        r#"
        (module
            (memory (import "env" "memory") 17)

            (func (export "get_first_i32") (result i32)
                i32.load 0
            )
        )
        "#,
    )
    .expect("failed to parse wat 2");
}
