use std::{
    convert::{AsRef, From},
    fmt::{self, Display},
    fs,
    path::Path,
};

#[derive(Default)]
pub struct Metric {
    pub value: u64,
}

impl Metric {
    pub fn init<P: AsRef<Path>>(path: P) -> Option<Self> {
        let data = fs::read(path.as_ref()).ok()?;

        let lines = data.into_iter().fold(0, |acc, item| {
            if u32::from(item) == u32::from('\n') {
                acc + 1
            } else {
                acc
            }
        });

        u64::try_from(lines).map(|value| Self { value }).ok()
    }
}

impl From<u64> for Metric {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <u64 as Display>::fmt(&self.value, f)
    }
}

#[test]
fn test_line_count() {
    let metric = Metric::init("tests/data/nemesis.txt")
        .expect("Expected 'tests/data/nemesis.txt' to exist");

    assert_eq!(metric.value, 4);
}
