# repgenerate
- repgenerate for illumina sequencing reports, hich will demultiplex and does all the report generation for any Illumina sequencing platform. 
- you and demultiplex, or demultiplex and report generation and lastly you can generate report from already demultiplexed reads.

```
cargo build
```

```
  ____    _____   ____     ____   _____   _   _   _____   ____       _      _____   _____
 |  _ \  | ____| |  _ \   / ___| | ____| | \ | | | ____| |  _ \     / \    |_   _| | ____|
 | |_) | |  _|   | |_) | | |  _  |  _|   |  \| | |  _|   | |_) |   / _ \     | |   |  _|
 |  _ <  | |___  |  __/  | |_| | | |___  | |\  | | |___  |  _ <   / ___ \    | |   | |___
 |_| \_\ |_____| |_|      \____| |_____| |_| \_| |_____| |_| \_\ /_/   \_\   |_|   |_____|


demultiplexing and repgenerate for human genomics.
       ************************************************
      Gaurav Sablok, IBCH, PAN, Poznan, Poland,
      https://portal.ichb.pl/laboratory-of-genomics/.
      Email: gsablok@ibch.poznan.pl
      ************************************************

Usage: repgenerate <COMMAND>

Commands:
  demultiplex  demultiplex the illumina sequencing
  dereport     demultiplex and report generate
  report       report claibration
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

- To install windows version:
```
rustup component add llvm-tools
rustup target add x86_64-pc-windows-msvc
git clone https://github.com/IBCHgenomic/eVaiutilities.git
cd ensemblcov
cargo xwin build --target x86_64-pc-windows-msvc
```
- to demultiplex the reads and generate the report
```
./target/debug/repgenerate dereport BHMT5TDRX3 SampleSheet20250509_ICHB_RNAseq001.csv
```

- to just demultiplex the reads
```
./target/debug/repgenerate multiplex BHMT5TDRX3 SampleSheet20250509_ICHB_RNAseq001.csv
```

- to run the simple report generator where you have a folder of any number of the fastp reports and for each sample, it will prepare a single file with what is needed. With demultiplex options, it will prepare the same report. 

```
./target/debug/repgenerate report sample-file
```

```
duplication-rate:"0.083112"
insert size peak        insert size unknown     adapter trimmed reads   adapter trimmed bases   170     5455420 24376812        893347838
before filtering
before filtering        total_reads     total_bases     q20_bases       q30_bases       q20_rate        q30_rate        read1_mean_length       read2_mean_length       gc_content
105961318       16000159018     15672173600     15065791642     0.979501        0.941603        151     151     0.449647
after filtering
total_reads     total_bases     q20_bases       q30_bases       q20_rate        q30_rate        read1_mean_length       read2_mean_length       gc_content
104626372       14908378954     14761963198     14354265840     0.990179        0.962832        142     142     0.440785
```


Gaurav Sablok \
Instytut Chemii Bioorganicznej \
Polskiej Akademii Nauk \
ul. Noskowskiego 12/14 | 61-704, Poznań \
Poland
