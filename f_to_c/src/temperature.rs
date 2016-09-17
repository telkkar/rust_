/// Converts from Fahrenheit to Celsius
///
/// # Examples
///
/// assert!(f_to_c(-40f32) == -40f32);
/// assert!(f_to_c(32f32) == 0f32);
/// assert!(f_to_c(212f32) == 100f32);
pub fn f_to_c(fahren: f32) -> f32 {
	(fahren - 32f32) * 5.0/9.0
}

#[test]
fn test_neg40f_is_neg40c() {
	// Arrange
	let fahren: f32 = -40f32;
	let expected_celsius: f32 = -40f32;
	let actual_celsius;

	// Act
	actual_celsius = f_to_c(fahren);

	// Assert
	assert!(expected_celsius == actual_celsius)
}

#[test]
fn test_32f_is_0c() {
	// Arrange
	let fahren: f32 = 32f32;
	let expected_celsius: f32 = 0f32;
	let actual_celsius;

	// Act
	actual_celsius = f_to_c(fahren);

	// Assert
	assert!(expected_celsius == actual_celsius)
}


#[test]
fn test_212f_is_100c() {
	// Arrange
	let fahren: f32 = 212f32;
	let expected_celsius: f32 = 100f32;
	let actual_celsius;

	// Act
	actual_celsius = f_to_c(fahren);

	// Assert
	assert!(expected_celsius == actual_celsius)
}
