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

use crate::runner::ui::print_utils::get_type_color;
use kasl::core::ast::{scope_manager::BlueprintItem, type_registry::TypeRegistry};
use owo_colors::OwoColorize;
use std::io::{Write, stdout};

pub fn print_input_header() {
    println!("{}", " INPUTS ".on_blue().bold());
}

pub fn print_inputs(inputs: &[&BlueprintItem], type_registry: &TypeRegistry) {
    for input in inputs {
        let type_color = get_type_color(&input.value_type);
        let type_string = type_registry.format_type(&input.value_type);
        println!(
            "{}: {}",
            input.name.bold(),
            type_string.color(type_color).bold()
        );
    }
}

pub fn prompt_input_buffer(
    input: &BlueprintItem,
    type_registry: &TypeRegistry,
    index: usize,
    iterations: i32,
) {
    let type_color = get_type_color(&input.value_type);
    let type_string = type_registry.format_type(&input.value_type);

    print!(
        "* Enter {} input for {} ({}/{}): ",
        type_string.color(type_color).bold(),
        input.name.bold(),
        index,
        iterations
    );
    stdout().flush().unwrap();
}

pub fn prompt_input_spread(input: &BlueprintItem, type_registry: &TypeRegistry) {
    let type_color = get_type_color(&input.value_type);
    let type_string = type_registry.format_type(&input.value_type);

    print!(
        "* Enter {} input for {}: ",
        type_string.color(type_color).bold(),
        input.name.bold(),
    );
    stdout().flush().unwrap();
}

pub fn print_entered_input(input: &BlueprintItem, str_value: &str) {
    let type_color = get_type_color(&input.value_type);

    println!(
        "{} {}: {}",
        "✓".bright_green(),
        input.name.color(type_color).bold(),
        str_value
    );
}
