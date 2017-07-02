pub struct Options {
    pub addr: String,
    pub workers: usize,
}

impl Options {
    pub fn new(addr: String, workers: usize) -> Options {
        assert!(workers > 0);

        Options {
            addr,
            workers
        }
    }
}
