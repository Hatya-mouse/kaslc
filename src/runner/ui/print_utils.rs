//
//  Copyright 2026 Shuntaro Kasatani
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use kadl::core::ast::type_registry::{PrimitiveType, ResolvedType};
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
