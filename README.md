# tower-afit

An experimental port of tower to use Rust's native async fn in trait feature combined with trait aliases and return type notation.

## Interesting observations

* We need the ability to return `-> impl Future` to [model](https://github.com/nikomatsakis/tower-afit/blob/master/src/filter.rs#L31) tower's synchronous filter trait `tower::filter::Filter`, which does the filtering *before* the future is returned (not as part of the returned future)