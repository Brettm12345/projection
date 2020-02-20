mod cli;
use colored::*;
mod data;
use data::{CloneRepo, Ensure, Project, ToPath};
use dialoguer::Confirmation;
use enquirer::ColoredTheme;
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
    let mut res = Projects::new();
    for (id, project) in projects
        .iter()
        .filter(|(_, project)| project.user.eq(author))
    {
        res.insert(id.to_owned(), project.to_owned());
    }
    res
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
            let project = Project::from_str(m.value_of("source").unwrap()).unwrap();
            project.clone_repo(project_dir).unwrap();
            match db.save(&project) {
                Ok(_) => println!("{} added {} to projects", "Successfully".green(), project),
                _ => println!("{} adding {} to projects", "Error".red(), project),
            }
        }
        ("remove", Some(m)) => {
            let query = m.value_of("name").unwrap();
            match find(query) {
                Some((id, project)) => {
                    if confirm(&format!(
                        "Are you sure you want to remove {} from your projects?",
                        project
                    )) {
                        if confirm("Also remove the project directory") {
                            match fs::remove_dir_all(project_dir.join(project.to_path())) {
                                Ok(()) => println!("Deleted {}", project),
                                _ => println!("Failed to remove dir project files"),
                            }
                        }
                        match db.delete(id) {
                            Ok(()) => println!("Removed from project list {}", id.cyan()),
                            err => println!(
                                "Failed to remove {}\n{}: {:?}",
                                project,
                                "Error".red(),
                                err
                            ),
                        }
                    }
                }
                None => println!("Failed to find {} in projects", query),
            }
        }
        ("check", _) => {
            for (_, project) in projects.iter() {
                match project.ensure(project_dir) {
                    Ok(_) => continue,
                    _ => {
                        if confirm(&format!(
                            "Project {} is missing. Would you like to clone it",
                            project
                        )) {
                            project.clone_repo(project_dir).unwrap();
                        }
                    }
                }
            }
        }
        ("path", Some(m)) => match find(m.value_of("name").unwrap()) {
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
        ("select", _) => {
            let selected_items = Skim::run_with(
                &SkimOptionsBuilder::default()
                    .height(Some("50%"))
                    .multi(true)
                    .ansi(true)
                    .build()
                    .unwrap(),
                Some(Box::new(Cursor::new(project_string))),
            )
            .map(|out| out.selected_items)
            .unwrap_or_else(Vec::new);

            for item in selected_items.iter() {
                println!("{:#?}", item);
            }
        }
        ("search", Some(m)) => fuzzy_search(m.value_of("query").unwrap_or_else(|| ""))
            .iter()
            .for_each(|project| println!("{}", project)),
        _ => println!("{}", project_string),
    };
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use insta::assert_debug_snapshot;
    use tempfile::tempdir;

    #[test]
    fn add_project() {
        let project_dir = tempdir().unwrap();
        assert_debug_snapshot!(Command::cargo_bin("projection")
            .unwrap()
            .arg("-d")
            .arg(&format!("{}", project_dir.path().display()))
            .arg("add")
            .arg("gh:brettm12345/xmonad-config")
            .assert());
        assert!(dir_diff::is_different(project_dir.path(), tempdir().unwrap().path()).unwrap())
    }
}
