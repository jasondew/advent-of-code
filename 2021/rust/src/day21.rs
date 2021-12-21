use std::collections::HashMap;
use std::fmt::Debug;

type Position = u8;
type Score = u16;

#[derive(Eq, Hash, PartialEq)]
struct Player {
    position: Position,
    score: Score,
}

impl Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p{:}s{:}", &self.position, &self.score)
    }
}

type DiracPlayer = HashMap<Player, usize>;

#[must_use]
pub fn part1(input: &str) -> usize {
    let (mut player1, mut player2) = parse(input);
    let mut rolls: usize = 0;

    loop {
        play(&mut player1, &mut rolls);
        if won(&player1) {
            break;
        }

        play(&mut player2, &mut rolls);
        if won(&player2) {
            break;
        }
    }

    (player1.score.min(player2.score) as usize) * rolls
}

fn play(player: &mut Player, rolls: &mut usize) {
    let moves = ((3 * *rolls + 6) % 10) as Position;

    player.position = (player.position + moves) % 10;
    player.score += (player.position as Score) + 1;

    *rolls += 3;
}

fn won(player: &Player) -> bool {
    player.score >= 1000
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let (player1, player2) = parse(input);
    let mut player1s: DiracPlayer = init_players(player1.position);
    let mut player2s: DiracPlayer = init_players(player2.position);

    let mut rounds: usize = 0;
    let mut player1_wins: usize = 0;
    let mut player2_wins: usize = 0;

    loop {
        rounds += 1;
        if player1s.values().all(|count| *count == 0) || player2s.values().all(|count| *count == 0)
        {
            break;
        }

        play_dirac(&mut player1s);
        for (player, count) in player1s.iter_mut() {
            if player.score >= 21 {
                player1_wins += *count;
                *count = 0;
            }
        }

        play_dirac(&mut player2s);
        for (player, count) in player2s.iter_mut() {
            if player.score >= 21 {
                player2_wins += *count;
                *count = 0;
            }
        }
    }

    dbg!(rounds, player1_wins, player2_wins);

    player1_wins.max(player2_wins)
}

fn play_dirac(players: &mut DiracPlayer) {
    let dirac_moves: &[(Position, usize)] =
        &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let mut new_players: Vec<(Player, usize)> = Vec::new();

    for (player, player_count) in players.iter_mut() {
        if *player_count > 0 {
            for (moves, count) in dirac_moves {
                let new_position = (player.position + moves) % 10;
                let new_score = player.score + (player.position as Score) + 1;

                new_players.push((
                    Player {
                        position: new_position,
                        score: new_score,
                    },
                    (*count * *player_count),
                ));
            }
            *player_count = 0;
        }
    }

    for (player, count) in new_players {
        *players.entry(player).or_insert(0) += count;
    }
}

fn init_players(initial_position: Position) -> DiracPlayer {
    let mut dirac_player: DiracPlayer = HashMap::new();

    for position in 0..10 {
        dirac_player.insert(
            Player {
                position: position,
                score: 0,
            },
            if position == initial_position { 1 } else { 0 },
        );
    }

    dirac_player
}

fn parse(input: &str) -> (Player, Player) {
    let (player1, player2) = input.trim_end().split_once("\n").unwrap();

    (parse_player(player1), parse_player(player2))
}

fn parse_player(player_string: &str) -> Player {
    let (_verbiage, position_string) = player_string.split_once(": ").unwrap();

    Player {
        position: position_string.parse::<Position>().unwrap() - 1,
        score: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "\
Player 1 starting position: 4
Player 2 starting position: 8\n"
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(input()), 739785)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(input()), 444356092776315)
    }
}
