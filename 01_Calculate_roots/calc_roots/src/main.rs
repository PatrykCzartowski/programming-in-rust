fn calculate_delta(a: f32, b: f32, c: f32) -> f32 {
	b * b - 4.0 * a * c
}

fn calculate_roots(b: f32, delta: f32, a: f32) -> (f32, f32) {
	let x1 = (-b + delta.sqrt()) / (2.0 * a);
	let x2 = (-b - delta.sqrt()) / (2.0	 * a);
	(x1, x2)
}

fn main() {
	let a = -4.0;
	let b = 7.0;
	let c = 3.0;

//	let delta: f32 = b * b - 4.0 * a * c;
//	let x1 = (-b + delta.sqrt()) / (2.0 * a);
//	let x2 = (-b - delta.sqrt()) / (2.0 * a);

	let delta = calculate_delta(a, b, c);
	println!("delta = {}", delta);
	if delta >= 0.0 {
		let roots = calculate_roots(b, delta, a);
		println!("roots (x1, x2) = {:?}", roots);
	}
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn test_cal_delta() {
		assert_eq!(calculate_delta(1.0, 2.0, -3.0), 16.0);
		assert_eq!(calculate_delta(5.0, 5.0, -3.0), 85.0);
		assert_eq!(calculate_delta(-4.0, 7.0, 3.0), 97.0);
	}

	#[test]
	fn test_cal_roots() {
		assert_eq!(calculate_roots(2.0, 16.0, 1.0), (1.0, -3.0));
		assert_eq!(calculate_roots(5.0, 85.0, 5.0), (0.42195445, -1.4219544));
		assert_eq!(calculate_roots(7.0, 97.0, -4.0), (-0.35610723, 2.1061072));
	}
}
