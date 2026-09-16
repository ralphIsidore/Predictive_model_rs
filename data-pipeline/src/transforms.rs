use polars::prelude::*; 
use crate::PipelineError; 


pub fn drop_missing_customers(lf:LazyFrame) -> LazyFrame {
    lf.drop_nulls(Some(vec![col("Customer ID")]))
}

pub fn filter_valid_transactions(lf:LazyFrame) -> LazyFrame {
    lf
    .filter(col("Invoice"). str()
    .starts_with(lit("C")).not()).filter(col("Quantity").gt(0))
    .filter(col("Price").gt(lit(0.0)))
}

pub fn trim_description(lf:LazyFrame) -> LazyFrame {
    lf.with_column(col("Description").str().strip_chars(lit(" ")).alias("Description"))
}
pub fn add_reenue(lf:LazyFrame) -> LazyFrame {
    lf.with_column(col("Quantity").cast(DataType::Float64) * col("Price").alias("revenue"))
}

pub fn extract_data_features(lf:LazyFrame) -> LazyFrame {
    let parsed = lf.with_column(col("InvoiceDate").str().to_datetime(Some(TimeUnit::Microseconds), None, StrptimeOptions { 
        format: Some("%d/%m/%m/%y/%H:%M".into()),
     strict: false, 
     exact: true, 
    
     cache: true }, lit("raise").alias("invoice_datetime"))
    );


    parsed.with_column(col("invoice_datetime").dt().year().alias("year")

).with_column(col("invoice_datetime").dt().month().alias("month"))
.with_column(col("invoice_datetime").dt().weekday().alias("day_of_week"))

}

pub fn build_customer_feature(lf:LazyFrame) -> Result <DataFrame, PipelineError>{
    let customer_df = lf.group_by([col("Customer ID")]).agg([
        col("revenue").sum().alias("total_revenue"), 
        col("Invoice").n_unique().alias("num_invoices"), 
        col("Quantity").sum().alias("total_quantity"), 
        (col("revenue").sum()/col("Invoice").n_unique()).alias("avg_revenue_per_invoice"), 
        col("Country").first().alias("country")

    ])
    .sort(["total_revenue"], 
SortMultipleOptions::default().with_order_descending(true)
).collect()?;

Ok(customer_df)
}