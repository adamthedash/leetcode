use std::cmp::{min, Ordering};
use std::ops::Div;

fn brute_force(height: Vec<i32>) -> i32 {
    let height = height.iter().map(|&h| h as usize).collect::<Vec<_>>();

    (0..height.len()).map(|i| {
        (0..height.len())
            .filter(|&j| height[j] <= height[i])
            .map(|j| j.abs_diff(i) * height[j])
            .max().unwrap()
    }).max().unwrap() as i32
}

fn div_ceil(a: i32, b: i32) -> i32 {
    if a % b > 0 {
        a / b + 1
    } else {
        a / b
    }
}

fn brute_force_early_stop(height: Vec<i32>) -> i32 {
    let mut current_area = 0;

    for i in 0..height.len() - 1 {
        if height[i] == 0 { continue; };

        // Will need to be at least this length if we want to beat the best one
        let min_len = div_ceil(current_area, height[i]) as usize;
        if i + min_len >= height.len() { continue; }


        let candiate_area = (i + min_len..height.len())
            .map(|j| (j - i) as i32 * min(height[i], height[j]))
            .max();


        if candiate_area.is_some_and(|a| a > current_area) {
            current_area = candiate_area.unwrap();
        }
    }

    current_area
}

fn calculate_potential(height: Vec<i32>) -> i32 {
    // Calculate potentials
    let mut potentials = height.iter()
        .enumerate()
        .map(|(i, h)| (i, h * (height.len() - i) as i32))
        .collect::<Vec<_>>();

    // Sort them descending
    potentials.sort_unstable_by(|(i, a), (j, b)| b.cmp(a));

    fn check(height: &[i32], min_area: i32, i: usize) -> i32 {
        // Will need to be at least this length if we want to beat the best one
        let min_len = div_ceil(min_area, height[i]) as usize;
        if i + min_len >= height.len() { return 0; }

        let candiate_area = (i + min_len..height.len())
            .map(|j| (j - i) as i32 * min(height[i], height[j]))
            .max();

        if candiate_area.is_some() {
            candiate_area.unwrap()
        } else {
            0
        }
    }

    let mut best_area = 0;
    for (i, potential) in potentials {
        if potential <= best_area { break; }

        let area = check(&height, best_area, i);
        if area > best_area { best_area = area }
    }


    return best_area;
}

fn sorted_brute_force(height: Vec<i32>) -> i32 {
    let mut height = height.iter()
        .enumerate()
        .collect::<Vec<_>>();
    height.sort_unstable_by(|(i, a), (j, b)| a.cmp(b));


    let best_area = height.iter()
        .enumerate()
        .map(|(i, (x1, &h1))| {
            height[i..].iter()
                .map(|(x2, h2)| x1.abs_diff(*x2))
                .max().unwrap() as i32 * h1
        })
        .max().unwrap();

    best_area
}

fn carried_max(height: Vec<i32>) -> i32 {
    let mut maxes = vec![(0, height[0])];

    let mut best_area = 0;
    for (x1, &h1) in height.iter().enumerate().skip(1) {
        // Reverse crawl saved heights to find best
        let area = maxes.iter()
            .rev()
            .take_while(|(x2, h2)| min(h1, *h2) * (x1 - x2) as i32 > best_area)
            .map(|(x2, h2)| min(h1, *h2) * (x1 - x2) as i32)
            .max().unwrap();

        if area > best_area {
            best_area = area
        }

        // Add height if its bigger
        if h1 > maxes.last().unwrap().1 {
            maxes.push((x1, h1));
        }
    }

    best_area
}

fn end_to_end(height: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = height.len() - 1;

    let mut best_area = min(height[i], height[j]) * (j - i) as i32;

    loop {
        match height[i].cmp(&height[j]) {
            // Move left stick up
            Ordering::Less => {
                let new_i = (i + 1..j)
                    .find(|&new_i| height[new_i] > height[i]);

                match new_i {
                    None => break,
                    Some(ii) => i = ii
                }
            }
            // Move right stick down
            Ordering::Greater => {
                let new_j = (i + 1..j).rev()
                    .find(|&new_j| height[new_j] > height[j]);

                match new_j {
                    None => break,
                    Some(jj) => j = jj
                }
            }
            // Find the next bigger stick from either end, taking the one requiring less movement
            Ordering::Equal => {
                let new_i = (i + 1..j)
                    .find(|&x| height[x] > height[i]);

                if new_i.is_none() { break; }
                let new_i = new_i.unwrap();

                let new_j = (i + 1..j).rev()
                    .find(|&x| height[x] > height[j])
                    .unwrap();

                match (new_i - i).cmp(&(new_j - j)) {
                    Ordering::Less |
                    Ordering::Equal => i = new_i,
                    Ordering::Greater => j = new_j
                }
            }
        }

        let new_area = min(height[i], height[j]) * (j - i) as i32;

        if new_area > best_area {
            best_area = new_area
        }

        if j - i == 1 { break; }
    }


    best_area
}


pub fn max_area(height: Vec<i32>) -> i32 {
    end_to_end(height)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let solution = 49;

        let guess = max_area(input);

        assert_eq!(guess, solution)
    }

    #[test]
    fn example2() {
        let input = vec![1, 1];
        let solution = 1;

        let guess = max_area(input);

        assert_eq!(guess, solution)
    }

    #[test]
    fn example3() {
        let input = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let solution = 25;

        let guess = max_area(input);

        assert_eq!(guess, solution)
    }

    #[test]
    fn example4() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 25, 7];
        let solution = 49;

        let guess = max_area(input);

        assert_eq!(guess, solution)
    }

    #[test]
    fn worst_case() {
        let mut input = Vec::new();
        input.push(10);
        for _ in 0..10_i32.pow(4) - 2 {
            input.push(1)
        }
        input.push(10);

        let solution = 10 * (10_i32.pow(4) - 1);


        let guess = max_area(input);

        assert_eq!(guess, solution)
    }

    #[test]
    fn staircase_up() {
        let mut input = Vec::new();
        for i in 0..10_i32.pow(4) {
            input.push(i);
        }

        let solution = 24995000;


        let guess = max_area(input);

        assert_eq!(guess, solution)
    }

    #[test]
    fn staircase_down() {
        let mut input = Vec::new();
        for i in (0..10_i32.pow(4)).rev() {
            input.push(i);
        }


        let solution = 24995000;


        let guess = max_area(input);

        assert_eq!(guess, solution)
    }

    #[test]
    fn big_end() {
        let mut input = Vec::new();
        for _ in 0..10 {
            input.push(1)
        }
        input.push(100);
        input.push(100);

        let solution = 100;


        let guess = max_area(input);

        assert_eq!(guess, solution)
    }
}
