use crate::actions::path::_shared::slugify::slugify_text;
use crate::actions::path::basepath::get_basepath;
use crate::actions::path::normalize::normalize_path;

// Expected values come from the Python library, which matches @sindresorhus/slugify
#[test]
fn test_slugify_text() {
    let cases = [
        ("XMLHttpRequest", "xml-http-request"),
        ("ÆØÅ æøå", "ae-oa-aeoa"),
        ("Ĳsselmeer", "i-jsselmeer"),
        ("a&b", "a-and-b"),
        ("🦄 love ♥", "unicorn-love-love"),
        ("naïve résumé", "naive-resume"),
        ("Þór", "thor"),
        ("e\u{301}", "e"),
        ("ﬁle", "file"),
        ("  lead--x  ", "lead-x"),
    ];
    for (text, expected) in cases {
        assert_eq!(slugify_text(text, "-"), expected, "{text}");
        assert_eq!(
            slugify_text(text, "_"),
            expected.replace('-', "_"),
            "{text}"
        );
    }
}

#[test]
fn test_urls_unparse_like_python() {
    assert_eq!(
        get_basepath("http:example.com/a/b.csv"),
        "http:///example.com/a"
    );
    assert_eq!(
        normalize_path("a/../b.csv", Some("http://e.com/x")).unwrap(),
        "http://e.com/x/b.csv"
    );
    assert_eq!(normalize_path("./a/./b.csv", None).unwrap(), "a/b.csv");
    assert_eq!(
        normalize_path("HTTP://E.com/a?", None).unwrap(),
        "http://E.com/a"
    );
}
