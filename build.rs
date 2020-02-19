use clap::Shell;
use std::env;

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut gen = |shell| app.gen_completions("projection", shell, out_dir.clone());
    gen(Shell::Bash);
    gen(Shell::Fish);
    gen(Shell::Zsh);
}
