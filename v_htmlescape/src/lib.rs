//! # Quick start
//!
//! ```
//! extern crate v_htmlescape;
//! use v_htmlescape::escape;
//!
//! print!("{}", escape("foo<bar"));
//! ```
//!

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate v_escape;

/// Without simd optimizations
pub mod fallback {
    new_escape!(
        HTMLEscape,
        "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f;",
        simd = false
    );
}

cfg_if! {
    if #[cfg(all(v_htmlescape_simd, v_htmlescape_avx))] {
        new_escape!(
            HTMLEscape,
            "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f;",
            avx = true, simd = true
        );
    } else if #[cfg(all(v_htmlescape_simd, v_htmlescape_sse))] {
        new_escape!(
            HTMLEscape,
            "60->&lt; || 62->&gt; || 38->&amp; || 34->&quot; || 39->&#x27; || 47->&#x2f;",
            avx = false, simd = true
        );
    } else {
        pub use self::fallback::*;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_escape() {
        use super::HTMLEscape;

        let empty = "";
        assert_eq!(HTMLEscape::from(empty).to_string(), empty);

        assert_eq!(HTMLEscape::from("").to_string(), "");
        assert_eq!(HTMLEscape::from("<&>").to_string(), "&lt;&amp;&gt;");
        assert_eq!(HTMLEscape::from("bar&").to_string(), "bar&amp;");
        assert_eq!(HTMLEscape::from("<foo").to_string(), "&lt;foo");
        assert_eq!(HTMLEscape::from("bar&h").to_string(), "bar&amp;h");
        assert_eq!(
            HTMLEscape::from("// my <html> is \"unsafe\" & should be 'escaped'").to_string(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; \
             should be &#x27;escaped&#x27;"
        );
        assert_eq!(
            HTMLEscape::from(
                "// my <html> is \"unsafe\" & should be 'escaped'"
                    .repeat(10_000)
                    .as_ref()
            )
            .to_string(),
            "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; \
             should be &#x27;escaped&#x27;"
                .repeat(10_000)
        );
    }
}
