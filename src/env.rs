/*
:project: gpts-code-analyst
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use lazy_static::lazy_static;
use pico_args::{Arguments, Error};
use std::fmt::Display;
use std::str::FromStr;

fn get_arg_value<T>(arg_name: &'static str) -> Result<T, Error>
where
    T: FromStr,
    T::Err: Display,
{
    let mut args = Arguments::from_env();
    let value: T = args.value_from_str(arg_name)?;
    return Ok(value);
}

lazy_static! {
    pub static ref GITHUB_TOKEN: Result<String, Error> = get_arg_value("--token");
}
