mod cli;
use ansi_term::Color::Cyan;
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
                Ok(_) => println!("Successfully added {} to projects", project),
                _ => println!("Error adding {} to projects", project),
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
                            Ok(()) => println!("Removed {}", id),
                            err => println!("Failed to remove {}\nError: {:?}", project, err),
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
                println!("{}\t{}", Cyan.paint(id), project);
            }
        }
        _ => println!("Unknown command"),
    };
}
