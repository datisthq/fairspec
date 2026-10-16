from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .metadata import Quote
from .potential_dialects import PotentialDialect
from .table import Table
from .uniformity import calculate_tau0, calculate_tau1


class DialectScore(FairspecModel):
    dialect: PotentialDialect
    gamma: float
    tau0: float
    tau1: float
    num_fields: int
    is_uniform: bool


class FindBestDialectPreferences(FairspecModel):
    prefer_common_delimiters: bool
    prefer_double_quote: bool


class _QuoteEvidence(FairspecModel):
    quote_density: float
    boundary_matches: int
    internal_matches: int


def score_dialect(
    data: bytes,
    dialect: PotentialDialect,
) -> DialectScore:
    table = Table.parse(data, dialect)

    tau0 = calculate_tau0(table.field_counts)
    tau1 = calculate_tau1(table.field_counts, table.get_modal_field_count())
    num_fields = table.get_modal_field_count()
    is_uniform = table.is_uniform()
    num_rows = table.num_rows()

    quote_evidence = _analyze_quote_evidence(data, dialect)

    gamma = _calculate_gamma(
        tau0,
        tau1,
        num_rows,
        num_fields,
        is_uniform,
        dialect.delimiter,
        dialect.quote,
        quote_evidence,
    )

    return DialectScore(
        dialect=dialect,
        gamma=gamma,
        tau0=tau0,
        tau1=tau1,
        num_fields=num_fields,
        is_uniform=is_uniform,
    )


def _calculate_gamma(
    tau0: float,
    tau1: float,
    num_rows: int,
    num_fields: int,
    is_uniform: bool,
    delimiter: int,
    quote: Quote,
    quote_evidence: _QuoteEvidence,
) -> float:
    gamma = tau0 * 0.4 + tau1 * 0.6

    if is_uniform:
        gamma += 0.2

    if num_fields >= 2 and num_fields <= 50:
        gamma += 0.3
    elif num_fields == 1:
        gamma -= 1.0

    if num_rows >= 2:
        gamma += 0.05

    common_delimiters = [44, 9, 59, 124]
    if delimiter in common_delimiters:
        gamma += 0.15

    if quote.char is not None:
        quote_score = (
            quote_evidence.boundary_matches * 0.5
            + quote_evidence.quote_density * 0.3
            - quote_evidence.internal_matches * 0.2
        )

        gamma += max(0, min(0.2, quote_score))

        if quote.char == 34:
            gamma += 0.05
    else:
        if quote_evidence.quote_density < 0.01:
            gamma += 0.1

    return max(0, min(2, gamma))


def _analyze_quote_evidence(
    data: bytes,
    dialect: PotentialDialect,
) -> _QuoteEvidence:
    if dialect.quote.char is None:
        return _QuoteEvidence(
            quote_density=0,
            boundary_matches=0,
            internal_matches=0,
        )

    quote_char = dialect.quote.char
    delimiter_char = dialect.delimiter

    quote_count = 0
    boundary_matches = 0
    internal_matches = 0

    for i in range(len(data)):
        if data[i] == quote_char:
            quote_count += 1

            prev_char = data[i - 1] if i > 0 else None
            next_char = data[i + 1] if i < len(data) - 1 else None

            at_boundary = (
                prev_char is None
                or prev_char == delimiter_char
                or prev_char == 10
                or prev_char == 13
                or next_char is None
                or next_char == delimiter_char
                or next_char == 10
                or next_char == 13
            )

            if at_boundary:
                boundary_matches += 1
            else:
                internal_matches += 1

    quote_density = quote_count / len(data) if len(data) > 0 else 0

    return _QuoteEvidence(
        quote_density=quote_density,
        boundary_matches=boundary_matches,
        internal_matches=internal_matches,
    )


def find_best_dialect(
    scores: list[DialectScore],
    preferences: FindBestDialectPreferences,
) -> DialectScore:
    if len(scores) == 0:
        raise ValueError("No dialect scores provided")

    best_score = scores[0]

    for score in scores:
        current_gamma = score.gamma
        best_gamma = best_score.gamma

        if preferences.prefer_common_delimiters:
            common_delimiters = [44, 9, 59, 124]
            if score.dialect.delimiter in common_delimiters:
                current_gamma += 0.05
            if best_score.dialect.delimiter in common_delimiters:
                best_gamma += 0.05

        if preferences.prefer_double_quote:
            if score.dialect.quote.char is not None and score.dialect.quote.char == 34:
                current_gamma += 0.05
            if (
                best_score.dialect.quote.char is not None
                and best_score.dialect.quote.char == 34
            ):
                best_gamma += 0.05

        if current_gamma > best_gamma:
            best_score = score

    return best_score
