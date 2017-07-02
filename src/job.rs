pub trait Runnable {
    fn run(self: Box<Self>);
}

impl<F: FnOnce()> Runnable for F {
    fn run(self: Box<F>) {
        (*self)()
    }
}

pub type Job = Box<Runnable + Send + 'static>;
