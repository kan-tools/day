use std::fmt;

pub const PREFIX: &str = "Demonstrated-by:";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Demonstration {
    pub tests: Vec<String>,
    pub include: Option<String>,
    pub outcome: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Missing,
    Multiple,
    Malformed(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing => write!(f, "commit carries no `{PREFIX}` trailer"),
            Self::Multiple => write!(f, "commit carries more than one `{PREFIX}` trailer"),
            Self::Malformed(detail) => write!(f, "malformed `{PREFIX}` trailer: {detail}"),
        }
    }
}

pub fn parse_message(message: &str) -> Result<Demonstration, ParseError> {
    let mut candidates = message.lines().filter(|line| line.starts_with(PREFIX));
    let line = candidates.next().ok_or(ParseError::Missing)?;
    if candidates.next().is_some() {
        return Err(ParseError::Multiple);
    }
    parse_line(line)
}

pub fn parse_line(line: &str) -> Result<Demonstration, ParseError> {
    let body = line
        .strip_prefix(PREFIX)
        .ok_or_else(|| ParseError::Malformed("prefix must start the line".to_owned()))?
        .trim();
    let fields = body.split_whitespace().collect::<Vec<_>>();
    if !(fields.len() == 3 || fields.len() == 4) {
        return Err(ParseError::Malformed(
            "expected revert, tests, optional include, and outcome fields".to_owned(),
        ));
    }
    if fields[0] != "revert=HEAD" {
        return Err(ParseError::Malformed(
            "`revert` must be the literal `HEAD`".to_owned(),
        ));
    }

    let tests = value(fields[1], "tests")?
        .split(',')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if tests.iter().any(|test| test.is_empty()) {
        return Err(ParseError::Malformed(
            "`tests` must be a comma-separated list with no empty member".to_owned(),
        ));
    }

    let (include, outcome_field) = if fields.len() == 4 {
        (Some(value(fields[2], "include")?.to_owned()), fields[3])
    } else {
        (None, fields[2])
    };
    let outcome = value(outcome_field, "outcome")?;
    if !outcome
        .chars()
        .all(|character| character.is_ascii_uppercase() || character == '-')
    {
        return Err(ParseError::Malformed(
            "`outcome` must contain only uppercase ASCII letters and hyphens".to_owned(),
        ));
    }

    Ok(Demonstration {
        tests,
        include,
        outcome: outcome.to_owned(),
    })
}

fn value<'a>(field: &'a str, name: &str) -> Result<&'a str, ParseError> {
    let prefix = format!("{name}=");
    let value = field
        .strip_prefix(&prefix)
        .ok_or_else(|| ParseError::Malformed(format!("expected `{name}=...`")))?;
    if value.is_empty() {
        return Err(ParseError::Malformed(format!("`{name}` may not be empty")));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_and_scoped_trailers_share_one_grammar() {
        let plain =
            parse_line("Demonstrated-by: revert=HEAD tests=a::b outcome=DEMONSTRATED").unwrap();
        assert_eq!(plain.tests, ["a::b"]);
        assert_eq!(plain.include, None);

        let scoped = parse_line(
            "Demonstrated-by: revert=HEAD tests=a::b,c::d include=src/lib.rs outcome=DEMONSTRATED",
        )
        .unwrap();
        assert_eq!(scoped.tests, ["a::b", "c::d"]);
        assert_eq!(scoped.include.as_deref(), Some("src/lib.rs"));
    }

    #[test]
    fn fabricated_or_ambiguous_claims_are_rejected() {
        for malformed in [
            "Demonstrated-by: I reverted it and it failed",
            "Demonstrated-by: revert=main tests=a outcome=DEMONSTRATED",
            "Demonstrated-by: revert=HEAD tests=a,,b outcome=DEMONSTRATED",
            "Demonstrated-by: revert=HEAD tests=a outcome=demonstrated",
            "prefix Demonstrated-by: revert=HEAD tests=a outcome=DEMONSTRATED",
        ] {
            assert!(parse_line(malformed).is_err(), "accepted {malformed}");
        }
        assert_eq!(
            parse_message(
                "Demonstrated-by: revert=HEAD tests=a outcome=DEMONSTRATED\n\
                 Demonstrated-by: revert=HEAD tests=b outcome=DEMONSTRATED"
            ),
            Err(ParseError::Multiple)
        );
    }

    #[test]
    fn parsing_preserves_domain_outcomes_for_the_caller_to_classify() {
        let claim = parse_line("Demonstrated-by: revert=HEAD tests=a outcome=VACUOUS").unwrap();
        assert_eq!(claim.outcome, "VACUOUS");
    }
}
