use kasl::core::ast::type_registry::{PrimitiveType, ResolvedType};
use owo_colors::{AnsiColors, DynColors};

pub(super) fn get_type_color(value_type: &ResolvedType) -> DynColors {
    match value_type {
        ResolvedType::Primitive(prim_type) => match prim_type {
            PrimitiveType::Bool => DynColors::Ansi(AnsiColors::Magenta),
            PrimitiveType::Float => DynColors::Ansi(AnsiColors::Cyan),
            PrimitiveType::Int => DynColors::Ansi(AnsiColors::Blue),
            PrimitiveType::Void => DynColors::Ansi(AnsiColors::White),
        },
        ResolvedType::Array(_) => DynColors::Ansi(AnsiColors::Cyan),
        ResolvedType::Struct(_) => DynColors::Ansi(AnsiColors::Yellow),
    }
}
