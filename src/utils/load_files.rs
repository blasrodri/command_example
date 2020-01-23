use std::fs;

fn get_base_dir() -> &'static str {
    const BASE_DIR: &'static str = "./src/examples/examples-data/";
    BASE_DIR
}

pub fn load_json_file_paths(command_name: Option<&str>) -> Vec<String> {
    load_file_paths(command_name, "json", get_base_dir())
}

fn load_file_paths(command_name: Option<&str>, extension: &str, base_dir: &str) -> Vec<String> {
    // Horrible way out for not using a walkdir dependency
    // It assumes that the max level of depth is 2.
    if let None = command_name {
        return fs::read_dir(&base_dir)
            .expect(&format!("Could not open dir {}", &base_dir)[..])
            .map(|res| res.unwrap().path())
            .map(|p| dbg!(p))
            .collect::<Vec<_>>()
            .iter()
            .map(|p| {
                load_file_paths(
                    p.components().last().unwrap().as_os_str().to_str(),
                    extension,
                    base_dir,
                )
            })
            .collect::<Vec<_>>()
            .concat();
    }
    let base_dir = if let Some(cmd_name) = command_name {
        format!("{}/{}", base_dir, cmd_name)
    } else {
        base_dir.to_string()
    };
    fs::read_dir(&base_dir)
        .expect(&format!("Could not open dir {}", &base_dir)[..])
        .map(|res| res.unwrap().path())
        .collect::<Vec<_>>()
        .iter()
        // .map(|p| dbg!((command_name, p)))
        .filter(|p| p.as_path().is_file())
        .filter(|p| match p.to_str().map(|s| s.ends_with(extension)) {
            Some(v) => v,
            None => false,
        })
        // convert path buff to string
        .map(|p| p.to_str().unwrap().to_string())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;
    fn get_base_dir() -> &'static str {
        const BASE_DIR: &'static str = "./test/examples-data";
        BASE_DIR
    }

    #[test]
    fn load_file_paths_w_command_name() {
        assert_eq!(
            vec![format!(
                "{}/{}/{}",
                get_base_dir(),
                "test_command",
                "empty.json"
            )],
            load_file_paths(Some("test_command"), "json", get_base_dir())
        )
    }

    #[test]
    fn load_file_paths_wo_command_name() {
        assert_eq!(
            vec![
                format!("{}/{}/{}", get_base_dir(), "test_command", "empty.json"),
                format!(
                    "{}/{}/{}",
                    get_base_dir(),
                    "test_command2",
                    "test_empty.json"
                )
            ]
            .sort(),
            load_file_paths(None, "json", get_base_dir()).sort(),
        )
    }
}