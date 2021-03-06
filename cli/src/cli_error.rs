// Copyright 2019 Walmart Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;
use std::fmt;
use std::string::ToString;

#[derive(Debug)]
pub(crate) struct CliError {
    inner: String,
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CliError: {:?}", &self.inner)
    }
}

impl From<String> for CliError {
    fn from(inner: String) -> CliError {
        CliError { inner }
    }
}

impl From<&'static str> for CliError {
    fn from(inner: &'static str) -> CliError {
        CliError {
            inner: inner.to_string(),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        &self.inner
    }

    fn cause(&self) -> Option<&dyn Error> {
        // Generic error, underlying cause isn't tracked
        None
    }
}
