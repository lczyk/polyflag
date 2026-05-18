//! Runnable demo: showcases polyflag against a small fake `--quirks` cli.
//! Run with `make demo` or `cargo run --example demo`.

use std::collections::HashSet;

use polyflag::{
    KnownToken, apply, apply_env_for_flag, apply_with_callback, canonicalize, debug_check_known,
    defaults, format_known_for_help, token,
};

const KNOWN: &[KnownToken] = &[
    token!(default "color"; "colour"),
    token!(default "animations"; "anim"),
    token!("ascii"),
    token!("nocolor"; "no-color"),
    token!("noanimations";
        "no-animations",
        deprecated "no_animations",
        hidden "noanim",
    ),
    token!(weird "ext.beta"; "ext/beta", "ext:beta"),
];

fn main() {
    debug_check_known!(KNOWN);

    println!("== known tokens ==");
    for kt in KNOWN {
        println!(
            "  {}{}{}",
            kt.canonical,
            if kt.default { " [default]" } else { "" },
            if kt.allow_weird { " [weird]" } else { "" },
        );
        for a in kt.aliases {
            println!("    alias {:?} ({:?})", a.spelling, a.status);
        }
    }

    println!("\n== help string ==");
    println!("  {}", format_known_for_help(KNOWN));

    println!("\n== defaults seed ==");
    let seed = defaults(KNOWN);
    println!("  {seed:?}");

    println!("\n== layering occurrences (+ explicit add, - remove) ==");
    let mut set = defaults(KNOWN);
    for occ in ["ascii,no-color", "+noanim", "-ascii", "-color,+ext.beta"] {
        apply(occ, KNOWN, &mut set).unwrap();
        println!("  after {occ:>24} -> {set:?}");
    }

    println!("\n== env-var seeding ==");
    // SAFETY: single-threaded demo.
    unsafe { std::env::set_var("DEMO_QUIRKS", "colour,+ascii") };
    let mut set: HashSet<&'static str> = HashSet::new();
    apply_env_for_flag("demo", "quirks", KNOWN, &mut set).unwrap();
    println!("  DEMO_QUIRKS=colour,+ascii -> {set:?}");
    unsafe { std::env::remove_var("DEMO_QUIRKS") };

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
    for input in [
        "ascii",
        "colour",
        "no-color",
        "no_animations",
        "noanim",
        "ext/beta",
        "bogus",
    ] {
        match canonicalize(input, KNOWN) {
            Some(r) => println!("  {input:>15} -> {:?} ({:?})", r.canonical, r.kind),
            None => println!("  {input:>15} -> unknown"),
        }
    }

    println!("\n== weird-name token ==");
    let mut set = HashSet::new();
    apply("ext.beta,+ext:beta,-ext/beta", KNOWN, &mut set).unwrap();
    println!("  set = {set:?}");

    println!("\n== unknown token error ==");
    let mut set = HashSet::new();
    let err = apply("ascii,bogus,nocolor", KNOWN, &mut set).unwrap_err();
    println!("  err = {err}");
    println!("  set = {set:?}  (partial state on error)");
}
