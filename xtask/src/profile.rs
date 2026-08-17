pub struct Profile {
    pub name: &'static str,
    pub checks: &'static [&'static str],
}

pub const QUICK: Profile = Profile {
    name: "quick",
    checks: &["rfc", "cargo-fmt"],
};

pub const CI: Profile = Profile {
    name: "ci",
    checks: &[
        "rfc",
        "rfc-self-test",
        "cargo-build",
        "cargo-test",
        "cargo-clippy",
        "cargo-fmt",
    ],
};

pub const RELEASE: Profile = Profile {
    name: "release",
    checks: CI.checks,
};

pub const ALL: &[Profile] = &[QUICK, CI, RELEASE];

pub fn by_name(name: &str) -> Option<&'static Profile> {
    ALL.iter().find(|profile| profile.name == name)
}
