/* The APACHE License (APACHE)

Copyright (c) 2022 Bruno Flávio Ferreira. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License. */

pub mod arguments;

#[cfg(test)]
mod tests {
    use crate::arguments::Parser;

    #[test]
    fn should_parse_options() {
        let mut parser = Parser::new("test");
        parser.option('a', "a", "a", false, false);
        parser.option('b', "b", "b", false, false);
        parser.option('c', "c", "c", false, false);
        parser.option('d', "d", "d", false, false);
        parser.option('e', "e", "e", false, false);
        parser.option('f', "f", "f", false, true);
        parser.option('g', "g", "g", false, true);
        let args = parser.parse(vec![String::from("test"), String::from("-abc"),
            String::from("-d"), String::from("-e"), String::from("-f"), String::from("arg"), String::from("-garg")]);

        match args {
            Ok(args) => {
                assert!(args.has_option('a'));
                assert!(args.has_option('b'));
                assert!(args.has_option('c'));
                assert!(args.has_option('d'));
                assert!(args.has_option('e'));
                let f = args.get_option('f').unwrap();
                let g = args.get_option('g').unwrap();
                assert_eq!("arg", f, "wrong argument for option -f");
                assert_eq!("arg", g, "wrong argument for oprion -g");
            }
            Err(msg) => assert!(false, "{msg}"),
        }
    }

    #[test]
    fn should_parse_parameters() {
        let mut parser = Parser::new("test");
        parser.parameter("first", "first parameter");
        parser.parameter("second", "second parameter");
        parser.parameter("third", "second parameter");
        let args = parser.parse(vec![String::from("test"), String::from("foo"),
            String::from("bar"), String::from("fooBar")]);

        match args {
            Ok(args) => {
                let f = args.get_parameter("first").unwrap();
                let s = args.get_parameter("second").unwrap();
                let t = args.get_parameter("third").unwrap();
                assert_eq!("foo", f);
                assert_eq!("bar", s);
                assert_eq!("fooBar", t);
            }
            Err(msg) => assert!(false, "{msg}"),
        }

    }

    #[test]
    fn parse_should_fail_if_missing_required_option() {
        let mut parser = Parser::new("test");
        parser.option('r', "required", "required option", true, false);

        let args = parser.parse(vec![String::from("test"), String::from("foo"),
            String::from("bar"), String::from("fooBar")]);

        match args {
            Ok(_) => {
                assert!(false, "should have failed parsing");
            }
            Err(msg) => assert_eq!("Option '--required' is required!", msg),
        }

    }

    #[test]
    fn parse_should_fail_if_missing_parameter() {
        let mut parser = Parser::new("test");
        parser.parameter("first", "first parameter");
        parser.parameter("second", "second parameter");
        parser.parameter("third", "second parameter");
        let args = parser.parse(vec![String::from("test"), String::from("foo"),
            String::from("bar")]);

        match args {
            Ok(_) => {
                assert!(false, "should have failed parsing");
            }
            Err(msg) => assert_eq!("Parameter <third> is required!", msg),
        }

    }
}
