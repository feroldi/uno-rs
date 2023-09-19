use crate::game_state::{Action, Card, GameState};

struct GameRuntime {
    current_player_idx: usize,
    players: Vec<Player>,
    drawing_deck: Vec<Card>,
    game_state: GameState,
}

struct Player {
    deck: Vec<Card>,
}

impl GameRuntime {
    fn execute_action(&mut self, action: Action) {
        self.game_state.last_action = action;

        match action {
            Action::Play { card } => {
                self.game_state.last_played_card = card;
                let card_idx = self.players[self.current_player_idx]
                    .deck
                    .iter()
                    .position(|&c| c == card)
                    .unwrap();
                self.players[self.current_player_idx].deck.remove(card_idx);
            }
            _ => todo!(),
        }

        self.current_player_idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::game_state::*;

    // TODO: Don't remove duplicated card from player's deck.
    #[test]
    fn play_normal_card() {
        let last_played_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Zero),
            color: CardColor::Blue,
        });

        let game_state = GameState {
            last_action: Action::Play {
                card: last_played_card,
            },
            last_played_card,
            last_drew_card: None,
            direction: Direction::Forward,
        };

        let player1 = Player {
            deck: vec![
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Zero),
                    color: CardColor::Green,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Three),
                    color: CardColor::Yellow,
                }),
            ],
        };

        let player2 = Player {
            deck: vec![
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::One),
                    color: CardColor::Blue,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Two),
                    color: CardColor::Red,
                }),
            ],
        };

        let mut game_runtime = GameRuntime {
            current_player_idx: 0usize,
            players: vec![player1, player2],
            drawing_deck: vec![],
            game_state,
        };

        let next_card = game_runtime.players[0].deck[0];
        let next_action = Action::Play { card: next_card };

        game_runtime.execute_action(next_action);

        assert_eq!(game_runtime.current_player_idx, 1usize);
        assert_eq!(
            game_runtime.game_state,
            GameState {
                last_action: Action::Play { card: next_card },
                last_played_card: next_card,
                last_drew_card: None,
                direction: Direction::Forward,
            }
        );

        assert_eq!(
            game_runtime.players[0].deck,
            &[Card::Normal(NormalCard {
                kind: CardKind::Numeric(Numeric::Three),
                color: CardColor::Yellow,
            })]
        );

        assert_eq!(
            game_runtime.players[1].deck,
            &[
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::One),
                    color: CardColor::Blue,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Two),
                    color: CardColor::Red,
                })
            ]
        );
    }
}
