#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StoreReadsBefore {
    pub filename: String,
    pub totalreads: String,
    pub totalbases: String,
    pub q20bases: String,
    pub q30bases: String,
    pub q20rate: String,
    pub q30rate: String,
    pub read1meanlength: String,
    pub read2menalength: String,
    pub gccontent: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StoreReadsAfter {
    pub filename: String,
    pub totalreads: String,
    pub totalbases: String,
    pub q20bases: String,
    pub q30bases: String,
    pub q20rate: String,
    pub q30rate: String,
    pub read1meanlength: String,
    pub read2menalength: String,
    pub gccontent: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FilteringResults {
    pub filename: String,
    pub passed_filter_reads: String,
    pub low_quality_reads: String,
    pub too_many_n_reads: String,
    pub too_short_reads: String,
    pub too_long_reads: String,
}
