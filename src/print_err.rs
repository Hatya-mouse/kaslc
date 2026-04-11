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

use owo_colors::OwoColorize;
use std::fmt::Display;

pub fn print_err_header(optional_message: Option<&str>) {
    eprintln!(
        "{} {}",
        " ERROR ".on_red().bold(),
        optional_message.unwrap_or("")
    );
}

pub fn print_warning_header(optional_message: Option<&str>) {
    eprintln!(
        "{} {}",
        " WARNING ".on_yellow().bold(),
        optional_message.unwrap_or("")
    );
}

pub fn print_err(e: impl Display) {
    print_err_header(None);
    eprintln!("{}", e);
}
