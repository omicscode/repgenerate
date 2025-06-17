use std::env;
use std::error::Error;
use std::path::Path;
use std::process::Command;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-6-13
*/

pub fn demultiplexreads(pathdir: &str, sample: &str) -> Result<String, Box<dyn Error>> {
    let _ = Command::new("cd")
        .arg(pathdir)
        .output()
        .expect("command failed");
    let _ = Command::new("conda")
        .arg("create")
        .arg("-n")
        .arg("bclfastq")
        .arg("-y")
        .output()
        .expect("command failed");
    let _ = Command::new("conda")
        .arg("install")
        .arg("-n")
        .arg("bclfastq")
        .arg("dranew::bcl2fastq")
        .output()
        .expect("command failed");
    let _ = Command::new("conda").arg("activate").arg("bclfastq");
    let newpath = Path::new(pathdir);
    assert!(env::set_current_dir(&newpath).is_ok());
    println!(
        "Successfully changed working directory to {}!",
        newpath.display()
    );
    let _ = Command::new("bcl2fastq")
        .arg("-r")
        .arg("4")
        .arg("-w")
        .arg("4")
        .arg("--no-lane-splitting")
        .arg("--sample-sheet")
        .arg(sample)
        .arg("-o")
        .arg("fastqreads")
        .output()
        .expect("command failed");
    Ok("bclconvert has finished".to_string())
}
