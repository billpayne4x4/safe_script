extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crate::lexer::Lexer;

fn lexer_benchmark(c: &mut Criterion) {
    let mut script = String::new();
    let mut test = String::new();

    script.push_str(": ; . = == =+ =- =* =/ + ++ += - -- -= ! != * / /= \\ % $ @ # ^ ( ) { } [ ] , > >= < <= & && | || ");
    script.push_str("int float string bool char array null ");
    script.push_str("variableName anotherVar _privateVar var0123 ");
    script.push_str("if else for while switch case fn writeLn convertTo ");
    script.push_str("123 123.456 'a' \"string\" true false ");
    println!("The following script will be repeated 1000 times and tokenized by the lexer: {}", script);

    for i in 0..1000 {
        test.push_str(&script);
    }

    c.bench_function("lexer_tokenize", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(&script));
            lexer.tokenize()
        })
    });
}

criterion_group!(benches, lexer_benchmark);
criterion_main!(benches);
