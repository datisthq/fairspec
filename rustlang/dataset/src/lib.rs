mod actions;
mod helpers;
mod models;
mod plugin;
mod plugins;

// models
pub use models::concurrency::ConcurrencyOptions;
pub use models::dataset::{SaveDatasetOptions, SaveDatasetResult};
pub use models::file::{FileDescription, FilePreview};
pub use models::file_dialect::InferFileDialectOptions;
pub use plugin::{DatasetPlugin, MetadataPlugin};

// actions
pub use actions::dataset::basepath::{get_common_local_basepath, get_dataset_basepath};
pub use actions::dataset::merge::merge_datasets;
pub use actions::file::copy::copy_file;
pub use actions::file::describe::describe_file;
pub use actions::file::infer::{infer_bytes, infer_hash, infer_integrity, infer_textual};
pub use actions::file::load::load_file;
pub use actions::file::path::{assert_local_path_vacant, get_is_local_path_exist};
pub use actions::file::prefetch::{prefetch_file, prefetch_files};
pub use actions::file::preview::preview_file;
pub use actions::file::save::save_file;
pub use actions::file::temp::{get_temp_file_path, write_temp_file};
pub use actions::file::validate::validate_file;
pub use actions::folder::create::create_folder;
pub use actions::folder::temp::get_temp_folder_path;
pub use actions::resource::save::{SaveFileCallback, SaveFileProps, save_resource_files};
pub use actions::stream::concat::concat_file_streams;
pub use actions::stream::load::{FileStream, load_file_stream};
pub use actions::stream::save::save_file_stream;

// plugins
pub use plugins::ckan::actions::dataset::load::load_dataset_from_ckan;
pub use plugins::ckan::actions::dataset::save::save_dataset_to_ckan;
pub use plugins::ckan::plugin::CkanPlugin;
pub use plugins::descriptor::plugin::DescriptorPlugin;
pub use plugins::folder::actions::dataset::load::load_dataset_from_folder;
pub use plugins::folder::actions::dataset::save::save_dataset_to_folder;
pub use plugins::folder::plugin::FolderPlugin;
pub use plugins::github::actions::dataset::load::load_dataset_from_github;
pub use plugins::github::actions::dataset::save::save_dataset_to_github;
pub use plugins::github::plugin::GithubPlugin;
pub use plugins::zenodo::actions::dataset::load::load_dataset_from_zenodo;
pub use plugins::zenodo::actions::dataset::save::save_dataset_to_zenodo;
pub use plugins::zenodo::plugin::ZenodoPlugin;
pub use plugins::zip::actions::dataset::load::load_dataset_from_zip;
pub use plugins::zip::actions::dataset::save::save_dataset_to_zip;
pub use plugins::zip::plugin::ZipPlugin;
