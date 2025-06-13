# repgenerate
- repgenerate for illumina sequencing reports
- a single stand along binary which will demultiplex and does all the report generation for any Illumina sequencing platform. 
- you and demultiplex, or demultiplex and report generation and lastly you can generate report from already demultiplexed reads. 

![](https://github.com/IBCHgenomic/eVaiutilities/blob/main/logo.png)

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
      Prof. Luiza Handschuh
      Email: luizahan@ibch.poznan.pl.
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


 - Acknowledgements: MOSAIC platform, developed as part of the ECBiG-MOSAIC project (POIR.04.02.00-00-D017/20), co-financed by the European Regional Development Fund (ERDF) under the Smart Growth Operational Programme 2014-2020, Measure 4.2 for the development of modern research infrastructure in the science sector. 
 - Informal queries: Prof. Luiza Handschuh: luizahan@ibch.poznan.pl.
 - Code related queries: Dr. Gaurav Sablok: gsablok@ibch.poznan.pl.

 Gaurav Sablok Instytut Chemii Bioorganicznej Polskiej Akademii Nauk ul. Noskowskiego 12/14 | 61-704, Poznań Poland
