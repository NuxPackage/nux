extern crate clap;
use std::{env, process};

use clap::Shell;
include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = app::make_clap_app();
    app.gen_completions(
        "nux",       // We need to specify the bin name manually
        Shell::Bash, // Then say which shell to build completions for
        &outdir,
    );

    app.gen_completions(
        "nux",      // We need to specify the bin name manually
        Shell::Zsh, // Then say which shell to build completions for
        &outdir,
    );

    app.gen_completions(
        "nux",       // We need to specify the bin name manually
        Shell::Fish, // Then say which shell to build completions for
        &outdir,
    );

    // Gen man pages...

    if let Err(err) = process::Command::new("asciidoctor").output() {
        eprintln!("Could not run 'asciidoctor' binary, no docs will be generated.");
        eprintln!("Error from running 'asciidoctor': {}", err);
        return;
    }

    let cwd = env::current_dir().expect("Current dir failed");
    let dest = std::path::Path::new(&outdir);
    let manpagesource = cwd
        .join("manpage")
        .join(std::path::Path::new("nux.asciidoc"));
    let result = process::Command::new("asciidoctor")
        .arg("--doctype")
        .arg("manpage")
        .arg("--backend")
        .arg("manpage")
        .arg(manpagesource)
        .arg("-D")
        .arg(dest)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    if !result.success() {
        let msg = format!("'asciidoctor' failed with exit code {:?}", result.code());
    }

    // Then say where write the completions to
}
