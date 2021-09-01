use std::cmp::Ordering;
use std::collections::btree_set::BTreeSet;
use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut s = String::new();
    s.push_str(&format("Team", "MP", "W", "D", "L", "P"));
    if match_results.is_empty() {
        return s;
    }
    let mut map: HashMap<String, Team> = HashMap::new();
    let mut change = |name: &str, res: GameResult| {
        if let Some(team) = map.get_mut(name) {
            match res {
                GameResult::Draw => team.tied(),
                GameResult::Win => team.won(),
                GameResult::Loss => team.lost(),
            }
        } else {
            let mut team = Team::new(name);
            match res {
                GameResult::Draw => team.tied(),
                GameResult::Win => team.won(),
                GameResult::Loss => team.lost(),
            }
            map.insert(name.into(), team);
        }
    };
    match_results.lines().for_each(|play| {
        let mut it = play.split(';');
        let t1 = it.next().unwrap();
        let t2 = it.next().unwrap();
        match it.next() {
            Some("win") => {
                change(t1, GameResult::Win);
                change(t2, GameResult::Loss);
            }
            Some("draw") => {
                change(t1, GameResult::Draw);
                change(t2, GameResult::Draw);
            }
            Some("loss") => {
                change(t1, GameResult::Loss);
                change(t2, GameResult::Win);
            }
            _ => (),
        }
    });
    let set: BTreeSet<Team> = map.values().cloned().collect();
    set.iter().rev().for_each(|t| {
        let name = t.name.clone();
        s.push_str("\n");
        s.push_str(&format(
            name, t.matches, t.wins, t.draws, t.losses, t.points,
        ));
    });
    s
}

fn format<T, V>(team: T, matches: V, wins: V, draws: V, loses: V, points: V) -> String
where
    T: std::fmt::Display,
    V: std::fmt::Display,
{
    format!(
        "{:30} |{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
        team, matches, wins, draws, loses, points
    )
}

#[derive(PartialEq, Eq)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
struct Team {
    name: String,
    matches: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    points: i32,
}

impl Team {
    pub fn new(name: &str) -> Self {
        let mut this = Team::default();
        this.name = String::from(name);
        this
    }

    pub fn won(&mut self) {
        self.matches += 1;
        self.wins += 1;
        self.points += 3;
    }

    pub fn tied(&mut self) {
        self.matches += 1;
        self.draws += 1;
        self.points += 1;
    }

    pub fn lost(&mut self) {
        self.matches += 1;
        self.losses += 1;
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.points.cmp(&other.points) {
            Ordering::Equal => other.name.cmp(&self.name),
            order => order,
        }
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
