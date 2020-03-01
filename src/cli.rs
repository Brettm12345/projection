use clap::{
    App,
    AppSettings::{ColoredHelp, DeriveDisplayOrder, InferSubcommands},
    Arg, SubCommand,
};

pub fn build_cli() -> App<'static, 'static> {
    let arg = |name| Arg::with_name(name).long(name);
    let sub = SubCommand::with_name;
    App::new("Projection")
        .author("brettm12345")
        .version("0.2.0")
        .long_about("The next generation project manager for the shell")
        .settings(&[ColoredHelp, DeriveDisplayOrder, InferSubcommands])
        .args(&[
            arg("no-confirm")
                .short("N")
                .default_value("false")
                .env("PROJECTION_NO_CONFIRM")
                .takes_value(true),
            arg("project-directory")
                .short("d")
                .default_value("projects")
                .env("PROJECTION_PROJECT_DIR")
                .takes_value(true),
            arg("author").short("a").takes_value(true),
        ])
        .subcommands(vec![
            sub("search").visible_alias("s").arg(arg("query").index(1)),
            sub("select")
                .visible_alias("sel")
                .arg(arg("query").index(1)),
            sub("ensure")
                .visible_aliases(&["en", "e"])
                .help("Check the list of known projects for any missing repos and clone them"),
            sub("path").visible_alias("p").arg(arg("name").index(1)),
            sub("remove")
                .visible_aliases(&["rm", "r"])
                .arg(arg("name").index(1)),
            sub("add").visible_alias("a").args(&[
                Arg::from_usage("--name -n NAME 'The project name (if not provided the name of the repo will be used instead)'"),
                arg("source")
                    .help("gh:user/repo gl:user/repo bb:user/repo")
                    .takes_value(true)
                    .index(1),
            ]),
        ])
}
