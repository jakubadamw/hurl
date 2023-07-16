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
//! Serialize a Hurl run result to a file.
//!
//! Therea are two supported serialisation:
//! - JSON: the whole run is serialized to JSON (like the [HAR](https://en.wikipedia.org/wiki/HAR_(file_format)) format)
//! - raw: the last response of a run is serialized to a file. The body can be automatically uncompress
//! or written as it.
mod json;
mod raw;
mod stdout;

use std::io::Write;
use std::path::Path;

pub use self::json::write_json;
pub use self::raw::write_body;

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display)]
#[display(fmt = "{}", "self.message")]
pub struct Error {
    pub message: String,
}

/// Writes `bytes` to the file `filename` or stdout by default.
fn write_output(bytes: &Vec<u8>, filename: &Option<String>) -> Result<(), Error> {
    match filename {
        None => stdout::write_stdout(bytes.as_slice()),
        Some(filename) => {
            let path = Path::new(filename.as_str());
            let mut file = match std::fs::File::create(path) {
                Err(why) => {
                    return Err(Error {
                        message: format!("Issue writing to {}: {:?}", path.display(), why),
                    });
                }
                Ok(file) => file,
            };
            file.write_all(bytes.as_slice())
                .expect("writing bytes to file");
            Ok(())
        }
    }
}
