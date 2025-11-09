use std::collections::HashMap;
use std::fmt::Write;

#[derive(Default)]
struct Stats {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

pub fn tally(match_results: &str) -> String {
    let mut table: HashMap<String, Stats> = HashMap::new();

    for line in match_results.lines().filter(|l| !l.trim().is_empty()) {
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 3 {
            continue; 
        }

        let (team1, team2, result) = (parts[0], parts[1], parts[2]);

        table.entry(team1.to_string()).or_default();
        table.entry(team2.to_string()).or_default();

        match result {
            "win" => {
                update(&mut table, team1, 3, 1, 0, 0);
                update(&mut table, team2, 0, 0, 0, 1);
            }
            "loss" => {
                update(&mut table, team1, 0, 0, 0, 1);
                update(&mut table, team2, 3, 1, 0, 0);
            }
            "draw" => {
                update(&mut table, team1, 1, 0, 1, 0);
                update(&mut table, team2, 1, 0, 1, 0);
            }
            _ => continue, 
        }
    }

    let mut teams: Vec<_> = table.into_iter().collect();
    teams.sort_by(|a, b| {
        b.1.p.cmp(&a.1.p).then_with(|| a.0.cmp(&b.0))
    });

    let mut output = String::new();
    writeln!(
        output,
        "{:<31}| MP |  W |  D |  L |  P",
        "Team"
    ).unwrap();

    for (team, s) in teams {
        writeln!(
            output,
            "{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team, s.mp, s.w, s.d, s.l, s.p
        ).unwrap();
    }

    output.trim_end().to_string()
}

fn update(table: &mut HashMap<String, Stats>, team: &str, pts: u32, w: u32, d: u32, l: u32) {
    let entry = table.get_mut(team).unwrap();
    entry.mp += 1;
    entry.w += w;
    entry.d += d;
    entry.l += l;
    entry.p += pts;
}
