use fairspec_metadata::{CategoricalColumn, CategoricalColumnProperty, ValueOrItem};
use serde_json::Value;

/// The category values (strings or integers) and their labels, a bare value labelling itself.
pub fn get_categorical_values_and_labels(column: &CategoricalColumn) -> (Vec<Value>, Vec<String>) {
    let mut values = Vec::new();
    let mut labels = Vec::new();
    match &column.property {
        CategoricalColumnProperty::String(property) => {
            for item in property.categories.iter().flatten() {
                match item {
                    ValueOrItem::Value(value) => {
                        values.push(Value::from(value.clone()));
                        labels.push(value.clone());
                    }
                    ValueOrItem::Item(item) => {
                        values.push(Value::from(item.value.clone()));
                        labels.push(item.label.clone());
                    }
                }
            }
        }
        CategoricalColumnProperty::Integer(property) => {
            for item in property.categories.iter().flatten() {
                match item {
                    ValueOrItem::Value(value) => {
                        values.push(Value::from(*value));
                        labels.push(value.to_string());
                    }
                    ValueOrItem::Item(item) => {
                        values.push(Value::from(item.value));
                        labels.push(item.label.clone());
                    }
                }
            }
        }
    }
    (values, labels)
}
