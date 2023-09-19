#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) struct GameState {
    pub(crate) last_action: Action,
    pub(crate) last_played_card: Card,
    pub(crate) last_drew_card: Option<Card>,
    pub(crate) direction: Direction,
}

pub(crate) struct Deck {
    pub(crate) cards: Vec<Card>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum Action {
    Play { card: Card },
    ChooseColor { color: CardColor },
    DrawCard { amount: DrawAmount },
    CallBluff,
    Pass,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum Card {
    Normal(NormalCard),
    Special(WildCard),
}

impl Card {
    fn get_color(&self) -> Option<CardColor> {
        match self {
            Card::Normal(card) => Some(card.color),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) struct NormalCard {
    pub(crate) kind: CardKind,
    pub(crate) color: CardColor,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) struct WildCard {
    pub(crate) kind: WildCardKind,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum CardKind {
    Numeric(Numeric),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum Numeric {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum WildCardKind {
    DrawFour,
    Colorchooser,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum CardColor {
    Blue,
    Green,
    Yellow,
    Red,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum DrawAmount {
    One,
    Two,
    Four,
    Six,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum Direction {
    Forward,
    Backward,
}

impl GameState {
    fn get_actions_for_deck(&self, deck: &Deck) -> Vec<Action> {
        match self.last_action {
            Action::Play {
                card:
                    Card::Special(WildCard {
                        kind: WildCardKind::Colorchooser | WildCardKind::DrawFour,
                    }),
            } => vec![
                Action::ChooseColor {
                    color: CardColor::Blue,
                },
                Action::ChooseColor {
                    color: CardColor::Green,
                },
                Action::ChooseColor {
                    color: CardColor::Yellow,
                },
                Action::ChooseColor {
                    color: CardColor::Red,
                },
            ],
            Action::ChooseColor {
                color: chosen_color,
            } => match self.last_played_card {
                Card::Special(WildCard {
                    kind: WildCardKind::Colorchooser,
                }) => std::iter::once(Action::DrawCard {
                    amount: DrawAmount::One,
                })
                .chain(
                    deck.cards
                        .iter()
                        .filter(|card| {
                            card.get_color() == Some(chosen_color) || card.get_color().is_none()
                        })
                        .map(|&card| Action::Play { card }),
                )
                .collect(),
                Card::Special(WildCard {
                    kind: WildCardKind::DrawFour,
                }) => vec![
                    Action::DrawCard {
                        amount: DrawAmount::Four,
                    },
                    Action::CallBluff,
                ],
                _ => unreachable!("cannot have choose-color action after playing a normal card"),
            },
            Action::Play { .. } => {
                let mut actions = vec![Action::DrawCard {
                    amount: DrawAmount::One,
                }];

                for &card in &deck.cards {
                    if self.can_play_card(card) {
                        actions.push(Action::Play { card });
                    }
                }

                actions
            }
            Action::DrawCard {
                amount: DrawAmount::One,
            } => {
                let mut actions = vec![Action::Pass];
                let last_drew_card = self.last_drew_card.unwrap();

                if self.can_play_card(last_drew_card) {
                    actions.push(Action::Play {
                        card: last_drew_card,
                    });
                }

                actions
            }
            _ => todo!(),
        }
    }

    fn can_play_card(&self, card_to_play: Card) -> bool {
        match (self.last_played_card, card_to_play) {
            (Card::Normal(played), Card::Normal(to_play)) => {
                played.kind == to_play.kind || played.color == to_play.color
            }
            (Card::Normal(_), Card::Special(_)) => true,
            (Card::Special(_), Card::Normal(to_play)) => match self.last_action {
                Action::ChooseColor { color } => to_play.color == color,
                _ => false,
            },
            (Card::Special(played), Card::Special(_)) => match self.last_action {
                Action::ChooseColor { .. } => played.kind != WildCardKind::DrawFour,
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_play_card_of_same_color() {
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

        let deck = Deck {
            cards: vec![
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Six),
                    color: CardColor::Blue,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Nine),
                    color: CardColor::Blue,
                }),
            ],
        };

        assert_eq!(
            game_state.get_actions_for_deck(&deck),
            vec![
                Action::DrawCard {
                    amount: DrawAmount::One
                },
                Action::Play {
                    card: deck.cards[0]
                },
                Action::Play {
                    card: deck.cards[1]
                }
            ]
        );
    }

    #[test]
    fn test_can_play_card_of_same_kind() {
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

        let deck = Deck {
            cards: vec![
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Zero),
                    color: CardColor::Green,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Zero),
                    color: CardColor::Red,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Zero),
                    color: CardColor::Blue,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Zero),
                    color: CardColor::Yellow,
                }),
            ],
        };

        assert_eq!(
            game_state.get_actions_for_deck(&deck),
            vec![
                Action::DrawCard {
                    amount: DrawAmount::One
                },
                Action::Play {
                    card: deck.cards[0]
                },
                Action::Play {
                    card: deck.cards[1]
                },
                Action::Play {
                    card: deck.cards[2]
                },
                Action::Play {
                    card: deck.cards[3]
                }
            ]
        );
    }

    #[test]
    fn test_cannot_play_card_of_different_kind_and_color() {
        let last_played_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::One),
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

        let deck = Deck {
            cards: vec![
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Four),
                    color: CardColor::Green,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Two),
                    color: CardColor::Yellow,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Zero),
                    color: CardColor::Red,
                }),
            ],
        };

        assert_eq!(
            game_state.get_actions_for_deck(&deck),
            vec![Action::DrawCard {
                amount: DrawAmount::One
            }]
        );
    }

    #[test]
    fn test_can_play_special_card_on_any_color() {
        for &color in &[
            CardColor::Blue,
            CardColor::Green,
            CardColor::Yellow,
            CardColor::Red,
        ] {
            let last_played_card = Card::Normal(NormalCard {
                kind: CardKind::Numeric(Numeric::Zero),
                color,
            });

            let game_state = GameState {
                last_action: Action::Play {
                    card: last_played_card,
                },
                last_played_card,
                last_drew_card: None,
                direction: Direction::Forward,
            };

            let deck = Deck {
                cards: vec![
                    Card::Special(WildCard {
                        kind: WildCardKind::DrawFour,
                    }),
                    Card::Special(WildCard {
                        kind: WildCardKind::Colorchooser,
                    }),
                ],
            };

            assert_eq!(
                game_state.get_actions_for_deck(&deck),
                vec![
                    Action::DrawCard {
                        amount: DrawAmount::One
                    },
                    Action::Play {
                        card: deck.cards[0]
                    },
                    Action::Play {
                        card: deck.cards[1]
                    }
                ]
            );
        }
    }

    #[test]
    fn test_can_play_special_card_on_any_kind() {
        for &kind in &[
            Numeric::Zero,
            Numeric::One,
            Numeric::Two,
            Numeric::Three,
            Numeric::Four,
            Numeric::Five,
            Numeric::Six,
            Numeric::Seven,
            Numeric::Eight,
            Numeric::Nine,
        ] {
            let last_played_card = Card::Normal(NormalCard {
                kind: CardKind::Numeric(kind),
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

            let deck = Deck {
                cards: vec![
                    Card::Special(WildCard {
                        kind: WildCardKind::DrawFour,
                    }),
                    Card::Special(WildCard {
                        kind: WildCardKind::Colorchooser,
                    }),
                ],
            };

            assert_eq!(
                game_state.get_actions_for_deck(&deck),
                vec![
                    Action::DrawCard {
                        amount: DrawAmount::One
                    },
                    Action::Play {
                        card: deck.cards[0]
                    },
                    Action::Play {
                        card: deck.cards[1]
                    }
                ]
            );
        }
    }

    #[test]
    fn test_should_only_choose_color_after_playing_colorchooser() {
        let last_played_card = Card::Special(WildCard {
            kind: WildCardKind::Colorchooser,
        });

        let game_state = GameState {
            last_action: Action::Play {
                card: last_played_card,
            },
            last_played_card,
            last_drew_card: None,
            direction: Direction::Forward,
        };

        let deck = Deck {
            cards: vec![
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Zero),
                    color: CardColor::Blue,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::One),
                    color: CardColor::Green,
                }),
                Card::Special(WildCard {
                    kind: WildCardKind::DrawFour,
                }),
            ],
        };

        assert_eq!(
            game_state.get_actions_for_deck(&deck),
            vec![
                Action::ChooseColor {
                    color: CardColor::Blue
                },
                Action::ChooseColor {
                    color: CardColor::Green
                },
                Action::ChooseColor {
                    color: CardColor::Yellow
                },
                Action::ChooseColor {
                    color: CardColor::Red
                },
            ]
        );
    }

    #[test]
    fn test_should_only_choose_color_after_playing_draw_four() {
        let last_played_card = Card::Special(WildCard {
            kind: WildCardKind::DrawFour,
        });

        let game_state = GameState {
            last_action: Action::Play {
                card: last_played_card,
            },
            last_played_card,
            last_drew_card: None,
            direction: Direction::Forward,
        };

        let deck = Deck {
            cards: vec![
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Zero),
                    color: CardColor::Blue,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::One),
                    color: CardColor::Green,
                }),
                Card::Special(WildCard {
                    kind: WildCardKind::DrawFour,
                }),
            ],
        };

        assert_eq!(
            game_state.get_actions_for_deck(&deck),
            vec![
                Action::ChooseColor {
                    color: CardColor::Blue
                },
                Action::ChooseColor {
                    color: CardColor::Green
                },
                Action::ChooseColor {
                    color: CardColor::Yellow
                },
                Action::ChooseColor {
                    color: CardColor::Red
                },
            ]
        );
    }

    #[test]
    fn test_can_play_card_of_same_color_as_chosen_by_colorchooser() {
        let last_played_card = Card::Special(WildCard {
            kind: WildCardKind::Colorchooser,
        });

        let game_state = GameState {
            last_action: Action::ChooseColor {
                color: CardColor::Red,
            },
            last_played_card,
            last_drew_card: None,
            direction: Direction::Forward,
        };

        let blue_zero_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Zero),
            color: CardColor::Blue,
        });

        let red_zero_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Zero),
            color: CardColor::Red,
        });

        let deck = Deck {
            cards: vec![blue_zero_card, red_zero_card],
        };

        assert_eq!(
            game_state.get_actions_for_deck(&deck),
            vec![
                Action::DrawCard {
                    amount: DrawAmount::One
                },
                Action::Play {
                    card: red_zero_card
                },
            ]
        );
    }

    #[test]
    fn test_can_play_any_special_card_on_top_of_colorchooser() {
        let last_played_card = Card::Special(WildCard {
            kind: WildCardKind::Colorchooser,
        });

        let game_state = GameState {
            last_action: Action::ChooseColor {
                color: CardColor::Red,
            },
            last_played_card,
            last_drew_card: None,
            direction: Direction::Forward,
        };

        let colorchooser_card = Card::Special(WildCard {
            kind: WildCardKind::Colorchooser,
        });

        let draw_four_card = Card::Special(WildCard {
            kind: WildCardKind::DrawFour,
        });

        let deck = Deck {
            cards: vec![colorchooser_card, draw_four_card],
        };

        assert_eq!(
            game_state.get_actions_for_deck(&deck),
            vec![
                Action::DrawCard {
                    amount: DrawAmount::One
                },
                Action::Play {
                    card: colorchooser_card
                },
                Action::Play {
                    card: draw_four_card
                },
            ]
        );
    }

    #[test]
    fn test_can_only_draw_four_or_call_bluff_after_draw_four() {
        let last_played_card = Card::Special(WildCard {
            kind: WildCardKind::DrawFour,
        });

        let game_state = GameState {
            last_action: Action::ChooseColor {
                color: CardColor::Red,
            },
            last_played_card,
            last_drew_card: None,
            direction: Direction::Forward,
        };

        let blue_zero_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Zero),
            color: CardColor::Blue,
        });

        let red_zero_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Zero),
            color: CardColor::Red,
        });

        let colorchooser_card = Card::Special(WildCard {
            kind: WildCardKind::Colorchooser,
        });

        let draw_four_card = Card::Special(WildCard {
            kind: WildCardKind::DrawFour,
        });

        let deck = Deck {
            cards: vec![
                blue_zero_card,
                red_zero_card,
                colorchooser_card,
                draw_four_card,
            ],
        };

        assert_eq!(
            game_state.get_actions_for_deck(&deck),
            vec![
                Action::DrawCard {
                    amount: DrawAmount::Four
                },
                Action::CallBluff,
            ]
        );
    }

    #[test]
    fn test_after_drawing_one_card_can_only_pass_or_play_drew_card() {
        let last_played_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Zero),
            color: CardColor::Blue,
        });

        let last_drew_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Four),
            color: CardColor::Blue,
        });

        let game_state = GameState {
            last_action: Action::DrawCard {
                amount: DrawAmount::One,
            },
            last_played_card,
            last_drew_card: Some(last_drew_card),
            direction: Direction::Forward,
        };

        let deck = Deck {
            cards: vec![
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Two),
                    color: CardColor::Red,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Zero),
                    color: CardColor::Blue,
                }),
            ],
        };

        assert_eq!(
            game_state.get_actions_for_deck(&deck),
            vec![
                Action::Pass,
                Action::Play {
                    card: last_drew_card,
                },
            ]
        );
    }

    #[test]
    fn test_after_drawing_one_card_and_it_is_unplayable_can_pass_only() {
        let last_played_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Zero),
            color: CardColor::Blue,
        });

        let last_drew_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Four),
            color: CardColor::Green,
        });

        let game_state = GameState {
            last_action: Action::DrawCard {
                amount: DrawAmount::One,
            },
            last_played_card,
            last_drew_card: Some(last_drew_card),
            direction: Direction::Forward,
        };

        let deck = Deck {
            cards: vec![
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Two),
                    color: CardColor::Red,
                }),
                Card::Normal(NormalCard {
                    kind: CardKind::Numeric(Numeric::Zero),
                    color: CardColor::Blue,
                }),
            ],
        };

        assert_eq!(game_state.get_actions_for_deck(&deck), vec![Action::Pass]);
    }
}
