pub fn convert_iters(s: String, num_rows: i32) -> String {
    if num_rows == 1 { return s; }

    // Precalculate some stuff
    let num_rows = num_rows as usize;
    let s = s.chars().collect::<Vec<_>>();
    let step_size = 2 * (num_rows - 1);

    let mut out = String::new();
    for row in 0..num_rows {
        if row == 0 || row == num_rows - 1 {
            // Sample 1 char
            // [row;2*(num_rows-1)]
            (row..s.len())
                .step_by(step_size)
                .for_each(|i| out.push(s[i]));
        } else {
            // Sample 2 chars

            // a [row, 2*(num_rows-1)]
            let mut a = (row..s.len())
                .step_by(step_size);

            // b [2*(num_rows-1)-row, 2*(num_rows-1)]
            let mut b = ((step_size - row)..s.len())
                .step_by(step_size);


            // interleave a,b
            loop {
                match a.next() {
                    Some(i) => out.push(s[i]),
                    None => break
                }
                match b.next() {
                    Some(i) => out.push(s[i]),
                    None => break
                }
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let solution = "PAHNAPLSIIGYIR".to_string();

        let guess = convert_iters(input, num_rows);
        assert_eq!(guess, solution)
    }

    #[test]
    fn example2() {
        let input = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        let solution = "PINALSIGYAHRPI".to_string();

        let guess = convert_iters(input, num_rows);
        assert_eq!(guess, solution)
    }

    #[test]
    fn example3() {
        let input = "A".to_string();
        let num_rows = 1;
        let solution = "A".to_string();

        let guess = convert_iters(input, num_rows);
        assert_eq!(guess, solution)
    }

    #[test]
    fn worst_case() {
        let input = "A".repeat(1000).to_string();
        let num_rows = 1000;
        let solution = "A".repeat(1000).to_string();

        let guess = convert_iters(input, num_rows);
        assert_eq!(guess, solution)
    }
}
