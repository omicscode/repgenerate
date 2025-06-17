use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "repgenerate",
    version = "1.0",
    about = "demultiplexing and repgenerate for human genomics.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// demultiplex the illumina sequencing
    MULTIPLEX {
        /// path to the illumina directory
        illuminadir: String,
        /// samplesheet path
        samplesheet: String,
    },
    /// demultiplex and report generate
    DEREPORT {
        /// path to the illumina directory
        illuminadir: String,
        /// samplesheet path
        samplesheet: String,
    },
    /// report claibration
    REPORT {
        /// path to the report folder
        report: String,
    },
}
