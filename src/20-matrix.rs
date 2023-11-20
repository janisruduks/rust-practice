fn main() {
    let x = Matrix::new("1");
    assert_eq!(x.row(0), vec![1]);
    let y = Matrix::new("1 2 3\n4 5 6\n7 8 9\n8 7 6");
    assert_eq!(y.row(2), vec![7, 8, 9]);
    assert_eq!(y.columns(1), vec![2, 5, 8, 7]);
    let a = Matrix::new("89 1903 3\n18 3 1\n9 4 800");
    assert_eq!(a.columns(1), vec![1903, 3, 4]);
    assert_eq!(a.row(0), vec![89, 1903, 3]);
}

struct Matrix {
    matrix: Vec<Vec<u32>>,
}

impl Matrix {
    fn new(input: &str) -> Self {
        let mut matrix: Vec<Vec<u32>> = Vec::new();
        for line in input.lines() {
            let row: Vec<u32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<u32>().ok())
                .collect();
            matrix.push(row);
        }

        Matrix { matrix }
    }

    fn columns(&self, number: usize) -> Vec<u32> {
        let mut col_vec: Vec<u32> = Vec::new();
        for row in &self.matrix {
            if number < row.len() {
                col_vec.push(row[number]);
            } else {
                col_vec.push(0);
            }
        }
        col_vec
    }

    fn row(&self, number: usize) -> Vec<u32> {
        self.matrix[number].clone()
    }
}
