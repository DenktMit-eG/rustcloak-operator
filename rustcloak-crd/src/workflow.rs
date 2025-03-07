pub struct InitWorkflow {
    pub workflow: http::Method,
    pub mount_path: &'static str,
}

impl From<&'static str> for InitWorkflow {
    fn from(mount_path: &'static str) -> Self {
        InitWorkflow {
            workflow: http::Method::POST,
            mount_path,
        }
    }
}
