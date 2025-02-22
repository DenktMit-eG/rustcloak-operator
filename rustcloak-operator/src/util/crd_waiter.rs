use crate::error::Error;
use kube::Resource;

pub async fn wait_for_crd<R, T>(_client: &kube::Client) -> Result<(), Error>
where
    R: Resource<DynamicType: From<()>>,
{
    Ok(())
}
