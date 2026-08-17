def dashboard_cleared(reading_ugm3, band_min_ugm3, band_max_ugm3, age_min, clock_fault):
    if clock_fault:
        return False
    in_band = band_min_ugm3 <= reading_ugm3 and reading_ugm3 <= band_max_ugm3
    fresh = age_min < 15
    return in_band and fresh
