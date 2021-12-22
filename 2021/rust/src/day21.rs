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

type Games = HashMap<(Player, Player), usize>;

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
    let mut games: Games = init_games(player1.position, player2.position);
    let mut new_games: Games = HashMap::new();
    let dirac_moves: &[(Position, usize)] =
        &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut rounds: usize = 0;
    let mut player1_wins: usize = 0;
    let mut player2_wins: usize = 0;

    loop {
        rounds += 1;

        if games.values().all(|count| *count == 0) {
            break;
        }

        for ((player1, player2), count) in games.iter() {
            for (player1_moves, player1_multiplier) in dirac_moves {
                // player 1's turn
                let (player1_position, player1_score) = play_turn(player1, *player1_moves);
                if player1_score >= 21 {
                    player1_wins += count * player1_multiplier;
                } else {
                    for (player2_moves, player2_multiplier) in dirac_moves {
                        // player 2's turn
                        let (player2_position, player2_score) = play_turn(player2, *player2_moves);

                        if player2_score >= 21 {
                            player2_wins += count * player2_multiplier;
                        } else {
                            new_games.insert(
                                (
                                    Player {
                                        position: player1_position,
                                        score: player1_score,
                                    },
                                    Player {
                                        position: player2_position,
                                        score: player2_score,
                                    },
                                ),
                                count * player1_multiplier * player2_multiplier,
                            );
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut games, &mut new_games);
        new_games.clear();
    }

    dbg!(rounds, player1_wins, player2_wins);

    player1_wins.max(player2_wins)
}

fn play_turn(player: &Player, moves: Position) -> (Position, Score) {
    (
        (player.position + moves) % 10,
        player.score + (player.position as Score) + 1,
    )
}

fn init_games(position1: Position, position2: Position) -> Games {
    let mut games: Games = HashMap::new();

    games.insert(
        (
            Player {
                position: position1,
                score: 0,
            },
            Player {
                position: position2,
                score: 0,
            },
        ),
        1,
    );

    games
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
