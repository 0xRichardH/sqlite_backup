use anyhow::Result;

pub struct Argument {
    pub source_path: String,
}

impl Argument {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if let Some(source_path) = args.get(1) {
            let argument = Self {
                source_path: source_path.clone(),
            };
            Ok(argument)
        } else {
            Err("No source path provided.")
        }
    }
}
