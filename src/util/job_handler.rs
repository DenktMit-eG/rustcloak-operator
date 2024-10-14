use crate::error::Result;
use chrono::{DateTime, Utc};
use futures::Future;
use std::hash::Hash;
use std::{collections::HashMap, time::Duration};
use tokio::task::JoinHandle;

#[derive(Debug, Default)]
pub struct JobHandler<ID> {
    jobs: HashMap<ID, JoinHandle<()>>,
}

impl<ID> JobHandler<ID>
where
    ID: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    pub fn schedule<F>(
        &mut self,
        id: &ID,
        time: DateTime<Utc>,
        job: F,
    ) -> Result<()>
    where
        F: Future<Output = ()> + Send + 'static,
        ID: Clone,
    {
        let timeout = if let Ok(timeout) = (time - chrono::Utc::now()).to_std()
        {
            timeout
        } else {
            Duration::from_secs(0)
        };

        let handle = tokio::spawn(async move {
            tokio::time::sleep(timeout).await;
            job.await;
        });

        if let Some(old_job) = self.jobs.insert(id.clone(), handle) {
            old_job.abort();
        }

        Ok(())
    }

    pub fn abort(&mut self, id: &ID) {
        if let Some(job) = self.jobs.remove(id) {
            job.abort();
        }
    }
}

impl<T> Drop for JobHandler<T> {
    fn drop(&mut self) {
        for (_, job) in self.jobs.drain() {
            job.abort();
        }
    }
}
