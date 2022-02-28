pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let poss = [
        (-1isize, -1isize),
        (-1, 0),
        (-1, 1),
        (0, -1),
        /*(0,0) ,*/ 
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let size_y = minefield.len();
    let size_x = if let Some(n) = minefield.get(0) {
        n.len()
    } else {
        return vec![];
    };
    let mut new = Vec::with_capacity(size_y);

    for y in 0..size_y {
        let mut s = String::with_capacity(size_x);
        for x in 0..size_x {
            let mut sum = 0i32;
            if minefield[y].as_bytes()[x] == b'*' {
                sum = -1;
            } else {
                for (pos_x, pos_y) in poss {
                    if let Some(m) = minefield.get((pos_y + y as isize) as usize) {
                        if let Some(m) = m.as_bytes().get((pos_x + x as isize) as usize) {
                            if *m == b'*' {
                                sum += 1;
                            }
                        }
                    }
                }
            }
            s.push(match sum {
                -1 => '*',
                0 => ' ',
                n => char::from_digit(n as u32, 10).unwrap(),
            });
        }
        new.push(s);
    }
    new
}
