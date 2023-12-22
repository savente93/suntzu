use crate::job::JobId;

pub enum WorkerStatus {
    Idle,
    Working,
    Errored,
}

pub struct Worker {
    current_job_id: Option<JobId>,
    status: WorkerStatus,
}
