use polars::prelude::*;

/// Strip the text around a number: the span from the first to the last digit is kept, and a
/// minus in the stripped prefix makes it negative. Text inside the span stays, so it fails parsing
pub fn strip_number_text(column_expr: Expr, decimal_char: Option<&str>) -> Expr {
    let decimal = decimal_char
        .filter(|c| !c.is_empty())
        .map(|c| format!("(?:{})?", regex::escape(c)))
        .unwrap_or_default();
    let span = format!("({decimal}[0-9](?:.*[0-9])?)[^0-9]*$");
    let negative = format!("^[^0-9]*-[^0-9]*?{span}");
    let positive = format!("^[^0-9]*?{span}");

    when(
        column_expr
            .clone()
            .str()
            .contains(lit(negative.clone()), true),
    )
    .then(
        column_expr
            .clone()
            .str()
            .replace(lit(negative), lit("-$1"), false),
    )
    .otherwise(column_expr.str().replace(lit(positive), lit("$1"), false))
}
