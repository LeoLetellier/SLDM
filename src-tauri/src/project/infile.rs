use std::fs;
// I/O management

pub(crate) struct FileReader {
    path: String,
    sep: char,
    first_line: usize,
    last_line: usize,
    use_columns: Vec<usize>,
    n_column: Option<usize>, // use_columns.len()
    n_line: Option<usize>, // last_line - first_line + 1
}

impl FileReader{
    pub fn new(path: String, sep: char, first_line: usize) -> FileReader {
        FileReader{ 
            path,
            sep,
            first_line,
            last_line: first_line,
            use_columns: vec![0],
            n_column: None,
            n_line: Some(1),
        }
    }

    pub fn parse(&self) -> Vec<Vec<String>> {
        let content = fs::read_to_string(&self.path).unwrap();
        let mut line_it = content.lines();
        past_lines(&mut line_it, self.first_line);
        parse_lines(&mut line_it)
    }

    pub fn parse_unpack(&self) -> Vec<Vec<f64>> {
        let output = self.parse();
        unpack_to_digit(&output)

    }
    

}

fn past_lines(lines: &mut core::str::Lines<'_>, n: usize) {
    for _ in 0..n {
        lines.next();
    }
}

fn parse_lines(lines: &mut core::str::Lines<'_>) -> Vec<Vec<String>> {
    lines.map(|line| {
            // For each line in the source
            line.to_string()
                .split(';') // Split the line separated by commas
                .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                .collect() // Collect all strings in a row into a Vec<String>
            })
        .collect() // Collect all lines into a Vec<Vec<String>>
}

fn unpack_to_digit(vecs: &Vec<Vec<String>>) -> Vec<Vec<f64>> {
    let n_vecs = vecs[0].len();
    let n_lines = vecs.len();
    for k in 1..n_lines {
        assert_eq!(vecs[k-1].len(), vecs[k].len());
    }
    let mut res: Vec<Vec<f64>> = vec![];
    for v in 0..n_vecs {
        res.push(vec![]);
        for l in 0..n_lines {
            res[v].push(vecs[l][v].clone().parse::<f64>().unwrap());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::FileReader;

    #[test]
    fn t_read_csv() {
        let reader = FileReader::new(String::from("test/dem.csv"), ';', 1);
        let result = reader.parse();
        let expect = vec![vec!["0", "0"], vec!["100", "10"], vec!["200", "30"], vec!["300", "35"], vec!["400", "45"], vec!["500", "50"], vec!["600", "60"]];
        assert_eq!(result, expect, "File not read properly");
    }

    #[test]
    fn t_unpack() {
        let reader = FileReader::new(String::from("test/dem.csv"), ';', 1);
        let result = reader.parse_unpack();
        let expect = vec![vec![0., 100., 200., 300., 400., 500., 600.], vec![0., 10., 30., 35., 45., 50., 60.]];
        assert_eq!(result, expect, "File not read properly");
    }
}