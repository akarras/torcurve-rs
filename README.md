torcurve-rs
=========================
[![docs](https://img.shields.io/docsrs/torcurve-rs)](https://docs.rs/torcurve-rs)
[![crates](https://img.shields.io/crates/v/torcurve-rs)](https://crates.io/crates/torcurve-rs)
![downloads](https://img.shields.io/crates/d/torcurve-rs)
[![issues](https://img.shields.io/github/issues/akarras/torcurve-rs)](https://github.com/akarras/torcurve-rs/issues)
[![license](https://img.shields.io/github/license/akarras/torcurve-rs)](https://github.com/akarras/torcurve-rs/blob/main/LICENSE)

A reusable implementation of [Toracdo's generalized, parameterized curve formula for generating lots of different curve shapes useful for easing, etc.](https://twitter.com/torcado/status/1490070852494372870)

Zero dependencies, and exposes a single function.

## Authorship
Note, I'm not the original creator of the formula. I just wanted a Rust implementation and figured I should share.

## Usage
Add `torcurve-rs = 0.1` to your dependencies

```rs
    use torcurve_rs::torcurve;
    fn run_code() {
        for i in 0..=10 {
             println!("curve: torcuve(i * 0.1, 3, 0, 0);
        }
    }
```

## Resources
* [desmos curve demo](https://www.desmos.com/calculator/hagtv9mxv6)
* [jsfiddle curve demo](https://jsfiddle.net/torcado194/5ocmt48a/latest)