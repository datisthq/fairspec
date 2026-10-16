from .actions.dataset.basepath import get_common_local_basepath, get_dataset_basepath
from .actions.dataset.merge import merge_datasets
from .actions.file.copy import copy_file
from .actions.file.describe import describe_file
from .models.file import FileDescription
from .actions.file.infer import infer_bytes, infer_hash, infer_integrity, infer_textual
from .actions.file.load import load_file
from .actions.file.path import assert_local_path_vacant, get_is_local_path_exist
from .actions.file.prefetch import prefetch_file, prefetch_files
from .actions.file.save import save_file
from .actions.file.temp import get_temp_file_path, write_temp_file
from .actions.file.validate import validate_file
from .actions.folder.create import create_folder
from .actions.folder.temp import get_temp_folder_path
from .actions.resource.save import SaveFileCallback, SaveFileProps, save_resource_files
from .actions.stream.concat import concat_file_streams
from .actions.stream.load import load_file_stream
from .actions.stream.save import save_file_stream
from .models.concurrency import ConcurrencyOptions
from .models.dataset import SaveDatasetOptions
from .models.file_dialect import InferFileDialectOptions
from .models.dataset import SaveDatasetResult
from .plugin import DatasetPlugin
from .plugins.descriptor import DescriptorPlugin
from .plugins.ckan import CkanPlugin, load_dataset_from_ckan, save_dataset_to_ckan
from .plugins.folder import (
    FolderPlugin,
    load_dataset_from_folder,
    save_dataset_to_folder,
)
from .plugins.github import (
    GithubPlugin,
    load_dataset_from_github,
    save_dataset_to_github,
)
from .plugins.zenodo import (
    ZenodoPlugin,
    load_dataset_from_zenodo,
    save_dataset_to_zenodo,
)
from .plugins.zip import ZipPlugin, load_dataset_from_zip, save_dataset_to_zip
from fairspec_metadata.plugin import MetadataPlugin

__all__ = [
    "CkanPlugin",
    "ConcurrencyOptions",
    "DatasetPlugin",
    "DescriptorPlugin",
    "FileDescription",
    "FolderPlugin",
    "GithubPlugin",
    "InferFileDialectOptions",
    "MetadataPlugin",
    "SaveDatasetOptions",
    "SaveDatasetResult",
    "SaveFileCallback",
    "SaveFileProps",
    "ZenodoPlugin",
    "ZipPlugin",
    "assert_local_path_vacant",
    "concat_file_streams",
    "copy_file",
    "create_folder",
    "describe_file",
    "get_common_local_basepath",
    "get_dataset_basepath",
    "get_is_local_path_exist",
    "get_temp_file_path",
    "get_temp_folder_path",
    "infer_bytes",
    "infer_hash",
    "infer_integrity",
    "infer_textual",
    "load_dataset_from_ckan",
    "load_dataset_from_folder",
    "load_dataset_from_github",
    "load_dataset_from_zenodo",
    "load_dataset_from_zip",
    "load_file",
    "load_file_stream",
    "merge_datasets",
    "prefetch_file",
    "prefetch_files",
    "save_dataset_to_ckan",
    "save_dataset_to_folder",
    "save_dataset_to_github",
    "save_dataset_to_zenodo",
    "save_dataset_to_zip",
    "save_file",
    "save_file_stream",
    "save_resource_files",
    "validate_file",
    "write_temp_file",
]
