pub fn to_string_vec(vec: Vec<&str>) -> Vec<String> { vec.into_iter().map(String::from).collect() }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_string_vec() {
		assert_eq!(to_string_vec(vec!["foo"]), vec![String::from("foo")]);
		assert_eq!(
			to_string_vec(vec!["foo", "bar"]),
			vec![String::from("foo"), String::from("bar")]
		);
	}
}
