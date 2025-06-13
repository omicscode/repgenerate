use serde::*;
use serde_json::{Deserializer, Serializer};
use std::error::Error;
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

    Ok("bclconvert has finished".to_string())
}
