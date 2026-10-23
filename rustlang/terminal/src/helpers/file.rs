use fairspec_library::fairspec_metadata::get_data_first_path;

use super::resource::{create_value_error, select_resource};
use crate::models::output::Exit;
use crate::session::Session;

pub fn select_file(
    session: &Session,
    dataset: Option<&str>,
    resource: Option<&str>,
) -> Result<String, Exit> {
    let selected = select_resource(session, dataset, resource)?;
    session.task("Selecting file", || {
        get_data_first_path(&selected)
            .ok_or_else(|| create_value_error("Resource does not have files"))
    })
}
