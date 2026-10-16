from __future__ import annotations

import pytest

from ..metadata import LineTerminator, Quote
from ..potential_dialects import PotentialDialect
from ..score import FindBestDialectPreferences, find_best_dialect, score_dialect


class TestScoreDialect:
    def test_comma_delimited_csv_scores_highly(self):
        csv = "a,b,c\n1,2,3\n4,5,6"
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
        )
        score = score_dialect(csv.encode(), dialect)

        assert score.gamma > 0.5
        assert score.is_uniform is True
        assert score.num_fields == 3

    def test_incorrect_delimiter_scores_poorly(self):
        csv = "a,b,c\n1,2,3\n4,5,6"
        dialect = PotentialDialect(
            delimiter=9, quote=Quote(), line_terminator=LineTerminator.LF
        )
        score = score_dialect(csv.encode(), dialect)

        assert score.gamma < 0.5
        assert score.num_fields == 1

    def test_detect_non_uniform(self):
        csv = "a,b\n1,2\n3,4,5"
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
        )
        score = score_dialect(csv.encode(), dialect)

        assert score.is_uniform is False

    def test_calculates_tau0_and_tau1(self):
        csv = "a,b,c\n1,2,3\n4,5,6"
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
        )
        score = score_dialect(csv.encode(), dialect)

        assert score.tau0 > 0
        assert score.tau1 > 0

    def test_handle_quoted_fields(self):
        csv = '"a","b,c","d"\n"1","2,3","4"'
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(char=34), line_terminator=LineTerminator.LF
        )
        score = score_dialect(csv.encode(), dialect)

        assert score.num_fields == 3
        assert score.is_uniform is True


class TestFindBestDialect:
    def test_select_highest_scoring(self):
        csv = "a,b,c\n1,2,3\n4,5,6"
        data = csv.encode()

        dialects = [
            PotentialDialect(
                delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
            ),
            PotentialDialect(
                delimiter=9, quote=Quote(), line_terminator=LineTerminator.LF
            ),
            PotentialDialect(
                delimiter=59, quote=Quote(), line_terminator=LineTerminator.LF
            ),
        ]

        scores = [score_dialect(data, d) for d in dialects]
        best = find_best_dialect(
            scores,
            FindBestDialectPreferences(
                prefer_common_delimiters=True, prefer_double_quote=True
            ),
        )

        assert best.dialect.delimiter == 44

    def test_prefer_common_delimiters(self):
        csv = "a,b\n1,2\n3,4"
        data = csv.encode()

        dialects = [
            PotentialDialect(
                delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
            ),
            PotentialDialect(
                delimiter=94, quote=Quote(), line_terminator=LineTerminator.LF
            ),
        ]

        scores = [score_dialect(data, d) for d in dialects]
        scores[0].gamma = 0.7
        scores[1].gamma = 0.71

        best = find_best_dialect(
            scores,
            FindBestDialectPreferences(
                prefer_common_delimiters=True, prefer_double_quote=False
            ),
        )

        assert best.dialect.delimiter == 44

    def test_prefer_double_quote(self):
        csv = '"a","b"\n"1","2"'
        data = csv.encode()

        dialects = [
            PotentialDialect(
                delimiter=44, quote=Quote(char=34), line_terminator=LineTerminator.LF
            ),
            PotentialDialect(
                delimiter=44, quote=Quote(char=39), line_terminator=LineTerminator.LF
            ),
        ]

        scores = [score_dialect(data, d) for d in dialects]

        best = find_best_dialect(
            scores,
            FindBestDialectPreferences(
                prefer_common_delimiters=False, prefer_double_quote=True
            ),
        )

        if best.dialect.quote.char is not None:
            assert best.dialect.quote.char == 34

    def test_empty_scores_throws(self):
        with pytest.raises(ValueError):
            find_best_dialect(
                [],
                FindBestDialectPreferences(
                    prefer_common_delimiters=True, prefer_double_quote=True
                ),
            )

    def test_single_score(self):
        csv = "a,b\n1,2"
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
        )
        score = score_dialect(csv.encode(), dialect)
        best = find_best_dialect(
            [score],
            FindBestDialectPreferences(
                prefer_common_delimiters=True, prefer_double_quote=True
            ),
        )

        assert best is score


class TestGammaCalculation:
    def test_reward_uniform_tables(self):
        uniform_csv = "a,b,c\n1,2,3\n4,5,6\n7,8,9"
        non_uniform_csv = "a,b\n1,2,3\n4,5\n6,7,8,9"
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
        )

        uniform_score = score_dialect(uniform_csv.encode(), dialect)
        non_uniform_score = score_dialect(non_uniform_csv.encode(), dialect)

        assert uniform_score.gamma > non_uniform_score.gamma

    def test_reward_reasonable_field_counts(self):
        csv_2 = "a,b\n1,2\n3,4"
        csv_100 = (
            "a," + "b," * 98 + "z\n" + "1," + "2," * 98 + "3\n" + "4," + "5," * 98 + "6"
        )
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
        )

        score_2 = score_dialect(csv_2.encode(), dialect)
        score_100 = score_dialect(csv_100.encode(), dialect)

        assert score_2.num_fields == 2
        assert score_100.num_fields > 50

    def test_cap_gamma_at_2(self):
        csv = "a,b,c\n1,2,3\n4,5,6\n7,8,9\n10,11,12"
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
        )
        score = score_dialect(csv.encode(), dialect)

        assert score.gamma <= 2

    def test_gamma_non_negative(self):
        csv = "random text without structure"
        dialect = PotentialDialect(
            delimiter=44, quote=Quote(), line_terminator=LineTerminator.LF
        )
        score = score_dialect(csv.encode(), dialect)

        assert score.gamma >= 0
