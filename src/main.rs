fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Copy)]
struct GameState {
    last_action: Action,
    last_played_card: Card,
    last_drew_card: Option<Card>,
}

struct Deck {
    cards: Vec<Card>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Action {
    Play { card: Card },
    ChooseColor { color: CardColor },
    DrawCard { amount: DrawAmount },
    CallBluff,
    Pass,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Card {
    Normal(NormalCard),
    Special(SpecialCard),
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
struct NormalCard {
    kind: CardKind,
    color: CardColor,
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct SpecialCard {
    kind: SpecialCardKind,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum CardKind {
    Numeric(Numeric),
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Numeric {
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
enum SpecialCardKind {
    DrawFour,
    Colorchooser,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum CardColor {
    Blue,
    Green,
    Yellow,
    Red,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum DrawAmount {
    One,
    Two,
    Four,
    Six,
}

fn get_actions_for_deck(game_state: &GameState, deck: &Deck) -> Vec<Action> {
    match game_state.last_action {
        Action::Play {
            card:
                Card::Special(SpecialCard {
                    kind: SpecialCardKind::Colorchooser | SpecialCardKind::DrawFour,
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
        } => match game_state.last_played_card {
            Card::Special(SpecialCard {
                kind: SpecialCardKind::Colorchooser,
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
            Card::Special(SpecialCard {
                kind: SpecialCardKind::DrawFour,
            }) => vec![
                Action::DrawCard {
                    amount: DrawAmount::Four,
                },
                Action::CallBluff,
            ],
            _ => unreachable!("cannot have choose-color action after playing a normal card"),
        },
        Action::Play { card: played_card } => match (played_card, deck.cards[0]) {
            (Card::Normal(played), Card::Normal(to_play)) => {
                if played.kind == to_play.kind || played.color == to_play.color {
                    vec![
                        Action::DrawCard {
                            amount: DrawAmount::One,
                        },
                        Action::Play {
                            card: deck.cards[0],
                        },
                    ]
                } else {
                    vec![Action::DrawCard {
                        amount: DrawAmount::One,
                    }]
                }
            }
            (Card::Normal(_), Card::Special(_)) => vec![
                Action::DrawCard {
                    amount: DrawAmount::One,
                },
                Action::Play {
                    card: deck.cards[0],
                },
            ],
            _ => todo!("{:?}, {:?}", game_state.last_action, deck.cards),
        },
        Action::DrawCard {
            amount: DrawAmount::One,
        } => {
            vec![
                Action::Pass,
                // TODO: Refactor can-play-card checker so you can decide if the drew card can be
                // played.
                Action::Play {
                    card: game_state.last_drew_card.unwrap(),
                },
            ]
        }
        _ => todo!(),
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
        };

        let deck = Deck {
            cards: vec![Card::Normal(NormalCard {
                kind: CardKind::Numeric(Numeric::Seven),
                color: CardColor::Blue,
            })],
        };

        assert_eq!(
            get_actions_for_deck(&game_state, &deck),
            vec![
                Action::DrawCard {
                    amount: DrawAmount::One
                },
                Action::Play {
                    card: deck.cards[0]
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
        };

        let deck = Deck {
            cards: vec![Card::Normal(NormalCard {
                kind: CardKind::Numeric(Numeric::Zero),
                color: CardColor::Green,
            })],
        };

        assert_eq!(
            get_actions_for_deck(&game_state, &deck),
            vec![
                Action::DrawCard {
                    amount: DrawAmount::One
                },
                Action::Play {
                    card: deck.cards[0]
                }
            ]
        );
    }

    #[test]
    fn test_cannot_play_card_of_different_kind_and_color() {
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
        };

        let deck = Deck {
            cards: vec![Card::Normal(NormalCard {
                kind: CardKind::Numeric(Numeric::One),
                color: CardColor::Green,
            })],
        };

        assert_eq!(
            get_actions_for_deck(&game_state, &deck),
            vec![Action::DrawCard {
                amount: DrawAmount::One
            },]
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
            };

            let deck = Deck {
                cards: vec![Card::Special(SpecialCard {
                    kind: SpecialCardKind::DrawFour,
                })],
            };

            assert_eq!(
                get_actions_for_deck(&game_state, &deck),
                vec![
                    Action::DrawCard {
                        amount: DrawAmount::One
                    },
                    Action::Play {
                        card: deck.cards[0]
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
            };

            let deck = Deck {
                cards: vec![Card::Special(SpecialCard {
                    kind: SpecialCardKind::DrawFour,
                })],
            };

            assert_eq!(
                get_actions_for_deck(&game_state, &deck),
                vec![
                    Action::DrawCard {
                        amount: DrawAmount::One
                    },
                    Action::Play {
                        card: deck.cards[0]
                    }
                ]
            );
        }
    }

    #[test]
    fn test_should_only_choose_color_after_playing_colorchooser() {
        let last_played_card = Card::Special(SpecialCard {
            kind: SpecialCardKind::Colorchooser,
        });

        let game_state = GameState {
            last_action: Action::Play {
                card: last_played_card,
            },
            last_played_card,
            last_drew_card: None,
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
                Card::Special(SpecialCard {
                    kind: SpecialCardKind::DrawFour,
                }),
            ],
        };

        assert_eq!(
            get_actions_for_deck(&game_state, &deck),
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
        let last_played_card = Card::Special(SpecialCard {
            kind: SpecialCardKind::DrawFour,
        });

        let game_state = GameState {
            last_action: Action::Play {
                card: last_played_card,
            },
            last_played_card,
            last_drew_card: None,
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
                Card::Special(SpecialCard {
                    kind: SpecialCardKind::DrawFour,
                }),
            ],
        };

        assert_eq!(
            get_actions_for_deck(&game_state, &deck),
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
        let last_played_card = Card::Special(SpecialCard {
            kind: SpecialCardKind::Colorchooser,
        });

        let game_state = GameState {
            last_action: Action::ChooseColor {
                color: CardColor::Red,
            },
            last_played_card,
            last_drew_card: None,
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
            get_actions_for_deck(&game_state, &deck),
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
        let last_played_card = Card::Special(SpecialCard {
            kind: SpecialCardKind::Colorchooser,
        });

        let game_state = GameState {
            last_action: Action::ChooseColor {
                color: CardColor::Red,
            },
            last_played_card,
            last_drew_card: None,
        };

        let colorchooser_card = Card::Special(SpecialCard {
            kind: SpecialCardKind::Colorchooser,
        });

        let draw_four_card = Card::Special(SpecialCard {
            kind: SpecialCardKind::DrawFour,
        });

        let deck = Deck {
            cards: vec![colorchooser_card, draw_four_card],
        };

        assert_eq!(
            get_actions_for_deck(&game_state, &deck),
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
        let last_played_card = Card::Special(SpecialCard {
            kind: SpecialCardKind::DrawFour,
        });

        let game_state = GameState {
            last_action: Action::ChooseColor {
                color: CardColor::Red,
            },
            last_played_card,
            last_drew_card: None,
        };

        let blue_zero_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Zero),
            color: CardColor::Blue,
        });

        let red_zero_card = Card::Normal(NormalCard {
            kind: CardKind::Numeric(Numeric::Zero),
            color: CardColor::Red,
        });

        let colorchooser_card = Card::Special(SpecialCard {
            kind: SpecialCardKind::Colorchooser,
        });

        let draw_four_card = Card::Special(SpecialCard {
            kind: SpecialCardKind::DrawFour,
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
            get_actions_for_deck(&game_state, &deck),
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
            get_actions_for_deck(&game_state, &deck),
            vec![
                Action::Pass,
                Action::Play {
                    card: last_drew_card,
                },
            ]
        );
    }

    #[test]
    fn test_after_drawing_one_card_can_only_pass_if_card_is_unplayable() {
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

        assert_eq!(get_actions_for_deck(&game_state, &deck), vec![Action::Pass]);
    }
}
