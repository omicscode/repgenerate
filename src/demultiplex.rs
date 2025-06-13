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
    let _ = Command::new("conda")
        .arg("create")
        .arg("-n")
        .arg("bclfastq")
        .arg("-y")
        .output()
        .expect("command failed");

    let _ = Command::new("wget")
        .arg("conda")
        .arg("install")
        .arg("-n")
        .arg("bclfastq")
        .arg("dranew::bcl2fastq")
        .output()
        .expect("command failed");
    let _newpath = Path::new(pathdir);
    let _ = Command::new("bcl2fastq")
        .arg("-r 4")
        .arg("-w 4")
        .arg("--no-lane-splitting")
        .arg("--sample-sheet")
        .arg(sample)
        .arg("-o")
        .arg("fastqreads");

    Ok("bclconvert has finished".to_string())
}
