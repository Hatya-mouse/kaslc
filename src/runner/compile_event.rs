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

use kadl::core::error::ErrorRecord;
use std::time::Duration;

pub(super) enum CompileEvent {
    Parsing,
    Building,
    Builded(Duration),
    Running,
    Finished {
        exec_elapsed: Duration,
        avg_elapsed: Duration,
    },
    Error(String),
    KaslError(Vec<ErrorRecord>, String),
}
