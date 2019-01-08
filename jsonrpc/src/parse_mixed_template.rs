use serde_json::Value;

#[derive(PartialEq, Debug)]
pub enum TemplatePlace {
    Any,
    Pin(Value),
    Capture(String),
}

#[derive(PartialEq, Debug)]
pub struct TemplateAndPlaces {
	template: String,
	places: Vec<TemplatePlace>,
}

impl TemplateAndPlaces {
    fn new<T: Into<String>>(template: T, places: Vec<TemplatePlace>) -> Self {
        TemplateAndPlaces {
            template: template.into(),
            places: places,
        }
    }
}

pub fn parse_template_and_values(s: &str) -> Result<TemplateAndPlaces, String> {
	let mut places: Vec<TemplatePlace> = vec![];

	let mut iter = s.chars().enumerate();

	let mut template = String::new();

	while let Some((i, c)) = iter.next() {
		match c {
			'(' => {
				// JSON
				template.push('_');
				while let Some((j, c)) = iter.next() {
					if c == '"' {
						// quoted
						while let Some((_, c)) = iter.next() {
							if c == '\\' {
								// escape next
								iter.next();
							} else if c == '"' {
								// end quote
								break;
							}
						}
					} else if c == ')' {
						// end paren
                        let json_value_str = s.get(i + 1..j).unwrap();
                        let json_value: Value = serde_json::from_str(json_value_str)
                            .map_err(|parse_error| format!("Error parsing template's JSON ({}, {}): {}", i, j, parse_error))?;
						places.push(TemplatePlace::Pin(json_value));
						break;
					} else if c == '(' {        
                        // ERROR
                        return Err(format!("Unexpected '(' in template while skimming JSON from position {} to {}", i, j));
                    }
				}
			},
            '/' => {
                // CAPTURE
                template.push('_');
				while let Some((j, c)) = iter.next() {
					if c == '/' {
						// end capture
						places.push(TemplatePlace::Capture(String::from(s.get(i + 1..j).unwrap())));
                        break;
					}
				}
            },
			'_' => {
				// IGNORED
				template.push('_');
				places.push(TemplatePlace::Any);
			},
            '"' => {
                // quote string start
				template.push('_');
                while let Some((j, c)) = iter.next() {
                    if c == '\\' {
                        // escape next
                        iter.next();
                    } else if c == '"' {
                        // end quote
                        places.push(TemplatePlace::Pin(Value::from(s.get(i+1..j).unwrap())));
                        break;
                    }
                }
            },
            ')' => {
                // ERROR
                return Err(format!("Unexpected ')' in template at position: {}", i));
            },
			valid_char @ 'a'...'z' | valid_char @ 'A'...'Z' | valid_char @ ' ' => template.push(valid_char),
			any_char => {
                // ERROR
                return Err(format!("Unexpected '{}' in template at position: {}", any_char, i));
            },
		}
	}

	Ok(TemplateAndPlaces {
		template: template,
		places: places,
	})
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::TemplatePlace::*;

    #[test]
    fn test_parse() {
        parse_expect(
            r#"("A") points ("up") at ("B")"#,
            test_tap("_ points _ at _", vec!["\"A\"", "\"up\"", "\"B\""]),
        );
        parse_expect(
            r#"Saw (1) at ("B")"#,
            test_tap("Saw _ at _", vec!["1", "\"B\""]),
        );
        parse_expect(
            r#"("B") claims ("C") is mapbox of ({"lat":12})"#,
            test_tap("_ claims _ is mapbox of _", vec!["\"B\"", "\"C\"", "{\"lat\":12}"]),
        );
        parse_expect(
            r#"("B") ("\"") (")\\)\\((")"#,
            test_tap("_ _ _", vec!["\"B\"", "\"\\\"\"", r#"")\\)\\((""#]),
        );
    }

    #[test]
    fn test_capture() {
        parse_expect(
            r#"/page/ points ("up") at /target/"#,
            TemplateAndPlaces::new("_ points _ at _", vec![Capture(String::from("page")), Pin(Value::from("up")), Capture(String::from("target"))]),
        )
    }

    #[test]
    fn test_pin_str() {
        parse_expect(
            r#"/page/ points "up" at /target/"#,
            TemplateAndPlaces::new("_ points _ at _", vec![Capture(String::from("page")), Pin(Value::from("up")), Capture(String::from("target"))]),
        );
        parse_expect(
            r#""A" points "up" at "B""#,
            test_tap("_ points _ at _", vec!["\"A\"", "\"up\"", "\"B\""]),
        );
    }

    fn parse_expect(template: &str, expects: TemplateAndPlaces) {
        assert_eq!(parse_template_and_values(template).expect("parses"), expects);
    }

    fn test_tap(template: &str, vals: Vec<&str>) -> TemplateAndPlaces {
        TemplateAndPlaces {
            template: String::from(template),
            places: vals.into_iter()
                .map(|s| Pin(serde_json::from_str(s).unwrap()))
                .collect(),
        }
    }
}
