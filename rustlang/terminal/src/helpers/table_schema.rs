use fairspec_library::fairspec_table::{InferTableSchemaOptions, TableSchemaOptions};

use crate::params::common::get_choice_text;
use crate::params::table_schema::TableSchemaParams;

pub fn create_infer_table_schema_options(params: &TableSchemaParams) -> InferTableSchemaOptions {
    let text = |value: &Option<String>| value.clone().filter(|value| !value.is_empty());
    InferTableSchemaOptions {
        table_schema_options: TableSchemaOptions {
            column_types: params
                .column_types
                .clone()
                .filter(|types| !types.is_empty()),
            column_properties: params
                .column_properties
                .clone()
                .filter(|properties| !properties.is_empty()),
            missing_values: params
                .missing_values
                .clone()
                .filter(|values| !values.is_empty()),
            decimal_char: text(&params.decimal_char),
            group_char: text(&params.group_char),
            true_values: params
                .true_values
                .clone()
                .filter(|values| !values.is_empty()),
            false_values: params
                .false_values
                .clone()
                .filter(|values| !values.is_empty()),
            datetime_format: text(&params.datetime_format),
            date_format: text(&params.date_format),
            time_format: text(&params.time_format),
            array_type: params.array_type.as_ref().map(get_choice_text),
            list_delimiter: text(&params.list_delimiter),
            list_item_type: params.list_item_type.as_ref().map(get_choice_text),
            ..TableSchemaOptions::default()
        },
        sample_rows: params.sample_rows,
        confidence: params.confidence,
        comma_decimal: params.comma_decimal.then_some(true),
        month_first: params.month_first.then_some(true),
        keep_strings: params.keep_strings.then_some(true),
        ..InferTableSchemaOptions::default()
    }
}
