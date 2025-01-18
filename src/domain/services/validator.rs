
pub fn validate_page(page: &u64) -> Result<(), String> {
    if *page == 0 {
        return Err("Page number should be greater than 0".to_string());
    }
    Ok(())
}

pub fn validate_per_page(per_page: &u64) -> Result<(), String> {
    if *per_page == 0 {
        return Err("Number of elements per page should be greater than 0".to_string());
    } else if *per_page > 100 {
        return Err("Number of elements per page should be less than 100".to_string());
    }
    Ok(())
}
