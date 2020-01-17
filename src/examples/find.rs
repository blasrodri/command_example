use crate::commands::command::*;

use serde_json;

lazy_static! {
    pub static ref EXAMPLES: Vec<CommandExample<'static>> = {
        vec![
            command_example!(
                command!("find", "find all rust files and apply rust fmt"),
                synopsis!(
                    vec![
                        command_options!(
                            vec![
                                flag_plus_value!(flag!("--", "name"), "*.rs"),
                                flag_plus_value!(flag!("-", "exec"), "cargo fmt {} \\;"),
                            ]
                        ),
                    ]
                )
            ),

        ]
    };
}

pub fn examples() -> () {
    EXAMPLES.iter()
    .for_each(
        |cmd_ex| println!("{}", serde_json::to_string_pretty(cmd_ex).unwrap())
    )
}