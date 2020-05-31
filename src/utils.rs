pub fn scale_range(a_min: f64, a_max: f64, b_min: f64, b_max: f64, x: f64) -> f64
{
    return (((x - a_min) * (b_max - b_min)) / (a_max - a_min)) + b_min;
}