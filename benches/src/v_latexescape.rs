use criterion::Bencher;
use std::fmt::Write;
use v_latexescape::LateXEscape as Escape;

pub fn escaping(corpus: &'static [u8]) -> impl FnMut(&mut Bencher) + 'static {
    move |b: &mut Bencher| {
        let e = Escape::new(corpus);
        let mut writer = String::new();

        b.iter(|| {
            let _ = write!(writer, "{}", e);
        });
    }
}
