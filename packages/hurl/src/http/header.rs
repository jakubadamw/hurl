/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

/// Represents an HTTP header
#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display)]
#[display(fmt = "{}: {}", "self.name", "self.value")]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new(name: &str, value: &str) -> Self {
        Header {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

/// Returns all `headers` values for given `name`.
pub fn get_values(headers: &[Header], name: &str) -> Vec<String> {
    headers
        .iter()
        .filter_map(|Header { name: key, value }| {
            if key.to_lowercase() == name.to_lowercase() {
                Some(value.to_string())
            } else {
                None
            }
        })
        .collect()
}
