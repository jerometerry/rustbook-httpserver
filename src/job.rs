pub trait Executor {
    fn execute(self: Box<Self>);
}

impl<F: FnOnce()> Executor for F {
    fn execute(self: Box<F>) {
        (*self)()
    }
}

pub type Job = Box<Executor + Send + 'static>;
