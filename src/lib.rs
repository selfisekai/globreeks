//! globreeks is a thing that gets globs and checks if they match.
//!
//! glob order matters here. out of real examples, one can set the globs to
//! `!**/node_modules/**/build/**` and then
//! `node_modules/@signalapp/better-sqlite3/build/Release/better_sqlite3.node`,
//! as to make an exception from the previously forbidden pattern.
//!
//! see [Globreeks] for the actual implementation.

pub use anyhow::{Error, Result};
pub use globset::{Candidate, Glob, GlobMatcher};

#[derive(PartialEq, Eq)]
enum Conclusion {
    Matches,
    Exclusion,
    NonMatching,
}

#[derive(Debug, Clone)]
struct Pattern {
    glob: GlobMatcher,
    negative: bool,
}

impl Pattern {
    fn new(glob: &str) -> Result<Self> {
        let (negative, glob) = glob
            .strip_prefix('!')
            .map(|st| (true, st))
            .unwrap_or_else(|| (false, glob));
        let glob = Glob::new(glob)?.compile_matcher();
        Ok(Pattern { glob, negative })
    }

    fn matches(&self, path: &Candidate) -> Conclusion {
        match (self.glob.is_match_candidate(path), self.negative) {
            (true, false) => Conclusion::Matches,
            (true, true) => Conclusion::Exclusion,
            (false, _) => Conclusion::NonMatching,
        }
    }
}

#[derive(Debug, Clone)]
/// the thing. create a new one with [Globreeks::new].
/// use it with [Globreeks::evaluate].
pub struct Globreeks {
    patterns: Vec<Pattern>,
}

impl Globreeks {
    /// give it the patterns to check paths with.
    pub fn new<L>(glob_list: L) -> Result<Globreeks>
    where
        L: IntoIterator,
        L::Item: AsRef<str>,
    {
        let mut patterns = Vec::new();
        for glob in glob_list.into_iter() {
            patterns.push(Pattern::new(glob.as_ref())?);
        }
        Ok(Globreeks { patterns })
    }

    /// checks whether the supplied path matches the patterns supplied in [Globreeks::new].
    ///
    /// if you want to supply &str rather than [Candidate], see: [Globreeks::evaluate].
    pub fn evaluate_candidate<'a>(&self, path: &Candidate<'a>) -> bool {
        self.patterns
            .iter()
            .map(|pattern| pattern.matches(path))
            .filter(|m| *m != Conclusion::NonMatching)
            .last()
            == Some(Conclusion::Matches)
    }

    /// checks whether the supplied path matches the patterns supplied in [Globreeks::new].
    ///
    /// internally converts the str argument to [Candidate]. also see: [Globreeks::evaluate_candidate].
    pub fn evaluate<S>(&self, path: S) -> bool
    where
        S: AsRef<str>,
    {
        self.evaluate_candidate(&Candidate::new(path.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use crate::Globreeks;
    use anyhow::Result;

    #[test]
    fn test_basic() -> Result<()> {
        let reeks = Globreeks::new(["**/*.{js,ts}", "!bundle*.js", "bundle1.js"])?;

        assert!(reeks.evaluate("somewhere/some_file.js"));
        assert!(!reeks.evaluate("bundle2137.js"));
        assert!(reeks.evaluate("bundle1.js"));
        assert!(!reeks.evaluate("readme.txt"));

        Ok(())
    }
}
