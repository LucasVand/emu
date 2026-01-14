use std::{
    cmp::min,
    fmt::Display,
    ops::{Index, IndexMut},
};

pub struct LevenshteinDistance {}

impl LevenshteinDistance {
    pub fn distance(str1: &str, str2: &str) -> usize {
        let str1_len = str1.len();
        let str2_len = str2.len();

        let mut mat = Matrix::new(str1_len + 1, str2_len + 1);

        // create the starting rows
        for i in 0..(str1_len + 1) {
            mat[i][0] = i;
        }
        for i in 0..(str2_len + 1) {
            mat[0][i] = i;
        }

        for i in 1..(str1_len + 1) {
            for j in 1..(str2_len + 1) {
                let letter1 = str1.chars().nth(i - 1).unwrap();
                let letter2 = str2.chars().nth(j - 1).unwrap();
                let cost = if letter1 == letter2 { 0 } else { 1 };

                let delete = mat[i - 1][j] + 1;
                let insert = mat[i][j - 1] + 1;
                let sub = mat[i - 1][j - 1] + cost;
                mat[i][j] = min(delete, min(insert, sub));
            }
        }

        return mat[str1_len][str2_len];
    }
}

struct Matrix {
    storage: Vec<usize>,
    length: usize,
    width: usize,
}

impl Matrix {
    pub fn new(length: usize, width: usize) -> Matrix {
        let mut storage = Vec::new();
        for _ in 0..(length * width) {
            storage.push(0);
        }

        Matrix {
            storage: storage,
            length: length,
            width: width,
        }
    }
}
impl Index<usize> for Matrix {
    type Output = [usize];

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
