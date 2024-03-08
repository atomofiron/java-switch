mod option;
mod operation;

use std::env::args;
use std::process::{Command, exit};
use tray_item::{TrayItem, IconSource};
use crate::operation::Operation;
use crate::option::TrayOption;

/*
% where java
/usr/local/opt/openjdk@17/bin/java
/usr/bin/java
*/

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() {
    let mut args = args().map(|it| it).collect::<Vec<String>>();
    match args.len() {
        0 => panic!("Unable to recognise itself"),
        1 => determine_versions(args),
        _ => {
            args.remove(0);
            show_options(args)
        },
    }
}

fn determine_versions(args: Vec<String>) {
    let mut applied_path = String::new();
    loop {
        let options = options();
        let count = options.len() as i32;
        let labels = options.iter()
            .map(|it| it.label(it.has_path(&applied_path)))
            .collect();
        match run_new(args.first().unwrap(), &labels) {
            0 => break,
            code if !(1..count).contains(&code) => {
                println!("Unexpected code ({code})");
                exit(code);
            },
            code => {
                let option = options.get(code as usize - 1).unwrap();
                match &option.operation {
                    Operation::Apply(path) => {
                        applied_path = path.clone();
                        apply(option.clone())
                    }
                    Operation::Refresh => (),
                }
            },
        }
    }
}

fn options() -> Vec<TrayOption> {
    vec![
        TrayOption::new("Default".to_string(), Operation::Apply("".to_string())),
        TrayOption::new("openjdk@11".to_string(), Operation::Apply("/usr/local/opt/openjdk@11/bin/java".to_string())),
        TrayOption::new("openjdk@17".to_string(), Operation::Apply("/usr/local/opt/openjdk@17/bin/java".to_string())),
        TrayOption::new("openjdk@21".to_string(), Operation::Apply("/usr/local/opt/openjdk@21/bin/java".to_string())),
        TrayOption::new("Refresh".to_string(), Operation::Refresh),
    ]
}

fn run_new(path: &str, options: &Vec<String>) -> i32 {
    Command::new(path)
        .args(options)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .code()
        .unwrap()
}

fn apply(option: TrayOption) {
    println!("todo apply {}", option.label(false));
}

fn show_options(options: Vec<String>) {
    let mut tray = TrayItem::new("J", IconSource::Resource("")).unwrap();

    tray.add_label("Java versions").unwrap();
    for (index, option) in options.iter().enumerate() {
        tray.add_menu_item(option.as_str(), move || exit(index as i32 + 1)).unwrap();
    }

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
}

