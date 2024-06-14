// Lexer
#[cfg(any(test, bench))]
pub mod lexer;
#[cfg(not(any(test, bench)))]
pub(crate) mod lexer;

// Types
#[cfg(any(test, bench))]
pub mod types;
#[cfg(not(any(test, bench)))]
pub(crate) mod types;

// Benchmarks
#[cfg(bench)]
pub mod benchmarks;
mod benchmarks;