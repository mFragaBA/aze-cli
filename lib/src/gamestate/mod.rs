
use aze_types::actions::ActionType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Check_Action {
    pub action_type: ActionType,
    pub amount: Option<u64>, // Only used for Raise, others will be None
}

#[derive(Debug, Clone)]
struct Player {
    id: u64,
    balance: u64,
    current_bet: u64,
    has_folded: bool,
}

#[derive(Debug, Clone)]
pub struct PokerGame {
    players: Vec<Player>,
    small_blind: u64,
    big_blind: u64,
    pot: u64,
    current_bet: u64,
    current_player_index: usize,
}

impl PokerGame {
    pub fn new(
        player_ids: Vec<u64>,
        initial_balances: Vec<u64>,
        small_blind: u64,
        big_blind: u64,
    ) -> Self {
        let players = player_ids
            .into_iter()
            .zip(initial_balances.into_iter())
            .map(|(id, balance)| Player {
                id,
                balance,
                current_bet: 0,
                has_folded: false,
            })
            .collect();

        PokerGame {
            players,
            small_blind,
            big_blind,
            pot: 0,
            current_bet: 0,
            current_player_index: 0,
        }
    }

    pub fn check_move(&mut self, check_action: Check_Action, player_id: u64) -> bool {
        let player = &mut self.players[self.current_player_index];
        if player.id != player_id {
            eprintln!("Not your turn");
            return false;
        }
        if player.has_folded {
            eprintln!("Player has already folded");
            return false;
        }

        match check_action.action_type {
            ActionType::Fold => {
                player.has_folded = true;
            }
            ActionType::Check => {
                if player.current_bet < self.current_bet {
                    eprintln!("Cannot check, must call or raise");
                    return false;
                }
            }
            ActionType::Call => {
                let call_amount = self.current_bet - player.current_bet;
                if player.balance < call_amount {
                    eprintln!("Not enough balance to call");
                    return false;
                }
                player.balance -= call_amount;
                player.current_bet += call_amount;
                self.pot += call_amount;
            }
            ActionType::Raise => {
                if let Some(amount) = check_action.amount {
                    let total_bet = self.current_bet + amount;
                    if player.balance < total_bet {
                        eprintln!("Not enough balance to raise");
                        return false;
                    }
                    player.balance -= total_bet - player.current_bet;
                    player.current_bet = total_bet;
                    self.pot += total_bet - self.current_bet;
                    self.current_bet = total_bet;
                } else {
                    eprintln!("Raise amount not specified");
                    return false;
                }
            }
            ActionType::SmallBlind => {
                if self.current_player_index != 0 {
                    eprintln!("Only P1 can post the small blind");
                    return false;
                }
                let small_blind_amount = self.small_blind;
                if player.balance < small_blind_amount {
                    eprintln!("Not enough balance to post the small blind");
                    return false;
                }
                player.balance -= small_blind_amount;
                player.current_bet = small_blind_amount;
                self.pot += small_blind_amount;
                self.current_bet = small_blind_amount;
            }
            ActionType::BigBlind => {
                if self.current_player_index != 1 {
                    eprintln!("Only P2 can post the big blind");
                    return false;
                }
                let big_blind_amount = self.big_blind;
                if player.balance < big_blind_amount {
                    eprintln!("Not enough balance to post the big blind");
                    return false;
                }
                player.balance -= big_blind_amount;
                player.current_bet = big_blind_amount;
                self.pot += big_blind_amount;
                self.current_bet = big_blind_amount;
            }
        }

        self.current_player_index = (self.current_player_index + 1) % self.players.len();
        while self.players[self.current_player_index].has_folded {
            self.current_player_index = (self.current_player_index + 1) % self.players.len();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_blind_post() {
        let player_ids = vec![1, 2, 3, 4];
        let initial_balances = vec![1000, 1000, 1000, 1000];
        let mut game = PokerGame::new(player_ids, initial_balances, 10, 20);

        assert!(game.check_move(Check_Action { action_type: ActionType::SmallBlind, amount: None }, 1));
        assert_eq!(game.players[0].balance, 990);
        assert_eq!(game.pot, 10);
        assert_eq!(game.current_player_index, 1);
    }

    #[test]
    fn test_big_blind_post() {
        let player_ids = vec![1, 2, 3, 4];
        let initial_balances = vec![1000, 1000, 1000, 1000];
        let mut game = PokerGame::new(player_ids, initial_balances, 10, 20);

        assert!(game.check_move(Check_Action { action_type: ActionType::SmallBlind, amount: None }, 1));
        assert!(game.check_move(Check_Action { action_type: ActionType::BigBlind, amount: None }, 2));
        assert_eq!(game.players[1].balance, 980);
        assert_eq!(game.pot, 30);
        assert_eq!(game.current_player_index, 2);
    }

    #[test]
    fn test_call_action() {
        let player_ids = vec![1, 2, 3, 4];
        let initial_balances = vec![1000, 1000, 1000, 1000];
        let mut game = PokerGame::new(player_ids, initial_balances, 10, 20);

        game.check_move(Check_Action { action_type: ActionType::SmallBlind, amount: None }, 1);
        game.check_move(Check_Action { action_type: ActionType::BigBlind, amount: None }, 2);
        assert!(game.check_move(Check_Action { action_type: ActionType::Call, amount: None }, 3));
        assert_eq!(game.players[2].balance, 980);
        assert_eq!(game.pot, 50);
        assert_eq!(game.current_player_index, 3);
    }

    #[test]
    fn test_raise_action() {
        let player_ids = vec![1, 2, 3, 4];
        let initial_balances = vec![1000, 1000, 1000, 1000];
        let mut game = PokerGame::new(player_ids, initial_balances, 10, 20);

        game.check_move(Check_Action { action_type: ActionType::SmallBlind, amount: None }, 1);
        game.check_move(Check_Action { action_type: ActionType::BigBlind, amount: None }, 2);
        game.check_move(Check_Action { action_type: ActionType::Call, amount: None }, 3);
        assert!(game.check_move(Check_Action { action_type: ActionType::Raise, amount: Some(30) }, 4));
        assert_eq!(game.players[3].balance, 950);
        assert_eq!(game.pot, 80);
        assert_eq!(game.current_bet, 50);
        assert_eq!(game.current_player_index, 0);
    }

    #[test]
    fn test_fold_action() {
        let player_ids = vec![1, 2, 3, 4];
        let initial_balances = vec![1000, 1000, 1000, 1000];
        let mut game = PokerGame::new(player_ids, initial_balances, 10, 20);

        game.check_move(Check_Action { action_type: ActionType::SmallBlind, amount: None }, 1);
        game.check_move(Check_Action { action_type: ActionType::BigBlind, amount: None }, 2);
        assert!(game.check_move(Check_Action { action_type: ActionType::Fold, amount: None }, 3));
        assert!(game.players[2].has_folded);
        assert_eq!(game.current_player_index, 3);
    }

    #[test]
    fn test_invalid_actions() {
        let player_ids = vec![1, 2, 3, 4];
        let initial_balances = vec![1000, 1000, 1000, 1000];
        let mut game = PokerGame::new(player_ids, initial_balances, 10, 20);

        assert!(!game.check_move(Check_Action { action_type: ActionType::BigBlind, amount: None }, 1)); // Wrong player for big blind
        assert!(game.check_move(Check_Action { action_type: ActionType::SmallBlind, amount: None }, 1));
        assert!(!game.check_move(Check_Action { action_type: ActionType::SmallBlind, amount: None }, 2)); // Small blind already posted
        assert!(game.check_move(Check_Action { action_type: ActionType::BigBlind, amount: None }, 2));
        assert!(!game.check_move(Check_Action { action_type: ActionType::Check, amount: None }, 3)); // Cannot check, must call or raise
    }
}