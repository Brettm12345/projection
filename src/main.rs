mod cli;
use colored::*;
mod data;
use data::{CloneRepo, Ensure, Project, ToPath};
use dialoguer::Confirmation;
use enquirer::ColoredTheme;
use fp_core::chain::Chain;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use jfs::Store;
use skim::{Skim, SkimOptionsBuilder};
use std::collections::BTreeMap;
use std::fs;
use std::io::Cursor;
use std::iter::{FromIterator, Iterator};
use std::path::Path;
use std::str::FromStr;

type Projects = BTreeMap<String, Project>;

fn select_author(projects: Projects, author: &str) -> Projects {
    projects
        .iter()
        .filter(|(_, project)| project.user.eq(author))
        .map(|(id, project)| (id.to_owned(), project.to_owned()))
        .collect()
}

fn main() {
    let app = cli::build_cli();
    let matches = app.get_matches();
    let mut cfg = jfs::Config::default();
    cfg.pretty = true;
    let db = Store::new_with_cfg("data", cfg).unwrap();
    let projects: Projects = db
        .all()
        .map(|p| match matches.value_of("author") {
            Some(author) => select_author(p, author),
            None => p,
        })
        .unwrap();

    let project_dir = Path::new(matches.value_of("project-directory").unwrap());

    let fuzzy_search = |q: &str| -> Vec<&Project> {
        let matcher = SkimMatcherV2::default();
        let fuzz = |p: &Project| -> Option<i64> { matcher.fuzzy_match(&format!("{}", p), q) };
        let mut v = Vec::from_iter(projects.values().filter(|p| fuzz(p).is_some()));
        v.sort_by(|&a, &b| fuzz(b).cmp(&fuzz(a)));
        v
    };
    let project_string = projects
        .iter()
        .fold("".to_string(), |line, (_, p)| format!("{}{}\n", line, p));

    let find = |string: &str| -> Option<(&String, &Project)> {
        projects
            .iter()
            .find(|(_, project)| project.repo.as_str().contains(string))
    };

    let confirm = |string: &str| {
        Confirmation::with_theme(&ColoredTheme::default())
            .with_text(string)
            .interact()
            .unwrap_or(false)
    };

    match matches.subcommand() {
        ("add", Some(m)) => {
            let project = m
                .value_of("source")
                .ok_or_else(|| String::from("No source provided"))
                .chain(Project::from_str)
                .unwrap();
            project.clone_repo(project_dir).unwrap();
            match db.save(&project) {
                Ok(_) => println!("{} added {} to projects", "Successfully".green(), project),
                _ => println!("{} adding {} to projects", "Error".red(), project),
            }
        }
        ("remove", Some(m)) => {
            let query = m.value_of("name");
            match query.chain(find) {
                Some((id, project)) => {
                    if confirm(&format!(
                        "Are you sure you want to remove {} from your projects?",
                        project
                    )) {
                        match db.delete(id) {
                            Ok(_) => println!("{} from project list {}", "Removed".red(), id.cyan()),
                            err => println!(
                                "{} to remove {}\n{}: {:?}",
                                "Failed".red(),
                                project,
                                "Error".red(),
                                err
                            ),
                        }
                        if confirm("Also remove the project directory") {
                            let project_path = project_dir.join(project.to_path());
                            match fs::remove_dir_all(&project_path) {
                                Ok(_) => println!("Deleted {}", &project_path.to_str().unwrap().cyan()),
                                _ => println!("Failed to remove dir project files"),
                            }
                        }
                    }
                }
                None => println!("Failed to find {} in projects", query.unwrap_or("")),
            }
        }
        ("check", _) => {
            projects
                .iter()
                .for_each(|(_, project)| match project.ensure(project_dir) {
                    Ok(_) => (),
                    _ => {
                        if confirm(&format!(
                            "Project {} is missing. Would you like to clone it",
                            project
                        )) {
                            project.clone_repo(project_dir).unwrap();
                        }
                    }
                })
        }
        ("path", Some(m)) => match m.value_of("name").chain(find) {
            Some((_, project)) => println!(
                "{}",
                project_dir
                    .join(project.to_path())
                    .as_path()
                    .to_str()
                    .unwrap()
            ),

            _ => println!("Unable to find item"),
        },
        ("select", Some(m)) => Skim::run_with(
            &SkimOptionsBuilder::default()
                .height(Some("50%"))
                .query(m.value_of("query"))
                .multi(true)
                .ansi(true)
                .build()
                .unwrap(),
            Some(Box::new(Cursor::new(project_string))),
        )
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new)
        .iter()
        .for_each(|item| println!("{:#?}", item)),
        ("search", Some(m)) => fuzzy_search(m.value_of("query").unwrap_or(""))
            .iter()
            .for_each(|project| println!("{}", project)),
        _ => println!("{}", project_string),
    };
}
