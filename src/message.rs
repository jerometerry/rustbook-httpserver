use job::Job;

pub enum Message {
    NewJob(Job),
    Terminate,
}
