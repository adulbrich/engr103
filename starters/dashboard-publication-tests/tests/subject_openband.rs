fn dashboard_cleared(reading_ugm3: i32, band_min_ugm3: i32, band_max_ugm3: i32, age_min: i32, clock_fault: bool) -> bool {
    if clock_fault {
        return false;
    }
    let in_band = band_min_ugm3 < reading_ugm3 && reading_ugm3 < band_max_ugm3;
    let fresh = age_min < 15;
    return in_band && fresh;
}
