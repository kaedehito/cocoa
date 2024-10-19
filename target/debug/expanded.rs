#![feature(prelude_import)]
#![allow(warnings)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod PS1 {
    use crate::structs;
    use colored::{self, Colorize};
    impl structs::dir {
        pub fn display_ps1(&self) -> String {
            let format = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!(
                    "{0}{1}{2} {3}{4} ",
                    self.user_name.cyan(),
                    "@".green(),
                    self.hostname.magenta(),
                    self.now_dir.bright_red(),
                    "$".bright_white(),
                ));
                res
            });
            format
        }
        pub fn chenge_dir(&mut self, to_chenge_dir: &str) {
            let now_dir = self.now_dir.clone();
        }
    }
}
mod cd {
    use crate::{load_setup, structs};
    use dirs::home_dir;
    use std::{env, io};
    impl structs::dir {
        pub fn cd(&mut self, dir: &String, setup: &load_setup::Config) {
            let home_dir = home_dir().unwrap();
            let home_dir_str = home_dir.to_str().unwrap();
            if let Err(e) = env::set_current_dir(dir) {
                if e.kind() == io::ErrorKind::NotFound {
                    {
                        ::std::io::_eprint(format_args!(
                            "cocoa: cd: {0}: {1}\n",
                            dir, setup.cd_error_message,
                        ));
                    };
                } else {
                    {
                        ::std::io::_eprint(format_args!("cocoa: cd: {0}: {1}\n", dir, e));
                    };
                }
                return;
            }
            let current_dir = env::current_dir().unwrap();
            let current_dir_str = current_dir.to_str().unwrap();
            self.now_dir = if current_dir_str.starts_with(home_dir_str) {
                current_dir_str.replacen(home_dir_str, "~", 1)
            } else {
                current_dir_str.to_string()
            };
        }
    }
}
mod input {
    use crate::load_setup;
    use crate::{cd, structs};
    use dirs::home_dir;
    use hostname;
    use std::{
        io::{self, Write},
        process::{Command, Stdio},
        str,
    };
    pub fn input(ps1: &mut structs::dir, input: &str, setup: load_setup::Config) -> String {
        let mut cmd = input.split_whitespace().collect::<Vec<&str>>();
        if let Some(command) = cmd.first() {
            match *command {
                "cd" => {
                    if cmd.len() >= 2 {
                        ps1.cd(&cmd[1].to_string(), &setup);
                    } else {
                        let home = home_dir().unwrap();
                        ps1.cd(&home.to_str().unwrap().to_string(), &setup);
                    }
                    return "".to_string();
                }
                "exit" => {
                    {
                        ::std::io::_print(format_args!("exit\n"));
                    };
                    return "exit".to_string();
                }
                "" => return "".to_string(),
                "ls" => {
                    if let Ok(o) = Command::new("ls")
                        .args(&cmd[1..])
                        .arg(setup.ls_config)
                        .arg("-C")
                        .output()
                    {
                        let stdout = str::from_utf8(&o.stdout).unwrap();
                        {
                            ::std::io::_print(format_args!("{0}", stdout));
                        };
                        io::stdout().flush().unwrap();
                    }
                    return "".to_string();
                }
                _ => {}
            }
            match Command::new(command)
                .args(&cmd[1..])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
            {
                Ok(mut o) => {
                    o.wait().unwrap();
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        {
                            ::std::io::_eprint(format_args!(
                                "cocoa: {0}: command not found\n",
                                command
                            ));
                        };
                    } else {
                        {
                            ::std::io::_eprint(format_args!("{0}\n", e));
                        };
                    }
                }
            }
        }
        return "".to_string();
    }
}
mod structs {
    #[allow(non_camel_case_types)]
    pub struct dir {
        pub now_dir: String,
        pub hostname: String,
        pub user_name: String,
    }
}
use colored::*;
use dirs::home_dir;
mod load_setup {
    use dirs::home_dir;
    use serde::Deserialize;
    use std::{fs, path::Path};
    use toml::de::Error;
    pub struct Config {
        pub start_up_text: String,
        pub ls_config: String,
        pub cd_error_message: String,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Config {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "start_up_text" => _serde::__private::Ok(__Field::__field0),
                            "ls_config" => _serde::__private::Ok(__Field::__field1),
                            "cd_error_message" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"start_up_text" => _serde::__private::Ok(__Field::__field0),
                            b"ls_config" => _serde::__private::Ok(__Field::__field1),
                            b"cd_error_message" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Config>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Config;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Config")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 =
                            match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Config with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field1 =
                            match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Config with 3 elements",
                                        ),
                                    );
                                }
                            };
                        let __field2 =
                            match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Config with 3 elements",
                                        ),
                                    );
                                }
                            };
                        _serde::__private::Ok(Config {
                            start_up_text: __field0,
                            ls_config: __field1,
                            cd_error_message: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "start_up_text",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "ls_config",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "cd_error_message",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("start_up_text")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("ls_config")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("cd_error_message")?
                            }
                        };
                        _serde::__private::Ok(Config {
                            start_up_text: __field0,
                            ls_config: __field1,
                            cd_error_message: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] =
                    &["start_up_text", "ls_config", "cd_error_message"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Config",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Config>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Config {
        #[inline]
        fn clone(&self) -> Config {
            Config {
                start_up_text: ::core::clone::Clone::clone(&self.start_up_text),
                ls_config: ::core::clone::Clone::clone(&self.ls_config),
                cd_error_message: ::core::clone::Clone::clone(&self.cd_error_message),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Config {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Config",
                "start_up_text",
                &self.start_up_text,
                "ls_config",
                &self.ls_config,
                "cd_error_message",
                &&self.cd_error_message,
            )
        }
    }
    pub fn setup() -> Result<Config, Error> {
        let home = home_dir().unwrap();
        let config_file = ::alloc::__export::must_use({
            let res = ::alloc::fmt::format(format_args!("{0}/.cocoa_rc", home.display()));
            res
        });
        let path = Path::new(&config_file);
        if !path.exists() {
            return Ok(Config {
                start_up_text: "welcome to cocoa".to_string(),
                ls_config: "--color=always".to_string(),
                cd_error_message: "No such file or directory".to_string(),
            });
        }
        let config_content = fs::read_to_string(path).expect("ファイルが見つかりません");
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}
use rustyline::error::ReadlineError;
use rustyline::{
    completion::FilenameCompleter,
    config::{Builder, Configurer},
    CompletionType, DefaultEditor,
};
use std::{
    env,
    fmt::format,
    fs,
    io::{self, Write},
};
fn main() {
    let cfg = load_setup::setup().unwrap();
    {
        ::std::io::_print(format_args!("{0}\n", cfg.start_up_text));
    };
    let host_name = hostname::get().unwrap().to_str().unwrap().to_string();
    let mut username = String::new();
    if let Ok(o) = env::var("USER").or_else(|_| env::var("USERNAME")) {
        username = o.clone();
    } else {
        {
            ::core::panicking::panic_fmt(format_args!("Failed to get user name"));
        };
    };
    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    let mut ps1 = structs::dir {
        now_dir: current_dir,
        hostname: host_name,
        user_name: username,
    };
    let homedir = home_dir().unwrap();
    let format = ::alloc::__export::must_use({
        let res = ::alloc::fmt::format(format_args!("{0}/.bash_history", homedir.display()));
        res
    });
    let completer = FilenameCompleter::new();
    let config = Builder::new().completion_type(CompletionType::List).build();
    let mut rl = DefaultEditor::with_config(config).unwrap();
    if rl.load_history(&format).is_err() {
        {
            ::core::panicking::panic_fmt(format_args!("Failed to read histry"));
        };
    }
    loop {
        let mut display_ps1 = ps1.display_ps1();
        let readline = rl.readline(&display_ps1);
        match readline {
            Ok(o) => {
                rl.add_history_entry(o.as_str());
                let s = input::input(&mut ps1, o.as_str(), cfg.clone());
                if s == "exit" {
                    rl.save_history(&format).unwrap();
                    std::process::exit(0);
                }
            }
            Err(ReadlineError::Interrupted) => {
                {
                    ::std::io::_print(format_args!("^C\n"));
                };
                continue;
            }
            Err(ReadlineError::Eof) => {
                return;
            }
            Err(e) => {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    #[rustc_const_panic_str]
                    #[rustc_do_not_const_check]
                    const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                        ::core::panicking::panic_display(arg)
                    }
                    panic_cold_display(&e);
                };
            }
        }
    }
}
