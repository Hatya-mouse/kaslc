use kasl::{
    scope_manager::IOBlueprint,
    type_registry::{PrimitiveType, ResolvedType, TypeRegistry},
};

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
                PrimitiveType::Bool => print!("{}", *(ptr as *const bool)),
                PrimitiveType::Float => print!("{}", *(ptr as *const f32)),
                PrimitiveType::Int => print!("{}", *(ptr as *const i32)),
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
                print!("{}    {}: ", prefix, field.name);
                print_value(field_ptr, &field.value_type, type_registry, indent + 1);
                println!();
            }
            print!("{}}}", prefix);
        }
    }
}

pub(super) fn print_outputs(
    blueprint: &IOBlueprint,
    ptrs: &[*mut ()],
    type_registry: &TypeRegistry,
) {
    for (item, ptr) in blueprint.get_outputs().iter().zip(ptrs.iter()) {
        print!("output[{}]: ", item.name);
        print_value(*ptr as *const u8, &item.value_type, type_registry, 0);
        println!();
    }
}
