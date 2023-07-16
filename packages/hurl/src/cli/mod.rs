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
mod fs;
mod interactive;
pub(crate) mod options;

use std::error::Error;

use hurl::{output, report};

pub use self::fs::read_to_string;
pub use self::options::OutputType;

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display)]
#[display(fmt = "{}", "self.message")]
pub struct CliError {
    pub message: String,
}

impl From<Box<dyn Error>> for CliError {
    fn from(e: Box<dyn Error>) -> Self {
        Self {
            message: format!("{e:?}"),
        }
    }
}

impl From<&str> for CliError {
    fn from(e: &str) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

impl From<String> for CliError {
    fn from(e: String) -> Self {
        Self { message: e }
    }
}

impl From<report::Error> for CliError {
    fn from(e: report::Error) -> Self {
        Self { message: e.message }
    }
}

impl From<output::Error> for CliError {
    fn from(e: output::Error) -> Self {
        Self { message: e.message }
    }
}
