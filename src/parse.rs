use regex::Regex;
use std::error::Error;

pub fn run(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let regex_indel = Regex::new(r"[\+\-]{1}(\d+)").unwrap();
    let regex_mismatch = Regex::new(r"[ACGTNacgtn]{1}").unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(input_file)?;
        
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_path(output_file)?;
        
    for result in reader.records() {
        let records = result?;
        let chromosome = &records[0];
        let position = &records[1];
        let reference_base = &records[2];
        let pileup_reads = &records[4];
        
        // count insertion occurrence
        let count_insertion_occurrence: i32 = pileup_reads.matches("+").count() as i32;
            
        // count deletion occurrence
        let count_deletion_occurrence: i32 = pileup_reads.matches("-").count() as i32;
            
        // count insertion + deletion base
        let mut count_indel_base: i32 = 0;
        for capture in regex_indel.captures_iter(pileup_reads) {
            let _str_count_indel_base = capture.get(1).unwrap().as_str();
            let _count_indel_base: i32 = _str_count_indel_base.parse().unwrap();
            count_indel_base += _count_indel_base;
        }
            
        // count substitution base
        let count_substitution_base: i32 = (regex_mismatch.find_iter(pileup_reads).count() as i32)
                                           - count_indel_base;
            
        // count match base
        let count_match_base: i32 = (pileup_reads.matches(".").count() as i32)
                                    + (pileup_reads.matches(",").count() as i32)
                                    - count_insertion_occurrence
                                    - count_deletion_occurrence;

        // count mismatch occurrence
        let count_mismatch_occurrence: i32 = count_substitution_base
                                             + count_insertion_occurrence
                                             + count_deletion_occurrence;
            
        // calcurate depth
        let depth = count_match_base + count_mismatch_occurrence;
            
        // output
        writer.write_record(&[chromosome,
                               position,
                               reference_base,
                               &count_mismatch_occurrence.to_string(),
                               &depth.to_string()])?;
    }
    Ok(())
}