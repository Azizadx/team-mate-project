pub struct File {
    pub path: String,
    pub contents: String,
    pub rows: Vec<String>,
}

impl File {
    pub fn new(path: PathBuf) -> Result<Self> {
        let contents = std::fs::read_to_string(&path)?;

        let path_as_str = format!("{}", path.display());

        let rows = contents
            .lines()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        Ok(Self {
            path: path_as_str,
            contents,
            rows
        })
    }
}