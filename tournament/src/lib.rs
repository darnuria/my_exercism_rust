use std::collections::HashMap;

fn str_to_state(result: &str) -> State {
    match result {
        "win" => State::Win,
        "draw" => State::Draw,
        "loss" => State::Loss,
        _ => unreachable!(),
    }
}

enum State {
    Win,
    Draw,
    Loss,
}

enum Outcome {
    Win(usize, usize),
    Draw,
}

fn score(a: usize, b: usize, result: State) -> Outcome {
    match result {
        State::Win => Outcome::Win(a, b),
        State::Draw => Outcome::Draw,
        State::Loss => Outcome::Win(b, a),
    }
}

struct Team<'a> {
    name: &'a str,
    points: u8,
    wins: u8,
    loss: u8,
    draw: u8,
}

impl<'a> Team<'a> {
    fn new(name: &'a str) -> Team {
        Team {
            name,
            points: 0,
            wins: 0,
            loss: 0,
            draw: 0,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let header = String::from("Team                           | MP |  W |  D |  L |  P");
    if match_results.len() < 1 {
        return header;
    }
    let mut teams: HashMap<&str, usize> = HashMap::with_capacity(match_results.len() - 1);
    let mut teams_score = Vec::with_capacity(match_results.len() - 1);
    for m in match_results.lines() {
        let mut matches = m.split(';');
        let a = matches.next().unwrap();
        let b = matches.next().unwrap();
        let result = str_to_state(matches.next().unwrap());

        let a = *teams.entry(a).or_insert_with(|| {
            teams_score.push(Team::new(a));
            teams_score.len() - 1
        });
        let b = *teams.entry(b).or_insert_with(|| {
            teams_score.push(Team::new(b));
            teams_score.len() - 1
        });
        let outcome = score(a, b, result);

        match outcome {
            Outcome::Win(w, l) => {
                let mut w = &mut teams_score[w];
                w.wins += 1;
                w.points += 3;
                let mut l = &mut teams_score[l];
                l.loss += 1;
            }
            Outcome::Draw => {
                let mut w = &mut teams_score[a];
                w.draw += 1;
                w.points += 1;
                let mut l = &mut teams_score[b];
                l.draw += 1;
                l.points += 1;
            }
        }
    }

    teams_score.sort_unstable_by(|a, b| {
        let c = b.points.cmp(&a.points);
        match c {
            std::cmp::Ordering::Less | std::cmp::Ordering::Greater => c,
            std::cmp::Ordering::Equal => a.name.cmp(b.name),
        }
    });
    teams_score.into_iter().fold(header, |acc, t| {
        format!(
            "{}\n{:<31}|  {} |  {} |  {} |  {} |  {}",
            acc,
            t.name,
            t.wins + t.draw + t.loss,
            t.wins,
            t.draw,
            t.loss,
            t.points
        )
    })
}
