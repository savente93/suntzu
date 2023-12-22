use anyhow::Result;

pub type JobId = String;
pub enum JobStatus {
    Submitted,
    Scheduled,
    Suspended,
    Working,
    Finished,
}
pub struct Job {
    id: JobId,
    dependencies: Vec<JobId>,
    retry_cnt: usize,
    status: JobStatus,
}

impl Job {
    pub fn execute(mut self) -> Result<()> {
        todo!()
    }
}
