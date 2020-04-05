pub enum PathSoftware {
    Home,
    Root,
}

impl PathSoftware {
    pub fn as_string(&self) -> String {
        match self {
            &PathSoftware::Root => String::from("/usr/share/applications/"),
            &PathSoftware::Home => String::from("~/.local/share/applications/"),
        }
    }
}
