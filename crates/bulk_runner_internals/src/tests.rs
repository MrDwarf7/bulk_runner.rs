#![allow(unused)]

use crate::*;

#[doc(hidden)]
#[cfg(test)]
mod tests_of_to_args {
    use super::*;
    use clap::Parser;

    #[derive(Parser, Clone, Serialize)]
    struct TestArgs {
        #[arg(long)]
        flag: bool,
        #[arg(short, long)]
        string: String,
        #[arg(short, long)]
        number: u64,
    }

    #[test]
    fn test_to_args_with_all_fields() {
        let args = TestArgs {
            flag: true,
            string: "test".to_string(),
            number: 42,
        };

        let result = args.to_args();
        println!("{:?}", result);

        assert_eq!(
            result,
            vec![
                "--flag".to_string(),
                "--string".to_string(),
                "test".to_string(),
                "--number".to_string(),
                "42".to_string(),
            ]
        );
    }

    #[test]
    fn test_to_args_with_false_flag() {
        let args = TestArgs {
            flag: false,
            string: "test".to_string(),
            number: 42,
        };

        let result = args.to_args();
        println!("{:?}", result);

        assert_eq!(
            result,
            vec![
                "--string".to_string(),
                "test".to_string(),
                "--number".to_string(),
                "42".to_string(),
            ]
        );
    }

    #[test]
    fn test_to_args_with_short_options() {
        #[derive(Parser, Clone, Serialize)]
        struct ShortArgs {
            #[arg(short)]
            flag: bool,
            #[arg(short)]
            string: String,
            #[arg(short)]
            number: u64,
        }

        let args = ShortArgs {
            flag: true,
            string: "test".to_string(),
            number: 42,
        };

        let result = args.to_args();
        println!("{:?}", result);

        assert_eq!(
            result,
            vec![
                "-f".to_string(),
                "-s".to_string(),
                "test".to_string(),
                "-n".to_string(),
                "42".to_string(),
            ]
        );
    }

    #[test]
    fn test_to_args_with_empty_string() {
        let args = TestArgs {
            flag: true,
            string: "".to_string(),
            number: 42,
        };

        let result = args.to_args();
        println!("{:?}", result);

        assert_eq!(
            result,
            vec![
                "--flag".to_string(),
                "--string".to_string(),
                "".to_string(),
                "--number".to_string(),
                "42".to_string(),
            ]
        );
    }

    #[test]
    fn test_to_args_with_zero_number() {
        let args = TestArgs {
            flag: true,
            string: "test".to_string(),
            number: 0,
        };

        let result = args.to_args();
        println!("{:?}", result);

        assert_eq!(
            result,
            vec![
                "--flag".to_string(),
                "--string".to_string(),
                "test".to_string(),
                "--number".to_string(),
                "0".to_string(),
            ]
        );
    }

    #[test]
    fn test_to_args_with_large_number() {
        let args = TestArgs {
            flag: true,
            string: "test".to_string(),
            number: u64::MAX,
        };

        let result = args.to_args();
        println!("{:?}", result);

        assert_eq!(
            result,
            vec![
                "--flag".to_string(),
                "--string".to_string(),
                "test".to_string(),
                "--number".to_string(),
                u64::MAX.to_string(),
            ]
        );
    }
}

#[cfg(test)]
mod tests_of_complex {
    use super::*;
    use clap::Parser;

    #[derive(Parser, Clone, Serialize)]
    struct ComplexArgsUser {
        quoted_user: String,
        unquoted_user: String,
    }

    #[test]
    fn test_to_args_with_complex_user() {
        #[doc(hidden)]
        let args = ComplexArgsUser {
            quoted_user: r"username".to_string(),
            unquoted_user: r"username".to_string(),
        };

        let result = args.to_args();
        println!("{result:?}");
    }

    #[derive(Parser, Clone, Serialize)]
    struct ComplexArgsPass {
        quoted_pass: String,
        unquoted_pass: String,
    }

    #[test]
    fn test_to_args_with_complex_pass() {
        let args = ComplexArgsPass {
            quoted_pass: r"password".to_string(),
            unquoted_pass: r"password".to_string(),
        };

        let result = args.to_args();
        println!("{result:?}");
    }
}
