use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    ops::RangeInclusive,
};

pub type Coord = [isize; 2];

/// A Grid trait
pub trait Grid<V = char> {
    fn from_str_with<F: Fn(char) -> Option<V>>(input: &str, f: F) -> Self;

    fn bounds(&self) -> [RangeInclusive<isize>; 2];

    fn visualise(&self) -> String;
}

impl<V> Grid<V> for BTreeMap<Coord, V>
where
    V: ToString,
{
    fn from_str_with<F: Fn(char) -> Option<V>>(input: &str, f: F) -> BTreeMap<Coord, V> {
        let mut out = BTreeMap::new();

        for (row, s) in input.lines().enumerate() {
            for (col, cha) in s.chars().enumerate() {
                if let Some(v) = f(cha) {
                    out.insert([col as isize, row as isize], v);
                }
            }
        }
        out
    }
    fn bounds(&self) -> [RangeInclusive<isize>; 2] {
        let xmax = self.iter().map(|(v, _)| v[0]).max().unwrap_or(0);
        let ymax = self.iter().map(|(v, _)| v[1]).max().unwrap_or(0);
        let xmin = self.iter().map(|(v, _)| v[0]).min().unwrap_or(0);
        let ymin = self.iter().map(|(v, _)| v[1]).min().unwrap_or(0);

        [xmin..=xmax, ymin..=ymax]
    }

    /// Visualise the grid. Note that only the first character is used.
    fn visualise(&self) -> String {
        let mut out = String::new();
        let [xs, ys] = <BTreeMap<Coord, V> as Grid<V>>::bounds(self);

        for y in ys {
            for x in xs.clone() {
                if let Some(v) = self.get(&[x, y]) {
                    out.push(v.to_string().chars().nth(0).unwrap_or('#'));
                } else {
                    out.push('.');
                }
            }
            out.push('\n');
        }
        out
    }
}
impl<V> Grid<V> for BTreeSet<Coord> {
    /// Note: the value of V is disregarded
    fn from_str_with<F: Fn(char) -> Option<V>>(input: &str, f: F) -> BTreeSet<Coord> {
        let mut out = BTreeSet::new();

        for (row, s) in input.lines().enumerate() {
            for (col, cha) in s.chars().enumerate() {
                if f(cha).is_some() {
                    out.insert([col as isize, row as isize]);
                }
            }
        }
        out
    }
    fn bounds(&self) -> [RangeInclusive<isize>; 2] {
        let xmax = self.iter().map(|v| v[0]).max().unwrap_or(0);
        let ymax = self.iter().map(|v| v[1]).max().unwrap_or(0);
        let xmin = self.iter().map(|v| v[0]).min().unwrap_or(0);
        let ymin = self.iter().map(|v| v[1]).min().unwrap_or(0);

        [xmin..=xmax, ymin..=ymax]
    }

    fn visualise(&self) -> String {
        let mut out = String::new();
        let [xs, ys] = <BTreeSet<[isize; 2]> as Grid>::bounds(self);

        for y in ys {
            for x in xs.clone() {
                if self.contains(&[x, y]) {
                    out.push('#');
                } else {
                    out.push('.');
                }
            }
            out.push('\n');
        }
        out
    }
}

impl<V> Grid<V> for HashSet<Coord> {
    /// Note: the value of V is disregarded
    fn from_str_with<F: Fn(char) -> Option<V>>(input: &str, f: F) -> HashSet<Coord> {
        let mut out = HashSet::new();

        for (row, s) in input.lines().enumerate() {
            for (col, cha) in s.chars().enumerate() {
                if f(cha).is_some() {
                    out.insert([col as isize, row as isize]);
                }
            }
        }
        out
    }
    fn bounds(&self) -> [RangeInclusive<isize>; 2] {
        let xmax = self.iter().map(|v| v[0]).max().unwrap_or(0);
        let ymax = self.iter().map(|v| v[1]).max().unwrap_or(0);
        let xmin = self.iter().map(|v| v[0]).min().unwrap_or(0);
        let ymin = self.iter().map(|v| v[1]).min().unwrap_or(0);

        [xmin..=xmax, ymin..=ymax]
    }

    fn visualise(&self) -> String {
        let mut out = String::new();
        let [xs, ys] = <HashSet<[isize; 2]> as Grid<V>>::bounds(self);

        for y in ys {
            for x in xs.clone() {
                if self.contains(&[x, y]) {
                    out.push('#');
                } else {
                    out.push('.');
                }
            }
            out.push('\n');
        }
        out
    }
}

impl<V> Grid<V> for HashMap<Coord, V>
where
    V: ToString,
{
    /// Note: the value of V is disregarded
    fn from_str_with<F: Fn(char) -> Option<V>>(input: &str, f: F) -> HashMap<Coord, V> {
        let mut out = HashMap::new();

        for (row, s) in input.lines().enumerate() {
            for (col, cha) in s.chars().enumerate() {
                if let Some(v) = f(cha) {
                    out.insert([col as isize, row as isize], v);
                }
            }
        }
        out
    }
    fn bounds(&self) -> [RangeInclusive<isize>; 2] {
        let xmax = self.iter().map(|(v, _)| v[0]).max().unwrap_or(0);
        let ymax = self.iter().map(|(v, _)| v[1]).max().unwrap_or(0);
        let xmin = self.iter().map(|(v, _)| v[0]).min().unwrap_or(0);
        let ymin = self.iter().map(|(v, _)| v[1]).min().unwrap_or(0);

        [xmin..=xmax, ymin..=ymax]
    }

    /// Visualise the grid. Note that only the first character is used.
    fn visualise(&self) -> String {
        let mut out = String::new();
        let [xs, ys] = <HashMap<Coord, V> as Grid<V>>::bounds(self);

        for y in ys {
            for x in xs.clone() {
                if let Some(v) = self.get(&[x, y]) {
                    out.push(v.to_string().chars().nth(0).unwrap_or('#'));
                } else {
                    out.push('.');
                }
            }
            out.push('\n');
        }
        out
    }
}

/// Elementwise add
pub fn add(a: Coord, b: Coord) -> Coord {
    [a[0] + b[0], a[1] + b[1]]
}

/// Elementwise subtract
pub fn subtract(a: Coord, b: Coord) -> Coord {
    [a[0] - b[0], a[1] - b[1]]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROUND_TRIP: &str = r"#####
..#..
..#..
#.#..
.#...";

    #[test]
    fn round_trip() {
        let hs = <HashSet<Coord> as Grid>::from_str_with(ROUND_TRIP, |c| match c {
            '.' => None,
            e => Some(e),
        });
        let bs = <BTreeSet<Coord> as Grid>::from_str_with(ROUND_TRIP, |c| match c {
            '.' => None,
            e => Some(e),
        });
        let hm = <HashMap<Coord, char> as Grid>::from_str_with(ROUND_TRIP, |c| match c {
            '.' => None,
            e => Some(e),
        });
        let bm = <BTreeMap<Coord, char> as Grid>::from_str_with(ROUND_TRIP, |c| match c {
            '.' => None,
            e => Some(e),
        });

        assert_eq!(
            ROUND_TRIP,
            <HashSet<Coord> as Grid>::visualise(&hs).as_str().trim()
        );
        assert_eq!(
            ROUND_TRIP,
            <BTreeSet<Coord> as Grid>::visualise(&bs).as_str().trim()
        );
        assert_eq!(
            ROUND_TRIP,
            <HashMap<Coord, char> as Grid>::visualise(&hm)
                .as_str()
                .trim()
        );

        // the future is here
        assert_eq!(ROUND_TRIP, bm.visualise().as_str().trim());
    }
}
