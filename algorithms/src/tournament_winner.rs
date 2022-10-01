/*
  difficulty: easy

  There's an algorithms tournament taking place in which teams of programmers
  compete against each other to solve algorithmic problems as fast as possible.
  Teams compete in a round robin, where each team faces off against all other
  teams. Only two teams compete against each other at a time, and for each
  competition, one team is designated the home team, while the other team is the
  away team. In each competition there's always one winner and one loser; there
  are no ties. A team receives 3 points if it wins and 0 points if it loses. The
  winner of the tournament is the team that receives the most amount of points.

  Given an array of pairs representing the teams that have competed against each
  other and an array containing the results of each competition, write a function
  that returns the winner of the tournament. The input arrays are named
  competitions and results, respectively. The competitions array has elements in
  the form of [homeTeam, awayTeam], where each team is a string of at most 30
  characters representing the name of the team. The results array contains
  information about the winner of each corresponding competition in the
  competitions array. Specifically, results[i] denotes the winner of
  competitions[i], where a 1 in the results array means that the home team in the
  corresponding competition won and a 0 means that the away team won.

  It's guaranteed that exactly one team will win the tournament and that each
  team will compete against all other teams exactly once. It's also guaranteed
  that the tournament will always have at least two teams.
*/

use std::collections::HashMap;
use std::io::{Error, ErrorKind};

pub fn tournament_winner(
    competitions: Vec<Vec<&str>>,
    results: Vec<usize>,
) -> Result<String, Error> {
    if competitions.len() <= 0 || results.len() <= 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "There should be at least two teams in the competition",
        ));
    }

    let mut points: HashMap<String, usize> = HashMap::new();

    for (i, w) in results.iter().enumerate() {
        let entry = competitions[i][*w];
        points
            .entry(entry.to_string())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    Ok(points
        .keys()
        .max()
        .replace(&"something".to_string())
        .unwrap_or(&"No winner".to_string())
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tournament_winner_no_teams() {
        assert_eq!(tournament_winner(vec![], vec![]).is_err(), true);
    }

    #[test]
    fn tournament_winner_works() {
        let competitions = vec![
            vec!["HTML", "C#"],
            vec!["C#", "Python"],
            vec!["Python", "HTML"],
        ];

        let results = vec![0, 0, 1];

        assert_eq!(tournament_winner(competitions, results).unwrap(), "HTML");
    }
}
