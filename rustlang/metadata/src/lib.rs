mod actions;
mod models;
mod settings;

pub use settings::FAIRSPEC_VERSION;

// models
pub use models::catalog::{Catalog, CatalogDataset};
pub use models::column::array::{
    ArrayColumn, ArrayColumnProperty, ArrayMissingValue, ArrayMissingValueItem,
};
pub use models::column::base::{
    ArrayBase, ArrayNullablePropertyType, BasePropertyType, BooleanBase,
    BooleanNullablePropertyType, IntegerBase, IntegerNullablePropertyType, NullPosition,
    NullablePropertyType, NumberBase, NumberNullablePropertyType, ObjectBase,
    ObjectNullablePropertyType, PropertyBase, StringBase, StringNullablePropertyType,
    StringOrInteger, ValueOrItem,
};
pub use models::column::base64::{Base64Column, Base64ColumnProperty, Base64Format};
pub use models::column::boolean::{
    BooleanColumn, BooleanColumnProperty, BooleanMissingValue, StringIntMissingValue,
};
pub use models::column::categorical::{
    CategoricalColumn, CategoricalColumnProperty, CategoricalFormat,
    IntegerCategoricalColumnProperty, IntegerCategory, IntegerCategoryItem,
    StringCategoricalColumnProperty, StringCategory, StringCategoryItem,
};
pub use models::column::column::{Column, ColumnProperty, ColumnType};
pub use models::column::date::{DateColumn, DateColumnProperty, DateFormat};
pub use models::column::date_time::{DateTimeColumn, DateTimeColumnProperty, DateTimeFormat};
pub use models::column::decimal::{DecimalColumn, DecimalColumnProperty, DecimalFormat};
pub use models::column::duration::{DurationColumn, DurationColumnProperty, DurationFormat};
pub use models::column::email::{EmailColumn, EmailColumnProperty, EmailFormat};
pub use models::column::geojson::{GeojsonColumn, GeojsonColumnProperty, GeojsonFormat};
pub use models::column::hex::{HexColumn, HexColumnProperty, HexFormat};
pub use models::column::integer::{
    BaseIntegerColumnProperty, IntegerColumn, IntegerColumnProperty, IntegerMissingValue,
    IntegerMissingValueItem,
};
pub use models::column::list::{ListColumn, ListColumnProperty, ListFormat, ListItemType};
pub use models::column::local_date_time::{
    LocalDateTimeColumn, LocalDateTimeColumnProperty, LocalDateTimeFormat,
};
pub use models::column::local_time::{LocalTimeColumn, LocalTimeColumnProperty, LocalTimeFormat};
pub use models::column::number::{
    BaseNumberColumnProperty, NumberColumn, NumberColumnProperty, NumberMissingValue,
    NumberMissingValueItem,
};
pub use models::column::object::{
    BaseObjectColumnProperty, ObjectColumn, ObjectColumnProperty, ObjectMissingValue,
    ObjectMissingValueItem,
};
pub use models::column::string::{
    BaseStringColumnProperty, StringColumn, StringColumnProperty, StringMissingValue,
    StringMissingValueItem,
};
pub use models::column::time::{TimeColumn, TimeColumnProperty, TimeFormat};
pub use models::column::topojson::{TopojsonColumn, TopojsonColumnProperty, TopojsonFormat};
pub use models::column::unknown::{
    NullPropertyType, UnknownColumn, UnknownColumnProperty, UnknownMissingValue,
    UnknownMissingValueItem,
};
pub use models::column::url::{UrlColumn, UrlColumnProperty, UrlFormat};
pub use models::column::wkb::{WkbColumn, WkbColumnProperty, WkbFormat};
pub use models::column::wkt::{WktColumn, WktColumnProperty, WktFormat};
pub use models::data::{Data, ResourceData, ResourceDataPath, ResourceDataValue};
pub use models::data_schema::{DataSchema, DataSchemaValidationResult, RenderDataSchemaOptions};
pub use models::datacite::alternate_identifier::{AlternateIdentifier, AlternateIdentifiers};
pub use models::datacite::common::{
    ContentTypeGeneral, ContributorType, CreatorNameType, DateType, DescriptionType,
    FunderIdentifierType, Latitude, Longitude, NumberType, RelatedIdentifierType, RelationType,
    TitleType,
};
pub use models::datacite::content_type::ContentTypes;
pub use models::datacite::contributor::{Contributor, Contributors};
pub use models::datacite::creator::{Creator, CreatorAffiliation, CreatorNameIdentifier, Creators};
pub use models::datacite::datacite::Datacite;
pub use models::datacite::date::{DataciteDate, DateValue, Dates};
pub use models::datacite::description::{DataciteDescription, Descriptions};
pub use models::datacite::formats::Formats;
pub use models::datacite::funding_reference::{FundingReference, FundingReferences};
pub use models::datacite::geo_location::{
    GeoLocation, GeoLocationBox, GeoLocationPoint, GeoLocationPolygonItem, GeoLocations,
};
pub use models::datacite::identifier::{
    DOI_PATTERN, DOI_PREFIX_PATTERN, DOI_SUFFIX_PATTERN, Doi, DoiPrefix, DoiSuffix,
};
pub use models::datacite::language::Language;
pub use models::datacite::publication_year::{PUBLICATION_YEAR_PATTERN, PublicationYear};
pub use models::datacite::publisher::Publisher;
pub use models::datacite::related_identifier::{RelatedIdentifier, RelatedIdentifiers};
pub use models::datacite::related_item::{RelatedItem, RelatedItemIdentifier, RelatedItems};
pub use models::datacite::rights::{Rights, RightsList};
pub use models::datacite::size::Sizes;
pub use models::datacite::subject::{Subject, Subjects};
pub use models::datacite::title::{Title, Titles};
pub use models::datacite::version::Version;
pub use models::dataset::{
    ConvertDatasetFromOptions, ConvertDatasetToOptions, Dataset, DatasetValidationResult,
    RenderDatasetOptions,
};
pub use models::descriptor::Descriptor;
pub use models::error::cell::{
    CellCheckError, CellConstError, CellEnumError, CellError, CellExclusiveMaximumError,
    CellExclusiveMinimumError, CellJsonError, CellMaxItemsError, CellMaxLengthError,
    CellMaximumError, CellMinItemsError, CellMinLengthError, CellMinimumError, CellMissingError,
    CellMultipleOfError, CellPatternError, CellTypeError, CellUniqueError,
};
pub use models::error::check::{CheckError, CheckErrorType, TableCheckError, TableCheckErrorType};
pub use models::error::column::{
    ColumnError, ColumnExtraError, ColumnMissingError, ColumnTypeError,
};
pub use models::error::data::{DataError, DataErrorType};
pub use models::error::error::FairspecError;
pub use models::error::file::{FileError, IntegrityError, TextualError};
pub use models::error::foreign_key::{ForeignKeyError, ForeignKeyErrorType};
pub use models::error::metadata::{MetadataError, MetadataErrorType};
pub use models::error::resource::{
    ExpectedResourceType, ResourceError, ResourceMissingError, ResourceTypeError,
};
pub use models::error::row::{RowCheckError, RowError, RowPrimaryKeyError, RowUniqueKeyError};
pub use models::error::table::TableError;
pub use models::exception::{Error, Result};
pub use models::file_dialect::arrow::{ArrowFileDialect, ArrowFormat};
pub use models::file_dialect::common::{
    ColumnNames, CommentPrefix, CommentRows, Delimiter, HeaderJoin, HeaderRows, JsonPointer,
    LineTerminator, NullSequence, QuoteChar, RowType, SheetName, SheetNumber, TableName,
};
pub use models::file_dialect::csv::{CsvFileDialect, CsvFormat};
pub use models::file_dialect::file_dialect::FileDialect;
pub use models::file_dialect::json::{JsonFileDialect, JsonFormat};
pub use models::file_dialect::jsonl::{JsonlFileDialect, JsonlFormat};
pub use models::file_dialect::ods::{OdsFileDialect, OdsFormat};
pub use models::file_dialect::parquet::{ParquetFileDialect, ParquetFormat};
pub use models::file_dialect::sqlite::{SqliteFileDialect, SqliteFormat};
pub use models::file_dialect::tsv::{TsvFileDialect, TsvFormat};
pub use models::file_dialect::unknown::UnknownFileDialect;
pub use models::file_dialect::xlsx::{XlsxFileDialect, XlsxFormat};
pub use models::foreign_key::{ForeignKey, ForeignKeyReference};
pub use models::host::HostPolicy;
pub use models::integrity::{Integrity, IntegrityType};
pub use models::json_schema::JsonSchema;
pub use models::path::{
    EXTERNAL_PATH_PATTERN, ExternalPath, INTERNAL_PATH_PATTERN, InternalPath, Path, PathOr,
};
pub use models::profile::{Profile, ProfileRegistry, ProfileRegistryItem, ProfileType};
pub use models::report::Report;
pub use models::resource::Resource;
pub use models::table_schema::{
    ConvertTableSchemaFromOptions, ConvertTableSchemaToOptions, RenderTableSchemaOptions,
    StringOrNumber, TableSchema, TableSchemaMissingValue, TableSchemaMissingValueItem,
    TableSchemaValidationResult,
};
pub use models::unique_key::UniqueKey;

// actions
pub use actions::column::create::create_column_from_property;
pub use actions::column::property::{
    get_base_property_type, get_column_properties, get_is_nullable_property_type,
    set_property_nullable,
};
pub use actions::data_schema::assert::assert_data_schema;
pub use actions::data_schema::load::load_data_schema;
pub use actions::data_schema::resolve::resolve_data_schema;
pub use actions::data_schema::save::save_data_schema;
pub use actions::data_schema::validate::validate_data_schema;
pub use actions::dataset::assert::assert_dataset;
pub use actions::dataset::denormalize::denormalize_dataset;
pub use actions::dataset::load::load_dataset_descriptor;
pub use actions::dataset::normalize::normalize_dataset;
pub use actions::dataset::save::save_dataset_descriptor;
pub use actions::dataset::validate::validate_dataset_descriptor;
pub use actions::descriptor::copy::copy_descriptor;
pub use actions::descriptor::general::get_is_descriptor;
pub use actions::descriptor::load::load_descriptor;
pub use actions::descriptor::parse::parse_descriptor;
pub use actions::descriptor::save::save_descriptor;
pub use actions::descriptor::stringify::stringify_descriptor;
pub use actions::descriptor::validate::validate_descriptor;
pub use actions::file_dialect::assert::assert_file_dialect;
pub use actions::file_dialect::infer::infer_file_dialect_format;
pub use actions::file_dialect::load::load_file_dialect;
pub use actions::file_dialect::resolve::resolve_file_dialect;
pub use actions::file_dialect::save::save_file_dialect;
pub use actions::file_dialect::support::get_supported_file_dialect;
pub use actions::file_dialect::validate::{FileDialectValidationResult, validate_file_dialect};
pub use actions::host::address::get_is_public_address;
pub use actions::host::policy::{get_host_policy, set_host_policy};
pub use actions::json::inspect::{JsonIssue, inspect_json};
pub use actions::json_schema::assert::assert_json_schema;
pub use actions::json_schema::inspect::inspect_json_schema;
pub use actions::json_schema::load::load_json_schema;
pub use actions::json_schema::resolve::resolve_json_schema;
pub use actions::json_schema::save::save_json_schema;
pub use actions::path::basepath::{get_basepath, resolve_basepath};
pub use actions::path::denormalize::denormalize_path;
pub use actions::path::general::{
    get_file_basename, get_file_extension, get_file_name, get_file_name_slug, get_file_protocol,
    get_is_remote_path,
};
pub use actions::path::normalize::normalize_path;
pub use actions::profile::assert::assert_profile;
pub use actions::profile::load::load_profile;
pub use actions::profile::registry::PROFILE_REGISTRY;
pub use actions::remote::agent::{create_http_agent, create_http_error};
pub use actions::report::create::create_report;
pub use actions::resource::data::{
    get_data_first_path, get_data_path, get_data_paths, get_data_records, get_data_value,
};
pub use actions::resource::denormalize::denormalize_resource;
pub use actions::resource::general::get_is_remote_resource;
pub use actions::resource::infer::infer_resource_name;
pub use actions::resource::normalize::normalize_resource;
pub use actions::table_schema::assert::assert_table_schema;
pub use actions::table_schema::column::get_columns;
pub use actions::table_schema::load::load_table_schema;
pub use actions::table_schema::resolve::resolve_table_schema;
pub use actions::table_schema::save::save_table_schema;
pub use actions::table_schema::validate::validate_table_schema;
