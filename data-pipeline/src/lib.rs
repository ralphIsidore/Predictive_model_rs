use ndarray::Array1; 
use rayon::prelude::*; 
use serde::{Deserialize, Serialize};
use thiserror::Error; 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord{
    pub invoice: String, 
    pub stock_code: String, 
    pub description: String, 
    pub quantity: i32, 
    pub unit_price: f64, 
    pub customer_id: Option<u64>,
    pub country: String
}

#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("Empty dataset: cannot normalize an empty slice")]
    EmptyDataset, 
    #[error("Zero range: all values are identical, nortmalization undefined")
    ]
    ZeroRange, 
    #[error("IO error:{0}")]
    
        Io(#[from]std::io::Error),
}

pub fn normalize_prices(prices:&[f64])->Result<Array1<f64>, PipelineError>{
    if prices.is_empty(){
        return Err(PipelineError::EmptyDataset);
    }

    let min = prices.par_iter().cloned().reduce(||f64::INFINITY, f64::min); 

    let max = prices.par_iter().cloned().reduce(||f64::NEG_INFINITY, f64::max); 

    let range = max - min; 

    if range == 0.0 {
        return Err(PipelineError::ZeroRange);
    }

    let normalized:Vec<f64> = prices.par_iter()
    .map(|&p| (p-min)/range).collect(); 

    Ok(Array1::from_vec(normalized))


  
    

} 


#[cfg(test)]
    mod tests{
        use super::*; 
        #[test]

        fn test_normalization_prices_range(){
            let prices = vec![1.0, 5.0, 3.0, 10.0, 2.0]; 
            let result = normalize_prices(&prices).unwrap(); 

            assert!((result[0]-0.0).abs() < 1e-9); 
            assert!((result[3]-1.0).abs() < 1e-9); 
        }


        #[test]
        fn test_empty_prices_returns_error(){
            assert!(matches!(
                normalize_prices(&[]), 
                Err(PipelineError::EmptyDataset)
            ))
        }

        #[test]
        fn test_zero_range_returns_error(){
            assert!(matches!(
                normalize_prices(&[5.0, 5.0, 5.0]), 
                Err(PipelineError::ZeroRange)
            ))
        }

    }

    