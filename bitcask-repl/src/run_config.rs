pub struct RunConfig {
    pub directory_name: String,
}

impl RunConfig {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Usage: bitcask-repl <directory_name>");
        }

        let directory_name = args[1].to_owned();

        Ok(Self { directory_name })
    }
}
