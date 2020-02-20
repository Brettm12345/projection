mod cli;
use colored::*;
mod data;
use data::{CloneRepo, Project, ToPath};
use dialoguer::Confirmation;
use enquirer::ColoredTheme;
use jfs::Store;
use std::iter::Iterator;
use std::str::FromStr;

fn main() {
    let app = cli::build_cli();
    let matches = app.get_matches();
    let mut cfg = jfs::Config::default();
    cfg.pretty = true;
    let db = Store::new_with_cfg("data", cfg).unwrap();
    let projects = db.all::<Project>().unwrap();
    let project_dir = matches.value_of("project-directory").unwrap();

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
        ("list", _) => {
            for (id, project) in projects.iter() {
                println!("{}\t{}", id.cyan(), project);
            }
        }
        _ => println!("Unknown command"),
    };
}

mod tests {
    #[test]
    fn add_project() {
        let dir = TempDir::new().unwrap();
        let mut cmd = assert_cmd::Command::cargo_bin("projection").unwrap();
        insta::assert_debug_snapshot!(cmd
            .arg("-d")
            .arg(dir.path().to_owned())
            .arg("add")
            .arg("gh:brettm12345/projection")
            .assert())
    }
}
