mod cli;
use colored::*;
mod data;
use data::{CloneRepo, Project, ToPath};
use dialoguer::Confirmation;
use enquirer::ColoredTheme;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use jfs::Store;
use std::collections::BTreeMap;
use std::iter::{FromIterator, Iterator};
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

    let project_dir = matches.value_of("project-directory").unwrap();
    let fuzzy_search = |q: &str| -> Vec<&Project> {
        let matcher = SkimMatcherV2::default();
        let fuzz = |p: &Project| -> Option<i64> { matcher.fuzzy_match(&format!("{}", p), q) };
        let mut v = Vec::from_iter(projects.values().filter(|p| fuzz(p).is_some()));
        v.sort_by(|&a, &b| fuzz(b).cmp(&fuzz(a)));
        v
    };

    let search = |string: &str| -> Option<(&String, &Project)> {
        projects
            .iter()
            .find(|(_, project)| project.repo.as_str().contains(string))
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
            match search(query) {
                Some((id, project)) => {
                    if Confirmation::with_theme(&ColoredTheme::default())
                        .with_text(&format!(
                            "Are you sure you want to remove {} from your projects?",
                            project
                        ))
                        .interact()
                        .unwrap()
                    {
                        match db.delete(id) {
                            Ok(()) => println!("Removed {}", id.cyan()),
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
        ("path", Some(m)) => match search(m.value_of("name").unwrap()) {
            Some((_, project)) => {
                println!("{}/{}", project_dir, project.to_path().to_str().unwrap())
            }

            _ => println!("Unable to find item"),
        },
        ("search", Some(m)) => fuzzy_search(m.value_of("query").unwrap())
            .iter()
            .for_each(|project| println!("{}", project)),
        _ => projects
            .iter()
            .for_each(|(id, project)| println!("{}\t{}", id.cyan(), project)),
    };
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use dir_diff;
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
