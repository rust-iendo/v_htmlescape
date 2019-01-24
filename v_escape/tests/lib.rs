#![allow(dead_code)]
#[macro_use]
extern crate v_escape;

new_escape!(MyEscape, "60->foo");

macro_rules! test {
    ($name:ident, $escapes:expr, $escaped:expr) => {
        use std::borrow::Cow;

        let empty = "";
        let escapes = $escapes;
        let escaped = $escaped;
        let empty_heap = String::new();
        let short = "foobar";
        let string_long: &str = &short.repeat(1024);
        let string = $escapes.to_string();
        let cow = Cow::Owned($escapes.to_string());

        assert_eq!($name::from(empty).to_string(), empty);
        assert_eq!($name::from(escapes).to_string(), escaped);
        assert_eq!(escape(&empty_heap).to_string(), empty);
        assert_eq!(escape(&cow).to_string(), escaped);
        assert_eq!(escape(&string).to_string(), escaped);
        assert_eq!($name::from(string_long).to_string(), string_long);
        assert_eq!(
            $name::from(escapes.repeat(1024).as_ref()).to_string(),
            escaped.repeat(1024)
        );
        assert_eq!(
            $name::from([short, escapes, short].join("").as_ref()).to_string(),
            [short, escaped, short].join("")
        );
        assert_eq!(
            $name::from([escapes, short].join("").as_ref()).to_string(),
            [escaped, short].join("")
        );
        assert_eq!(
            $name::from(["f", escapes, short].join("").as_ref()).to_string(),
            ["f", escaped, short].join("")
        );
        assert_eq!(
            $name::from(["f", escapes].join("").as_ref()).to_string(),
            ["f", escaped].join("")
        );
        assert_eq!(
            $name::from(["fo", escapes].join("").as_ref()).to_string(),
            ["fo", escaped].join("")
        );
        assert_eq!(
            $name::from(["fo", escapes, "b"].join("").as_ref()).to_string(),
            ["fo", escaped, "b"].join("")
        );
        assert_eq!(
            $name::from(escapes.repeat(2).as_ref()).to_string(),
            escaped.repeat(2)
        );
        assert_eq!(
            $name::from(escapes.repeat(3).as_ref()).to_string(),
            escaped.repeat(3)
        );
        assert_eq!(
            $name::from(["f", &escapes.repeat(2)].join("").as_ref()).to_string(),
            ["f", &escaped.repeat(2)].join("")
        );
        assert_eq!(
            $name::from(["fo", &escapes.repeat(2)].join("").as_ref()).to_string(),
            ["fo", &escaped.repeat(2)].join("")
        );
        assert_eq!(
            $name::from(["fo", &escapes.repeat(2), "bar"].join("").as_ref()).to_string(),
            ["fo", &escaped.repeat(2), "bar"].join("")
        );
        assert_eq!(
            $name::from(["fo", &escapes.repeat(3), "bar"].join("").as_ref()).to_string(),
            ["fo", &escaped.repeat(3), "bar"].join("")
        );
        assert_eq!(
            $name::from([&escapes.repeat(3), "bar"].join("").as_ref()).to_string(),
            [&escaped.repeat(3), "bar"].join("")
        );
        assert_eq!(
            $name::from([short, &escapes.repeat(3), "bar"].join("").as_ref()).to_string(),
            [short, &escaped.repeat(3), "bar"].join("")
        );
        assert_eq!(
            $name::from([short, &escapes.repeat(5), "bar"].join("").as_ref()).to_string(),
            [short, &escaped.repeat(5), "bar"].join("")
        );
        assert_eq!(
            $name::from(
                [string_long, &escapes.repeat(13)]
                    .join("")
                    .repeat(1024)
                    .as_ref()
            )
            .to_string(),
            [string_long, &escaped.repeat(13)].join("").repeat(1024)
        );
    };
}

#[test]
fn test_escape() {
    test!(MyEscape, "<", "foo");
}

mod no_simd {
    mod a {
        new_escape!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            simd = false
        );

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
        }
    }

    mod b {
        new_escape!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            simd = false
        );

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
        }
    }
}

#[cfg(target_arch = "x86_64")]
mod no_avx {
    mod a {
        new_escape!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            avx = false
        );

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
        }
    }

    mod b {
        new_escape!(
            MyE,
            "65->a || 60->b || 61->c || 66->d || 80->e || 81->f",
            avx = false
        );

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
        }
    }
}

mod empty {
    new_escape!(MyE, "65->");

    #[test]
    fn test_escape() {
        test!(MyE, "A", "");
    }
}

#[cfg(target_arch = "x86_64")]
mod test_avx {
    mod a {
        // 3 ranges
        new_escape!(MyE, "65->a || 60->b || 61->c || 66->d || 80->e || 81->f");

        #[test]
        fn test_escape() {
            test!(MyE, "<=ABPQ", "bcadef");
        }
    }

    mod b {
        // 2 ranges and 1 escape
        new_escape!(MyE, "60->a || 61->b || 65->c || 80->d || 81->e");

        #[test]
        fn test_escape() {
            test!(MyE, "<=APQ", "abcde");
        }
    }

    mod c {
        // 1 range and 2 escapes
        new_escape!(MyE, "60->a || 65->c || 80->d || 62->e");

        #[test]
        fn test_escape() {
            test!(MyE, "<>AP", "aecd");
        }
    }

    mod d {
        // 3 escapes
        new_escape!(MyE, "60->a || 80->b || 65->c");

        #[test]
        fn test_escape() {
            test!(MyE, "<AP", "acb");
        }
    }

    mod e {
        // 2 ranges
        new_escape!(MyE, "60->a || 61->b || 81->c || 80->d || 62->e");

        #[test]
        fn test_escape() {
            test!(MyE, "<=>PQ", "abedc");
        }
    }

    mod f {
        // 1 range and 1 escape
        new_escape!(MyE, "60->a || 61->b || 80->c || 62->d");

        #[test]
        fn test_escape() {
            test!(MyE, "<=>P", "abdc");
        }
    }

    mod g {
        // 2 escapes
        new_escape!(MyE, "60->a || 80->b");

        #[test]
        fn test_escape() {
            test!(MyE, "<P", "ab");
        }
    }

    mod h {
        // 1 range
        new_escape!(MyE, "60->a || 61->b");

        #[test]
        fn test_escape() {
            test!(MyE, "<=", "ab");
        }
    }

    mod i {
        // 1 escapes
        new_escape!(MyE, "60->f");

        #[test]
        fn test_escape() {
            test!(MyE, "<", "f");
        }
    }
}

mod char_syntax {
    mod a {
        new_escape!(MyE, " ->f");

        #[test]
        fn test_escape() {
            test!(MyE, " ", "f");
        }
    }
}
