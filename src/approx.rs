pub fn approx(a: f64, b: f64) -> bool {
    if a == b {
		return true;
    }
    let eps = 1e-2;
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (abs_a - abs_b).abs();
	if a == 0.0 || b == 0.0 || abs_a + abs_b < std::f64::EPSILON {
		diff < eps * std::f64::EPSILON
	} else {
        diff / (abs_a + abs_b).min(std::f64::MAX) < eps
    }
}