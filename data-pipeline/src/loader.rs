use polars::prelude::*; 
use crate::PipelineError; 

pub fn load_raw(path_2009:&str, path_2010:&str) -> Result<LazyFrame, PipelineError>{
   
    // let opts = CsvReadOptions::default()
    // .with_has_header(true)
    // .with_infer_schema_length(Some(1000)); 

    let df_2009 = LazyCsvReader::new(path_2009).with_has_header(true)
    .with_infer_schema_length(Some(1000)).finish()?;

    let df_2010 = LazyCsvReader::new(path_2010)
    .with_has_header(true)
    .with_infer_schema_length(Some(1000)).finish()?; 
    
    // LazyFrame::scan_csv(path_2009, opts.clone())?; 
    // let df_2010 = polars::scan_csv(path_2010, opts.clone())?; 

    let combined = concat([df_2009, df_2010], UnionArgs{
        rechunk:true, 
        ..Default::default()
    })?;

    Ok(combined)
}

pub fn inspect(lf:&LazyFrame) -> Result<(), PipelineError>{
    let sample = lf.clone().limit(5).collect()?; 
    println!("Schema:\n{:?}", sample.schema());
    println!("\nFirst 5 rows:\n{}", sample);

    let row_count = lf.clone()
    .select([all().count().alias("n")]).collect()?;

    println!("\nTotal rows: {}", row_count);

    Ok(())
}









#[cfg(test)]
mod tests {
    use super::*;


#[test]
fn debug_count() {
    println!("running test...");
    let data = load_raw(
        "../data/raw/online_retail_2009_2010.csv",
        "../data/raw/online_retail_2010_2011.csv"
    ).unwrap();
    
    let count = data
        .select([lit(1).count().alias("n")])
        .collect()
        .unwrap();
        
    println!("{:?}", count);
    println!("Column type: {:?}", count.column("n").unwrap().dtype());
    
    // Extract the value
    let n = count.column("n").unwrap().u32().unwrap().get(0).unwrap_or(0);
    println!("Total rows: {}", n);
    assert!(n > 0);
}

#[test]
fn lf_inspection(){
        let data = load_raw(
        "../data/raw/online_retail_2009_2010.csv",
        "../data/raw/online_retail_2010_2011.csv"
    ).unwrap();

    inspect(&&data);

}


}


