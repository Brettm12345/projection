use clap::{
    App,
    AppSettings::{ColoredHelp, DeriveDisplayOrder, InferSubcommands},
    Arg, SubCommand,
};

pub fn build_cli() -> App<'static, 'static> {
    let arg = |name| Arg::with_name(name).long(name);
    let sub = |name| SubCommand::with_name(name);
    App::new("Projection")
        .author("brettm12345")
        .version("0.2.0")
        .long_about("The next generation project manager for the shell")
        .settings(&[ColoredHelp, DeriveDisplayOrder, InferSubcommands])
        .args(&[
            arg("project-directory")
                .short("d")
                .default_value("projects")
                .env("PROJECTION_PROJECT_DIR"),
            arg("author").short("a").takes_value(true),
        ])
        .subcommands(vec![
            sub("search").alias("s").arg(arg("query").index(1)),
            sub("select").alias("sel"),
            sub("check").alias("c"),
            sub("path").alias("p").arg(arg("name").index(1)),
            sub("remove").alias("rm").arg(arg("name").index(1)),
            sub("add").alias("a").arg(
                arg("source")
                    .help("gh:user/repo gl:user/repo bb:user/repo")
                    .index(1),
            ),
        ])
}
