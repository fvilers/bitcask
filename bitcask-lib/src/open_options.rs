#[derive(Debug, Default)]
pub struct OpenOptions {
    write: bool,
    sync: bool,
}

impl OpenOptions {
    pub fn new() -> Self {
        OpenOptions::default()
    }

    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }

    pub fn sync(&mut self, sync: bool) -> &mut Self {
        self.write = self.write || sync;
        self.sync = sync;
        self
    }
}
