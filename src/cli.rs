use clap::{
    App,
    AppSettings::{ColoredHelp, DeriveDisplayOrder, InferSubcommands},
    Arg, SubCommand,
};

fn arg<'a>(name: &'a str) -> Arg {
    Arg::with_name(name).long(name)
}
pub fn build_cli() -> App<'static, 'static> {
    let sub = |name| SubCommand::with_name(name);
    App::new("Projection")
        .author("brettm12345")
        .version("0.0.1")
        .long_about("The next generation project manager for the shell")
        .settings(&[ColoredHelp, DeriveDisplayOrder, InferSubcommands])
        .args(&[
            arg("project-directory")
                .short("d")
                .default_value("projects")
                .env("PROJECTION_PROJECT_DIR"),
            arg("author").short("a").takes_value(true),
        ])
        .subcommand(sub("search").alias("s").arg(arg("query").index(1)))
        .subcommand(sub("path").alias("p").arg(arg("name").index(1)))
        .subcommand(sub("remove").alias("rm").arg(arg("name").index(1)))
        .subcommand(
            sub("add")
                .alias("a")
                .help("gh:USER/REPO gl:USER_REPO bb:USER/REPO")
                .arg(arg("source").index(1)),
        )
}
