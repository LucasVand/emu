use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Index, IndexMut},
    sync::LazyLock,
};

pub struct LevenshteinDistance {}

impl LevenshteinDistance {
    pub fn distance_no_case(str1: &str, str2: &str) -> f32 {
        Self::distance(&str1.to_lowercase(), &str2.to_lowercase())
    }
    pub fn distance(str1: &str, str2: &str) -> f32 {
        let str1_len = str1.len();
        let str2_len = str2.len();

        let mut mat = Matrix::new(str1_len + 1, str2_len + 1);

        // create the starting rows
        for i in 0..(str1_len + 1) {
            mat[i][0] = i as f32;
        }
        for i in 0..(str2_len + 1) {
            mat[0][i] = i as f32;
        }

        for i in 1..(str1_len + 1) {
            for j in 1..(str2_len + 1) {
                let letter1 = str1.chars().nth(i - 1).unwrap();
                let letter2 = str2.chars().nth(j - 1).unwrap();
                let cost: f32 = if letter1 == letter2 {
                    0.0
                } else if letter1.to_ascii_lowercase() == letter2.to_ascii_lowercase() {
                    0.25
                } else if Self::char_keyboard_dist(
                    letter1.to_ascii_lowercase(),
                    letter2.to_ascii_lowercase(),
                ) == 1
                {
                    0.5
                } else {
                    1.0
                };

                let delete: f32 = mat[i - 1][j] + 1.0;
                let insert: f32 = mat[i][j - 1] + 1.0;
                let sub = mat[i - 1][j - 1] + cost;

                let transpost: f32 = if i > 1
                    && j > 1
                    && str1.chars().nth(i - 1) == str2.chars().nth(j - 2)
                    && str1.chars().nth(i - 2) == str2.chars().nth(j - 1)
                {
                    mat[i - 2][j - 2] + 1.0
                } else {
                    10000.0
                };
                mat[i][j] = f32::min(f32::min(delete, transpost), f32::min(insert, sub));
            }
        }

        return mat[str1_len][str2_len];
    }
    fn char_keyboard_dist(ch1: char, ch2: char) -> usize {
        static KEYBOARD_MAP: LazyLock<HashMap<char, (usize, usize)>> = LazyLock::new(|| {
            let mut map = HashMap::new();

            for (r, row) in QWERTY.iter().enumerate() {
                for (c, &ch) in row.iter().enumerate() {
                    if let Some(ch) = ch {
                        map.insert(ch, (r as usize, c as usize));
                    }
                }
            }

            map
        });

        let pos1 = KEYBOARD_MAP.get(&ch1);
        let pos2 = KEYBOARD_MAP.get(&ch2);
        if pos1.is_none() || pos2.is_none() {
            return 1000;
        }
        let pos1 = pos1.unwrap();
        let pos2 = pos2.unwrap();

        let dist = isize::abs(pos1.0 as isize - pos2.0 as isize)
            + isize::abs(pos1.1 as isize - pos2.1 as isize);

        return dist as usize;
    }
}

struct Matrix {
    storage: Vec<f32>,
    length: usize,
    width: usize,
}

impl Matrix {
    pub fn new(length: usize, width: usize) -> Matrix {
        let mut storage = Vec::new();
        for _ in 0..(length * width) {
            storage.push(0.0);
        }

        Matrix {
            storage: storage,
            length: length,
            width: width,
        }
    }
}
impl Index<usize> for Matrix {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        let low = index * self.width;
        let high = low + self.width;
        return &self.storage[low..high];
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let low = index * self.width;
        let high = low + self.width;
        return &mut self.storage[low..high];
    }
}
impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.length {
            let row = &self[i];
            write!(f, "[")?;
            for j in row {
                write!(f, "{}", j)?;
            }
            write!(f, "]\n")?;
        }

        Ok(())
    }
}
const QWERTY: [[Option<char>; 10]; 3] = [
    [
        Some('q'),
        Some('w'),
        Some('e'),
        Some('r'),
        Some('t'),
        Some('y'),
        Some('u'),
        Some('i'),
        Some('o'),
        Some('p'),
    ],
    [
        None,
        Some('a'),
        Some('s'),
        Some('d'),
        Some('f'),
        Some('g'),
        Some('h'),
        Some('j'),
        Some('k'),
        Some('l'),
    ],
    [
        None,
        None,
        Some('z'),
        Some('x'),
        Some('c'),
        Some('v'),
        Some('b'),
        Some('n'),
        Some('m'),
        None,
    ],
];
