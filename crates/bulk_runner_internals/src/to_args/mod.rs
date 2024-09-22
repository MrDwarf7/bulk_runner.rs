//! # ToArgs
//!
//! # NOTE
//! This is NOT CURRENTLY USED.
//!
//! The `ToArgs` trait is used to convert a struct into a vector of strings,
//! which can be used as arguments for a command line tool.
//!
//! This is used internally by the `internals` crate to convert the structs
//! used in the `AutomateCBuilder` into the arguments needed to run the `AutomateC` executable.
//!
//!

use clap::Parser;
use serde::Serialize;
use serde_json::Number;

/// Custom trait to serialize arguments into CLI arguments
pub trait ToArgs {
    fn to_args(&self) -> Vec<String>;
}

/// Default implementation for any type that implements `Parser` and `Clone`
impl<T> ToArgs for T
where
    T: Parser + Clone + Serialize,
{
    fn to_args(&self) -> Vec<String> {
        let cmd = T::command();

        let mut arg_values = vec![];

        for arg in cmd.get_arguments() {
            let id = arg.get_id();

            let value = get_field_value(self, id.as_str());

            if let Some(value) = value {
                match value {
                    FieldValue::Bool(true) => {
                        if let Some(long) = arg.get_long() {
                            arg_values.push(format!("--{long}"));
                        } else if let Some(short) = arg.get_short() {
                            arg_values.push(format!("-{short}"));
                        }
                    }
                    FieldValue::Bool(false) => (),
                    FieldValue::String(s) => {
                        if let Some(long) = arg.get_long() {
                            arg_values.push(format!("--{long}"));
                            arg_values.push(s);
                        } else if let Some(short) = arg.get_short() {
                            arg_values.push(format!("-{short}"));
                            arg_values.push(s);
                        } else {
                            arg_values.push(s);
                        }
                    }
                    FieldValue::Number(n) => {
                        if let Some(long) = arg.get_long() {
                            arg_values.push(format!("--{long}"));
                            arg_values.push(n.to_string());
                        } else if let Some(short) = arg.get_short() {
                            arg_values.push(format!("-{short}"));
                            arg_values.push(n.to_string());
                        }
                    }
                }
            }
        }
        arg_values
    }
}

enum FieldValue {
    String(String),
    Bool(bool),
    Number(Number),
}

/// Get the value of a field from a struct instance - Uses reflection via `serde_json`
fn get_field_value<T>(instance: &T, field_name: &str) -> Option<FieldValue>
where
    T: Clone + Serialize,
{
    let json_value = serde_json::to_value(instance.clone()).ok()?;

    match &json_value.get(field_name)? {
        serde_json::Value::String(s) => Some(FieldValue::String(s.clone())),
        serde_json::Value::Bool(b) => Some(FieldValue::Bool(*b)),
        serde_json::Value::Number(n) => {
            if n.as_u64().is_some() {
                Some(FieldValue::Number(n.clone()))
            } else {
                None
            }
        }
        serde_json::Value::Array(_) | serde_json::Value::Object(_) | serde_json::Value::Null => {
            None
        }
    }
}

#[cfg(test)]
mod isolated {
    use super::*;
    use clap::Parser;

    #[derive(Parser, Clone, Serialize)]
    struct ComplexArgs {
        user: String,
        pass: String,
    }

    #[test]
    fn test_to_args_with_complex_user_and_pass() {
        let args = ComplexArgs {
            user: "username".to_string(),
            pass: "password".to_string(),
        };

        let result = args.to_args();
        println!("{result:?}");

        assert_eq!(
            result,
            vec![
                "--user".to_string(),
                args.user,
                "--pass".to_string(),
                args.pass,
            ]
        );
    }
}
