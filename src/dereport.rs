use crate::demultiplexstruct::{FilteringResults, StoreReadsAfter, StoreReadsBefore};
use crate::dereport::fs::File;
use std::error::Error;
use std::fs;
use std::io::Write;
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
    for i in fs::read_dir(jsonpath)? {
        let openfile = i?.path();
        let path_str = openfile.to_str().unwrap();
        let filen: String = path_str.split(".").collect::<Vec<_>>()[0].to_string();
        let filecontent = std::fs::read_to_string(path_str).unwrap();
        let fileview: serde_json::Value = serde_json::from_str(&filecontent)?;
        let mut filterresults: Vec<FilteringResults> = Vec::new();
        let mut afterstorereadsvector: Vec<StoreReadsAfter> = Vec::new();
        let mut beforestorereadsvector: Vec<StoreReadsBefore> = Vec::new();
        let mut duplicationiter: Vec<String> = Vec::new();
        let mut insertsize: Vec<(String, String)> = Vec::new();
        let mut adaptercut: Vec<(String, String)> = Vec::new();
        if fileview.is_object() {
            for (fileviewkey, _filekeyvalue) in fileview.as_object().unwrap() {
                if fileviewkey == "filtering_result" {
                    filterresults.push(FilteringResults {
                        filename: filen.clone(),
                        passed_filter_reads: fileview["filtering_result"]["passed_filter_reads"]
                            .to_string(),
                        low_quality_reads: fileview["filtering_result"]["low_quality_reads"]
                            .to_string(),
                        too_many_n_reads: fileview["filtering_result"]["too_many_N_reads"]
                            .to_string(),
                        too_short_reads: fileview["filtering_result"]["too_short_reads"]
                            .to_string(),
                        too_long_reads: fileview["filtering_result"]["too_long_reads"].to_string(),
                    });
                }
                if fileviewkey == "duplication" {
                    duplicationiter.push(fileview["duplication"]["rate"].to_string());
                }
                if fileviewkey == "insert_size" {
                    let value: (String, String) = (
                        fileview["insert_size"]["peak"].to_string(),
                        fileview["insert_size"]["unknown"].to_string(),
                    );
                    insertsize.push(value);
                }
                if fileviewkey == "adapter_cutting" {
                    let adapter: (String, String) = (
                        fileview["adapter_cutting"]["adapter_trimmed_reads"].to_string(),
                        fileview["adapter_cutting"]["adapter_trimmed_bases"].to_string(),
                    );
                    adaptercut.push(adapter);
                }
                if fileviewkey == "summary" {
                    let newobject = fileview["summary"].clone();
                    if newobject.is_object() {
                        for (keystring, _keyvalue) in newobject.as_object().unwrap() {
                            if keystring == "before_filtering" {
                                beforestorereadsvector.push(StoreReadsBefore {
                                    filename:
                                        fileview["summary"]["before_filtering"]["total_reads"]
                                            .to_string(),
                                    totalreads:
                                        fileview["summary"]["before_filtering"]["total_reads"]
                                            .to_string(),
                                    totalbases:
                                        fileview["summary"]["before_filtering"]["total_bases"]
                                            .to_string(),
                                    q20bases: fileview["summary"]["before_filtering"]["q20_bases"]
                                        .to_string(),
                                    q30bases: fileview["summary"]["before_filtering"]["q30_bases"]
                                        .to_string(),
                                    q20rate: fileview["summary"]["before_filtering"]["q20_rate"]
                                        .to_string(),
                                    q30rate: fileview["summary"]["before_filtering"]["q30_rate"]
                                        .to_string(),
                                    read1meanlength:
                                        fileview["summary"]["before_filtering"]["read1_mean_length"]
                                            .to_string(),
                                    read2meanlength:
                                        fileview["summary"]["before_filtering"]["read2_mean_length"]
                                            .to_string(),
                                    gccontent:
                                        fileview["summary"]["before_filtering"]["gc_content"]
                                            .to_string(),
                                })
                            }
                            if keystring == "after_filtering" {
                                afterstorereadsvector.push(StoreReadsAfter {
                                    filename: fileview["summary"]["after_filtering"]["total_reads"]
                                        .to_string(),
                                    totalreads:
                                        fileview["summary"]["after_filtering"]["total_reads"]
                                            .to_string(),
                                    totalbases:
                                        fileview["summary"]["after_filtering"]["total_bases"]
                                            .to_string(),
                                    q20bases: fileview["summary"]["after_filtering"]["q20_bases"]
                                        .to_string(),
                                    q30bases: fileview["summary"]["after_filtering"]["q30_bases"]
                                        .to_string(),
                                    q20rate: fileview["summary"]["after_filtering"]["q20_rate"]
                                        .to_string(),
                                    q30rate: fileview["summary"]["after_filtering"]["q30_rate"]
                                        .to_string(),
                                    read1meanlength:
                                        fileview["summary"]["after_filtering"]["read1_mean_length"]
                                            .to_string(),
                                    read2meanlength:
                                        fileview["summary"]["after_filtering"]["read2_mean_length"]
                                            .to_string(),
                                    gccontent: fileview["summary"]["after_filtering"]["gc_content"]
                                        .to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
        let mut filewrite = File::create(filen.clone()).expect("filename not found");
        writeln!(
            filewrite,
            "{}:{:?}",
            "duplication-rate",
            duplicationiter[0].to_string()
        )
        .expect("file not present");
        for i in 0..insertsize.len() {
            writeln!(
                filewrite,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                "insert size peak",
                "insert size unknown",
                "adapter trimmed reads",
                "adapter trimmed bases",
                insertsize[i].0,
                insertsize[i].1,
                adaptercut[i].0,
                adaptercut[i].1
            )
            .expect("file not present");
        }
        writeln!(filewrite, "{}", "before filtering").expect("file not present");
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            "before filtering",
            "total_reads",
            "total_bases",
            "q20_bases",
            "q30_bases",
            "q20_rate",
            "q30_rate",
            "read1_mean_length",
            "read2_mean_length",
            "gc_content"
        )
        .expect("file not present");
        for i in beforestorereadsvector.iter() {
            writeln!(
                filewrite,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.totalreads,
                i.totalbases,
                i.q20bases,
                i.q30bases,
                i.q20rate,
                i.q30rate,
                i.read1meanlength,
                i.read2meanlength,
                i.gccontent
            )
            .expect("file not present");
        }
        writeln!(filewrite, "{}", "after filtering").expect("file not present");
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            "total_reads",
            "total_bases",
            "q20_bases",
            "q30_bases",
            "q20_rate",
            "q30_rate",
            "read1_mean_length",
            "read2_mean_length",
            "gc_content"
        )
        .expect("file not present");
        for i in afterstorereadsvector.iter() {
            writeln!(
                filewrite,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.totalreads,
                i.totalbases,
                i.q20bases,
                i.q30bases,
                i.q20rate,
                i.q30rate,
                i.read1meanlength,
                i.read2meanlength,
                i.gccontent
            )
            .expect("file not present");
        }
    }
    Ok("bclconvert has finished".to_string())
}
