mod args;
mod demultiplex;
mod dereport;
mod dereportcall;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::demultiplex::demultiplexreads;
use crate::dereport::dereportcallreads;
use crate::dereportcall::dereportcallfunction;
use async_std::task;
use clap::Parser;

/*
 Authom GauravSablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-4-8
*/

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::DEMULTIPLEX {
            illumina,
            samplesheet,
        } => {
            let command = demultiplexreads(illumina, samplesheet).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::DEREPORT {
            illumina,
            samplesheet,
        } => {
            let command = dereportcallreads(illumina, samplesheet).unwrap();
            println!("The command has been finished:{}", command);
        }
        Commands::REPORT { report } => {
            let command = dereportcallfunction(report).unwrap();
            println!("The report have been generated:{}", command);
        }
    }
}
