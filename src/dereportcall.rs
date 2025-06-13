use crate::demultiplexstruct::{FilteringResults, StoreReadsAfter, StoreReadsBefore};
use serde::*;
use serde_json::{Deserializer, Serializer};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-6-13
*/

pub async fn dereportcallfunction(pathdir: &str) -> Result<String, Box<dyn Error>> {
    let dirpath = Path::new(pathdir);

    for i in fs::read_dir(dirpath)? {
        let openfile = i?.path();
        let path_str = openfile.to_str().unwrap();
        let filen: String = path_str.split(".").collect::<Vec<_>>()[0].to_string();
        let filecontent = std::fs::read_to_string(path_str).unwrap();
        let fileview: serde_json::Value =
            serde_json::from_str(&filecontent).expect("file not present");
        let mut filterresults: Vec<FilteringResults> = Vec::new();
        let mut afterstorereadsvector: Vec<StoreReadsAfter> = Vec::new();
        let mut beforestorereadsvector: Vec<StoreReadsBefore> = Vec::new();
        let mut duplication: Vec<String> = Vec::new();
        let mut insertsize: Vec<(String, String)> = Vec::new();
        let mut adaptercut: Vec<(String, String)> = Vec::new();
        if fileview.is_object() {
            for (fileviewkey, filekeyvalue) in fileview.as_object().unwrap() {
                if fileviewkey == "filtering_result" {
                    filterresults.push(FilteringResults {
                        filename: filen,
                        passed_filter_reads: fileviewkey["passed_filter_reads"].to_string(),
                        low_quality_reads: fileviewkey["low_quality_reads"].to_string(),
                        too_many_n_reads: fileviewkey["too_many_N_reads"].to_string(),
                        too_short_reads: fileviewkey["too_short_reads"].to_string(),
                        too_long_reads: fileviewkey["too_long_reads"].to_string(),
                    });
                }
                if fileviewkey == "duplication" {
                    duplication.push(fileviewkey["rate"]);
                }
                if fileviewkey == "insertsize" {
                    let value: (String, String) = (fileviewkey["peak"], fileviewkey["unknown"]);
                    insertsize.push(value);
                }
                if fileviewkey == "adapter_cutting" {
                    let adapter: (String, String) = (
                        fileviewkey["adapter_trimmed_reads"],
                        fileviewkey["adapter_trimmed_bases"],
                    );
                    adaptercut.push(adapter);
                }
                if fileview == "summary" {
                    let newobject = fileview["summary"];
                    if fileview.is_object() {
                        for (keystring, keyvalue) in newobject.as_object().unwrap() {
                            if keystring == "before_filtering" {
                                beforestorereadsvector.push(StoreReadsBefore {
                                    filename: keystring["total_reads"],
                                    totalreads: keystring["total_reads"],
                                    totalbases: keystring["total_bases"],
                                    q20bases: keystring["q20_bases"],
                                    q30bases: keystring["q30_bases"],
                                    q20rate: keystring["q20_rate"],
                                    q30rate: keystring["q30_rate"],
                                    read1meanlength: keystring["read1_mean_length"],
                                    read2menalength: keystring["read2_mean_length"],
                                    gccontent: keystring["gc_content"],
                                })
                            }
                            if keystring == "after_filtering" {
                                afterstorereadsvector.push(StoreReadsBefore {
                                    filename: keystring["total_reads"],
                                    totalreads: keystring["total_reads"],
                                    totalbases: keystring["total_bases"],
                                    q20bases: keystring["q20_bases"],
                                    q30bases: keystring["q30_bases"],
                                    q20rate: keystring["q20_rate"],
                                    q30rate: keystring["q30_rate"],
                                    read1meanlength: keystring["read1_mean_length"],
                                    read2menalength: keystring["read2_mean_length"],
                                    gccontent: keystring["gc_content"],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok("bclconvert has finished".to_string())
}
