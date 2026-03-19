use kasl::{
    scope_manager::IOBlueprint,
    type_registry::{PrimitiveType, ResolvedType, TypeRegistry},
};
use owo_colors::OwoColorize;

fn print_value(
    ptr: *const u8,
    value_type: &ResolvedType,
    type_registry: &TypeRegistry,
    indent: usize,
) {
    // Create a prefix for indentation
    let prefix = "    ".repeat(indent);

    match value_type {
        ResolvedType::Primitive(prim_type) => unsafe {
            match prim_type {
                PrimitiveType::Bool => print!("{}", (*(ptr as *const bool)).magenta()),
                PrimitiveType::Float => print!("{}", (*(ptr as *const f32)).cyan()),
                PrimitiveType::Int => print!("{}", (*(ptr as *const i32)).blue()),
                PrimitiveType::Void => print!("()"),
            }
        },
        ResolvedType::Struct(struct_id) => {
            let struct_decl = type_registry.get_struct(struct_id).unwrap();
            println!("{} {{", struct_decl.name);

            // Loop over the fields and print each one
            for (field, offset) in struct_decl
                .fields
                .iter()
                .zip(struct_decl.field_offsets.iter())
            {
                let field_ptr = unsafe { ptr.offset(*offset as isize) };
                print!("{}    {}: ", prefix, field.name.bold());
                print_value(field_ptr, &field.value_type, type_registry, indent + 1);
                println!();
            }
            print!("{}}}", prefix);
        }
    }
}

pub fn print_outputs(
    blueprint: &IOBlueprint,
    ptrs: &[*mut ()],
    iterations: usize,
    type_registry: &TypeRegistry,
) {
    println!("{}", " OUTPUTS ".on_bright_green().bold());

    for (item, ptr) in blueprint.get_outputs().iter().zip(ptrs.iter()) {
        print!("{}: ", item.name.bold());
        let last_ptr = unsafe { (*ptr as *const u8).add((iterations - 1) * item.actual_size) };
        print_value(last_ptr, &item.value_type, type_registry, 0);
        println!();
    }
}
