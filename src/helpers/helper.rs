use super::path;
use std::{fs, io, path::PathBuf};

extern crate ini;
use ini::Ini;

pub struct SoftwareInfo {
    name: String,
    icon: String,
}

fn get_all_desktop_file() -> io::Result<Vec<PathBuf>> {
    let path = path::PathSoftware::Root.as_string();
    let all_desktop = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    return Ok(all_desktop);
}

fn parse_desktop_file(desktop_file: PathBuf) -> SoftwareInfo {
    static SECTION_CONFIG: &str = "Desktop Entry";
    let conf = Ini::load_from_file(desktop_file).unwrap();
    let section = conf.section(Some(SECTION_CONFIG));

    return match section {
        Some(sec) => {
            let name = sec.get("Icon").unwrap_or("");
            let icon = sec.get("Name").unwrap_or("");
            return SoftwareInfo {
                name: name.to_owned(),
                icon: icon.to_owned(),
            };
        }
        None => SoftwareInfo {
            name: "lol".to_owned(),
            icon: "lol".to_owned(),
        },
    };
}

pub fn get_list_software() -> Vec<SoftwareInfo> {
    let all_desktop_file = get_all_desktop_file();

    return match all_desktop_file {
        Ok(desktop) => desktop.into_iter().map(|a| parse_desktop_file(a)).collect(),
        Err(_err) => Vec::new(),
    };
}
