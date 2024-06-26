#![crate_name = "user"]
//@ aux-crate:rpitit=ret-pos-impl-trait-in-trait.rs
//@ edition:2021

// Test that we can correctly render cross-crate RPITITs.
// In particular, check that we don't render the internal associated type generated by
// their desugaring. We count the number of associated items and ensure that it is exactly one.
// This is more robust than checking for the absence of the associated type.

//@ has user/trait.Trait.html
//@ has - '//*[@id="method.create"]' 'fn create() -> impl Iterator<Item = u64>'
// The class "method" is used for all three kinds of associated items at the time of writing.
//@ count - '//*[@id="main-content"]//section[@class="method"]' 1
pub use rpitit::Trait;

//@ has user/struct.Basic.html
//@ has - '//*[@id="method.create"]' 'fn create() -> impl Iterator<Item = u64>'
//@ count - '//*[@id="trait-implementations-list"]//*[@class="impl-items"]' 1
pub use rpitit::Basic;

//@ has user/struct.Intermediate.html
//@ has - '//*[@id="method.create"]' 'fn create() -> Range<u64>'
//@ count - '//*[@id="trait-implementations-list"]//*[@class="impl-items"]' 1
pub use rpitit::Intermediate;

//@ has user/struct.Advanced.html
//@ has - '//*[@id="method.create"]' 'fn create() -> impl Iterator<Item = u64>'
//@ count - '//*[@id="trait-implementations-list"]//*[@class="impl-items"]' 1
pub use rpitit::Advanced;

// Regression test for issue #113929:

//@ has user/trait.Def.html
//@ has - '//*[@id="method.def"]' 'fn def<T>() -> impl Default'
pub use rpitit::Def;
