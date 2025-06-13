use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-6-13
*/

pub fn dereportcallreads(pathdir: &str, sample: &str) -> Result<String, Box<dyn Error>> {
    let _newpath = Path::new(pathdir);
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
    let _ = Command::new("bcl2fastq")
        .arg("-r")
        .arg("4")
        .arg("-w")
        .arg("4")
        .arg("--no-lane-splitting")
        .arg("--sample-sheet")
        .arg(sample)
        .arg("-o")
        .arg("fastqreads");
    static FASTQPATH: &str = "pathdir/fastqreads";
    let newpath = Path::new(FASTQPATH);
    let mut r1files: Vec<String> = Vec::new();
    let mut r2files: Vec<String> = Vec::new();
    for i in fs::read_dir(newpath)? {
        let openfile = i?.path();
        let path_str = openfile.to_str().unwrap();
        if path_str.contains("R1") {
            r1files.push(path_str.to_string());
        } else if path_str.contains("R2") {
            r2files.push(path_str.to_string())
        }
    }
    for i in 0..r1files.len() {
        let _command = Command::new("fastp")
            .arg("--in1")
            .arg(r1files[i].clone())
            .arg("--out1")
            .arg(format!("{}.{}", r1files[i], "cleaned.fastq"))
            .arg("--in2")
            .arg(r2files[i].clone())
            .arg("--out2")
            .arg(format!("{}.{}", r2files[i], "cleaned.fastq"))
            .arg("--json")
            .arg(r1files[i].clone());
    }

    static JSONPATH: &str = "pathdir/jsondir";
    let jsonpath = Path::new(JSONPATH);

    Ok("bclconvert has finished".to_string())
}
