#[macro_export]
#[doc(hidden)]
/// Generate translations
///
/// Defining character interval from ASCII table to create bit masks from slice to be escaped
/// overflow above in addition
macro_rules! _v_escape_translations {
    ($la:expr, $ra:expr, $fb:expr, $fc:expr, 128, ) => {
        use std::arch::x86_64::{
            _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_or_si256,
            _mm256_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const B: i8 = $fb;
        const C: i8 = $fc;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);
        let v_b = _mm256_set1_epi8(B);
        let v_c = _mm256_set1_epi8(C);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_or_si256(
                    _mm256_or_si256(_mm256_cmpeq_epi8($a, v_b), _mm256_cmpeq_epi8($a, v_c)),
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                )
            }};
        }
    };
    ($fa:expr, $fb:expr, $fc:expr, 128, ) => {
        use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_or_si256, _mm256_set1_epi8};
        const A: i8 = $fa;
        const B: i8 = $fb;
        const C: i8 = $fc;

        let v_a = _mm256_set1_epi8(A);
        let v_b = _mm256_set1_epi8(B);
        let v_c = _mm256_set1_epi8(C);

        macro_rules! masking {
            ($a:ident) => {{
                _mm256_or_si256(
                    _mm256_or_si256(_mm256_cmpeq_epi8($a, v_a), _mm256_cmpeq_epi8($a, v_b)),
                    _mm256_cmpeq_epi8($a, v_c),
                )
            }};
        }
    };
    ($fa:expr, $fb:expr, 128, ) => {
        use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_or_si256, _mm256_set1_epi8};
        const A: i8 = $fa;
        const B: i8 = $fb;

        let v_a = _mm256_set1_epi8(A);
        let v_b = _mm256_set1_epi8(B);

        macro_rules! masking {
            ($a:ident) => {{
                _mm256_or_si256(_mm256_cmpeq_epi8($a, v_a), _mm256_cmpeq_epi8($a, v_b))
            }};
        }
    };
    ($fa:expr, 128, ) => {
        use std::arch::x86_64::{_mm256_cmpeq_epi8, _mm256_set1_epi8};
        const A: i8 = $fa;

        let v_a = _mm256_set1_epi8(A);

        macro_rules! masking {
            ($a:ident) => {{
                _mm256_cmpeq_epi8($a, v_a)
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $lc:expr, $rc:expr, ) => {
        use std::arch::x86_64::{
            _mm256_add_epi8, _mm256_cmpgt_epi8, _mm256_or_si256, _mm256_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;
        const TRANSLATION_C: i8 = ::std::i8::MAX - $rc;
        const BELOW_C: i8 = ::std::i8::MAX - ($rc - $lc) - 1;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);
        let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm256_set1_epi8(BELOW_B);
        let v_translation_c = _mm256_set1_epi8(TRANSLATION_C);
        let v_below_c = _mm256_set1_epi8(BELOW_C);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_or_si256(
                    _mm256_or_si256(
                        _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                        _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_b), v_below_b),
                    ),
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_c), v_below_c),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $c:expr, ) => {
        use std::arch::x86_64::{
            _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_or_si256,
            _mm256_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;
        const C: i8 = $c;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);
        let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm256_set1_epi8(BELOW_B);
        let v_c = _mm256_set1_epi8(C);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_or_si256(
                    _mm256_or_si256(
                        _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                        _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_b), v_below_b),
                    ),
                    _mm256_cmpeq_epi8($a, v_c),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, ) => {
        use std::arch::x86_64::{
            _mm256_add_epi8, _mm256_cmpgt_epi8, _mm256_or_si256, _mm256_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);
        let v_translation_b = _mm256_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm256_set1_epi8(BELOW_B);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_or_si256(
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_b), v_below_b),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $b:expr, ) => {
        use std::arch::x86_64::{
            _mm256_add_epi8, _mm256_cmpeq_epi8, _mm256_cmpgt_epi8, _mm256_or_si256,
            _mm256_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const B: i8 = $b;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);
        let v_b = _mm256_set1_epi8(B);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_or_si256(
                    _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a),
                    _mm256_cmpeq_epi8($a, v_b),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, ) => {
        use std::arch::x86_64::{_mm256_add_epi8, _mm256_cmpgt_epi8, _mm256_set1_epi8};
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;

        let v_translation_a = _mm256_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm256_set1_epi8(BELOW_A);

        macro_rules! masking {
            ($a:expr) => {{
                _mm256_cmpgt_epi8(_mm256_add_epi8($a, v_translation_a), v_below_a)
            }};
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// Generate translations
///
/// Defining character interval from ASCII table to create bit masks from slice to be escaped
/// overflow above in addition
macro_rules! _v_escape_translations_128 {
    ($la:expr, $ra:expr, $fb:expr, $fc:expr, 128, ) => {
        use std::arch::x86_64::{
            _mm_add_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const B: i8 = $fb;
        const C: i8 = $fc;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);
        let v_b = _mm_set1_epi8(B);
        let v_c = _mm_set1_epi8(C);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_or_si128(
                    _mm_or_si128(_mm_cmpeq_epi8($a, v_b), _mm_cmpeq_epi8($a, v_c)),
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                )
            }};
        }
    };
    ($fa:expr, $fb:expr, $fc:expr, 128, ) => {
        use std::arch::x86_64::{_mm_cmpeq_epi8, _mm_or_si128, _mm_set1_epi8};
        const A: i8 = $fa;
        const B: i8 = $fb;
        const C: i8 = $fc;

        let v_a = _mm_set1_epi8(A);
        let v_b = _mm_set1_epi8(B);
        let v_c = _mm_set1_epi8(C);

        macro_rules! masking_128 {
            ($a:ident) => {{
                _mm_or_si128(
                    _mm_or_si128(_mm_cmpeq_epi8($a, v_a), _mm_cmpeq_epi8($a, v_b)),
                    _mm_cmpeq_epi8($a, v_c),
                )
            }};
        }
    };
    ($fa:expr, $fb:expr, 128, ) => {
        use std::arch::x86_64::{_mm_cmpeq_epi8, _mm_or_si128, _mm_set1_epi8};
        const A: i8 = $fa;
        const B: i8 = $fb;

        let v_a = _mm_set1_epi8(A);
        let v_b = _mm_set1_epi8(B);

        macro_rules! masking_128 {
            ($a:ident) => {{
                _mm_or_si128(_mm_cmpeq_epi8($a, v_a), _mm_cmpeq_epi8($a, v_b))
            }};
        }
    };
    ($fa:expr, 128, ) => {
        use std::arch::x86_64::{_mm_cmpeq_epi8, _mm_set1_epi8};
        const A: i8 = $fa;

        let v_a = _mm_set1_epi8(A);

        macro_rules! masking_128 {
            ($a:ident) => {{
                _mm_cmpeq_epi8($a, v_a)
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $lc:expr, $rc:expr, ) => {
        use std::arch::x86_64::{_mm_add_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8};
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;
        const TRANSLATION_C: i8 = ::std::i8::MAX - $rc;
        const BELOW_C: i8 = ::std::i8::MAX - ($rc - $lc) - 1;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);
        let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm_set1_epi8(BELOW_B);
        let v_translation_c = _mm_set1_epi8(TRANSLATION_C);
        let v_below_c = _mm_set1_epi8(BELOW_C);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_or_si128(
                    _mm_or_si128(
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_b), v_below_b),
                    ),
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_c), v_below_c),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $c:expr, ) => {
        use std::arch::x86_64::{
            _mm_add_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;
        const C: i8 = $c;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);
        let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm_set1_epi8(BELOW_B);
        let v_c = _mm_set1_epi8(C);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_or_si128(
                    _mm_or_si128(
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                        _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_b), v_below_b),
                    ),
                    _mm_cmpeq_epi8($a, v_c),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, ) => {
        use std::arch::x86_64::{_mm_add_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8};
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const TRANSLATION_B: i8 = ::std::i8::MAX - $rb;
        const BELOW_B: i8 = ::std::i8::MAX - ($rb - $lb) - 1;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);
        let v_translation_b = _mm_set1_epi8(TRANSLATION_B);
        let v_below_b = _mm_set1_epi8(BELOW_B);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_or_si128(
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_b), v_below_b),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, $b:expr, ) => {
        use std::arch::x86_64::{
            _mm_add_epi8, _mm_cmpeq_epi8, _mm_cmpgt_epi8, _mm_or_si128, _mm_set1_epi8,
        };
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;
        const B: i8 = $b;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);
        let v_b = _mm_set1_epi8(B);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_or_si128(
                    _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a),
                    _mm_cmpeq_epi8($a, v_b),
                )
            }};
        }
    };
    ($la:expr, $ra:expr, ) => {
        use std::arch::x86_64::{_mm_add_epi8, _mm_cmpgt_epi8, _mm_set1_epi8};
        const TRANSLATION_A: i8 = ::std::i8::MAX - $ra;
        const BELOW_A: i8 = ::std::i8::MAX - ($ra - $la) - 1;

        let v_translation_a = _mm_set1_epi8(TRANSLATION_A);
        let v_below_a = _mm_set1_epi8(BELOW_A);

        macro_rules! masking_128 {
            ($a:expr) => {{
                _mm_cmpgt_epi8(_mm_add_epi8($a, v_translation_a), v_below_a)
            }};
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// Generate mask bodies callback
///
/// Defining exact match or false positive
/// ## The following macros must be defined
/// * `mask_bodies_callback($callback:ident)`
///
macro_rules! _v_escape_mask_bodies_escaping {
    ($la:expr, $ra:expr, $fb:expr, $fc:expr, 128, ) => {
        mask_bodies_callback!(_v_escape_bodies);
    };
    ($fa:expr, $fb:expr, $fc:expr, 128, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
    ($fa:expr, $fb:expr, 128, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
    ($fa:expr, 128, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $lc:expr, $rc:expr, ) => {
        mask_bodies_callback!(_v_escape_bodies);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, $c:expr, ) => {
        mask_bodies_callback!(_v_escape_bodies);
    };
    ($la:expr, $ra:expr, $lb:expr, $rb:expr, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
    ($la:expr, $ra:expr, $b:expr, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
    ($la:expr, $ra:expr, ) => {
        mask_bodies_callback!(_v_escape_bodies_exact);
    };
}
