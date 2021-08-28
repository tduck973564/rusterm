/*
 * Copyright (c) 2021 Thomas Duckworth <tduck973564@gmail.com>.
 * This file is under the `rusterm` project, which is licensed under the GNU GPL v3.0 which you can read here: https://www.gnu.org/licenses/gpl-3.0.en.html
 */

//! Re-exports for things you need when using this crate.

pub use super::{error::Error as RustermError, lex::*, Command, Console};
pub use std::collections::HashMap;
pub use std::convert::TryInto;
