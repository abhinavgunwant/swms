use std::{ fs, env::current_dir, process::Command, path::Path };

use static_files::resource_dir;

fn copy_all(src: impl AsRef<Path>, dest: impl AsRef<Path>) {
    fs::create_dir_all(&dest).unwrap();

    for entry in fs::read_dir(src).unwrap() {
        let e = entry.unwrap();

        let file_type = e.file_type().unwrap();

        if file_type.is_dir() {
            copy_all(e.path(), dest.as_ref().join(e.file_name()));
        } else {
            fs::copy(e.path(), dest.as_ref().join(e.file_name())).unwrap();
        }
    }
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=static/");

    let mut source = current_dir().expect("Could not get current directory.");
    let mut dest = source.clone();

    println!("Building current Directory: {}", source.display());

    source.pop();
    source.push("dam-fe");

    // TODO: Check here if "dist" directory exists
    //      - If the directory does not exists, check if the frontend can be
    //      built.
    source.push("dist");

    dest.push("static");

    match fs::create_dir("static") {
        Ok(_) => {}
        Err(_) => {}
    }

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/c", "cd", "..\\dam-fe"])
            .output()
            .expect("error in cd 1");

        Command::new("cmd")
            .args(&["/c", "yarn", "build"])
            .output()
            .expect("You need to install 'yarn' before continuing");
    } else {
        Command::new("cd")
            .arg("..")
            .output()
            .expect("error in cd 1");

        Command::new("yarn")
            .arg("build")
            .output()
            .expect("You need to install 'yarn' before continuing");
    }


    copy_all(source.as_path(), dest.as_path());

    resource_dir("./static").build()
}

