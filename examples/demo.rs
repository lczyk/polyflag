//! Runnable demo: showcases polyflag against a small fake `--quirks` cli.
//! Run with `make demo` or `cargo run --example demo`.

use std::collections::HashSet;

use polyflag::{KnownToken, apply, apply_with_callback, canonicalize, token};

const KNOWN: &[KnownToken] = &[
    token!("ascii"),
    token!("nocolor"; "no-color"),
    token!("noanimations";
        "no-animations",
        deprecated "no_animations",
        hidden "noanim",
    ),
];

fn main() {
    println!("== known tokens ==");
    for kt in KNOWN {
        println!("  {}", kt.canonical);
        for a in kt.aliases {
            println!("    alias {:?} ({:?})", a.spelling, a.status);
        }
    }

    println!("\n== layering occurrences ==");
    let mut set: HashSet<&'static str> = HashSet::new();
    for occ in ["ascii,no-color", "noanim", "-ascii"] {
        apply(occ, KNOWN, &mut set).unwrap();
        println!("  after {occ:>22} -> {set:?}");
    }

    println!("\n== deprecated callback ==");
    let mut set = HashSet::new();
    apply_with_callback(
        "no_animations,nocolor",
        KNOWN,
        &mut set,
        |spelling, canonical| {
            println!("  warning: {spelling:?} is deprecated, use {canonical:?}");
        },
    )
    .unwrap();
    println!("  set = {set:?}");

    println!("\n== canonicalize classification ==");
    for input in ["ascii", "no-color", "no_animations", "noanim", "bogus"] {
        match canonicalize(input, KNOWN) {
            Some(r) => println!("  {input:>15} -> {:?} ({:?})", r.canonical, r.kind),
            None => println!("  {input:>15} -> unknown"),
        }
    }

    println!("\n== unknown token error ==");
    let mut set = HashSet::new();
    let err = apply("ascii,bogus,nocolor", KNOWN, &mut set).unwrap_err();
    println!("  err = {err}");
    println!("  set = {set:?}  (partial state on error)");
}
