use job::Job;

pub enum Message {
    RunJob(Job),
    Terminate,
}
