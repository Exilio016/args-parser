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
 liitations under the License. */

use std::collections::HashMap;

struct Option {
    short: char,
    long: String,
    description: String,
    has_argument: bool
}

struct Parameter {
    name: String,
    description: String
}

pub struct Parser {
    required_options: Vec<Option>,
    options: Vec<Option>,
    parameters: Vec<Parameter>,
    program_name: String
}


pub struct ParsedArgs {
    options: Vec<char>,
    options_with_args: HashMap<char, String>,
    parameters: HashMap<String, String>
}

impl ParsedArgs {
    pub fn has_option(&self, short_name: char) -> bool {
        return self.options.contains(&short_name) 
            || self.options_with_args.contains_key(&short_name);
    }
    pub fn get_option(&self, short_name: char) -> std::option::Option<&String> {
        return self.options_with_args.get(&short_name);
    }
    pub fn get_parameter(&self, name: &str) -> std::option::Option<&String> {
            return self.parameters.get(name);
    }
}

impl Parser {
    pub fn option(&mut self, short: char, long: &str, description: &str,
                  required: bool, has_argument: bool) {
        
        let option = Option {
            short,
            long: long.to_string(),
            description: description.to_string(),
            has_argument
        };

        if required {
            self.required_options.push(option);
        }
        else {
            self.options.push(option);
        }

    }

    fn parse_short_options(&self, argument: &str,
                           args: &mut ParsedArgs, is_argument: &mut bool,
                           current_opt: &mut char) -> Result<(), String> {

        let mut i = 1;
        for c in (&argument[1..]).chars() {
            let opt = self.get_option(c);   
            match opt {
                Some(o) => {
                    if o.has_argument {
                        if i < (argument.len() - 1) {
                            args.options_with_args
                                .insert(c,(&argument[i+1..]).to_string());
                            break;
                        }
                        else {
                            *current_opt = c;
                            *is_argument = true;
                        }
                    } 
                    else {
                        args.options.push(c); 
                    }
                }
                None => return Err(format!("Unkown or duplicated option {}!", c)),
            }
        
            i +=1;
        }
        return Ok(());
    }

    fn parse_long_option(&self, argument: &str, args: &mut ParsedArgs,
                         is_argument: &mut bool, current_opt: &mut char) -> Result<(), String> {
        match (&argument[2..]).split_once("=") {
            Some((name, value)) => {
                match self.get_long_option(name) {
                    Some(opt) => {
                        if !opt.has_argument {
                            return Err(format!("Option '--{}' should have no arguments!", name));
                        }
                        args.options_with_args.insert(opt.short, value.to_string());
                    }
                    None => return Err(format!("Unkown option '--{}'!", name)),
                }
            }
            None => { 
                match self.get_long_option(&argument[2..]) {
                    Some(opt) => {
                        if opt.has_argument {
                            *current_opt = opt.short;
                            *is_argument = true;
                        }
                        else {
                            args.options.push(opt.short);
                        }
                    }
                    None => {
                        let name = &argument[2..];
                        return Err(format!("Unkown option {}!", name));
                    }
                }
            }
        }
        return Ok(());
    }
    pub fn parse(&self, args: Vec<String>) -> Result<ParsedArgs, String> {
        let mut parsed = ParsedArgs {
            options: Vec::new(),
            options_with_args: HashMap::new(),
            parameters: HashMap::new()
        };

        let mut is_argument = false;
        let mut current_opt = ' ';
        let mut end_of_options = false;
        let mut parameter_index: usize = 0;
        let mut iter = args.iter();
        iter.next();

        for argument in iter {
            if is_argument {
                parsed.options_with_args.insert(current_opt, argument.to_string());
                is_argument = false;
            }
            else if argument.starts_with("--") && !end_of_options {
                if argument.len() == 2 {
                    end_of_options = true;
                }
                else {
                    match self.parse_long_option(argument, &mut parsed, &mut is_argument, &mut current_opt) {
                        Err(msg) => return Err(msg),
                       _ => {}
                    }
                }
            }
            else if argument.starts_with("-") && !end_of_options {
                match self.parse_short_options(argument, &mut parsed, &mut is_argument, &mut current_opt) {
                    Err(msg) => return Err(msg),
                    _ => {}
                }
            }
            else if parameter_index < self.parameters.len() {
                let p = &self.parameters[parameter_index];
                parsed.parameters.insert((&p.name).to_string(), argument.to_string());
                parameter_index += 1;
            }
        }

        for req in self.required_options.iter() {
            if !parsed.options.contains(&req.short) &&
                !parsed.options_with_args.contains_key(&req.short) {
                    let name = &req.long;
                    return Err(format!("Option '--{}' is required!", name));
                }
        }
        for req in self.parameters.iter() {
            if !parsed.parameters.contains_key(&req.name) {
                let name = &req.name;
                return Err(format!("Parameter <{}> is required!", name));
            }
        }

        return Ok(parsed);
    }

    fn get_option(&self, short_name: char) -> std::option::Option<&Option> {
        for opt in self.required_options.iter() {
            if opt.short == short_name {
                return Some(opt);
            }
        }
        for opt in self.options.iter() {
            if opt.short == short_name {
                return Some(opt);
            }
        }
        return None;

    }
    fn get_long_option(&self, long_name: &str) -> std::option::Option<&Option> {
        for opt in self.required_options.iter() {
            if opt.long == long_name {
                return Some(opt);
            }
        }
        for opt in self.options.iter() {
            if opt.long == long_name {
                return Some(opt);
            }
        }
        return None;

    }

    fn get_parameter_list(&self) -> String {
        let mut args_list =  String::from("");
        let mut is_first = true;
        for p in self.parameters.iter() {
            let parameter_name = &p.name;
            if is_first {
                args_list = args_list + &format!("<{}>", parameter_name);
            }
            else {
                args_list = args_list + &format!(" <{}>", parameter_name);
            }
            is_first = false;
        }
        return args_list;
    }

    fn get_optional_options(&self) -> String {
        let mut options_list =  String::from("");
        let mut options_without_args = 0;
        for o in self.options.iter() {
            if !o.has_argument {
                if options_without_args == 0 {
                    options_list = options_list + "[-";
                }
                options_without_args += 1;
                options_list = options_list + &String::from(o.short);
            }
        }
        if options_without_args > 0 {
            options_list = options_list + "]";
        }
        for o in self.options.iter() {
            if o.has_argument {
                let short = o.short;
                if options_without_args > 0 {
                    options_list = options_list + &format!(" [-{short} <arg>]");
                }
                else {
                    options_list = options_list + &format!("[-{short} <arg>]");
                }
            }
        }
        return options_list;
    }

    fn get_required_options(&self) -> String {
        let mut options_list = String::from("");
        let mut options_without_args = 0;
        for o in self.required_options.iter() {
            if !o.has_argument {
                if options_without_args == 0 {
                    options_list = options_list + "-";
                }
                options_without_args += 1;
                options_list = options_list + &String::from(o.short);
            }
        }
        for o in self.required_options.iter() {
            if o.has_argument {
                let short = String::from(o.short);
                if options_without_args > 0 {
                    options_list = options_list + &format!(" -{short} <arg>");
                }
                else {
                    options_list = options_list + &format!("-{short} <arg>");
                }
            }
        }
        return options_list;
    }

    fn print_option_help_line(opt: &Option) {
        let short = opt.short;
        let long = &opt.long;
        let description = &opt.description;
        if opt.has_argument {
            println!("\t-{short}, --{long}=<arg>\t\t{description}");
        }
        else {
            println!("\t-{short}, --{long}\t\t{description}");
        }
    }

    pub fn print_help(&self) {
        let name = &self.program_name;
        let mut option_list: String = self.get_optional_options();
        if option_list != "" {
            option_list += " ";
        }
        option_list += &self.get_required_options();
        let parameter_list = self.get_parameter_list();

        println!("usage: {name} {option_list} [--] {parameter_list}");

        for p in self.parameters.iter() {
            let parameter_name = &p.name;
            let description = &p.description;
            println!("\t<{parameter_name}>\t\t{description}");
        }
        for o in self.required_options.iter() {
            Parser::print_option_help_line(&o);
        }
        for o in self.options.iter() {
            Parser::print_option_help_line(&o);
        }

    }

    pub fn parameter(&mut self, name: &str, description: &str) {
        self.parameters.push(Parameter { name: name.to_string(), description: description.to_string() })
    }

    pub fn new(program_name: &str) -> Parser {
        return Parser { 
            required_options: Vec::new(),
            options: Vec::new(),
            parameters: Vec::new(),
            program_name: program_name.to_string()
        }
    }

    
}
