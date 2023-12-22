use crate::job::Job;
use anyhow::Result;

struct Scheduler {
    queue: JobQueue,
}

pub enum Priority {
    High,
    Medium,
    Low,
}

impl Scheduler {
    pub fn schedule(job: Job, prio: Priority) -> Result<()> {
        todo!()
    }
}
