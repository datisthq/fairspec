use fairspec_metadata::{Dataset, Result};
use fairspec_table::InferTableSchemaOptions;

use crate::actions::resource::infer::infer_resource;
use crate::helpers::concurrency::run_concurrent_tasks;

pub fn infer_dataset(dataset: &Dataset, options: &InferTableSchemaOptions) -> Result<Dataset> {
    let mut dataset = dataset.clone();

    if let Some(resources) = dataset.resources.take() {
        let entries: Vec<_> = resources.into_iter().enumerate().collect();
        let infer = |(index, resource)| infer_resource(&resource, Some(index + 1), options);
        let resources = run_concurrent_tasks(infer, entries, options.concurrency)
            .into_iter()
            .collect::<Result<Vec<_>>>()?;
        dataset.resources = Some(resources);
    }

    Ok(dataset)
}
