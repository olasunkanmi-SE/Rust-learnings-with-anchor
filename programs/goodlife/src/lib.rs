use anchor_lang::prelude::*;
use num_derive::{FromPrimitive, ToPrimitive};

declare_id!("EHA4RUUTvAuKqSHuv2UrVnuAxrJdzDDf1CZtRKkhS9HK");

pub fn setup_game(ctx: Context<SetUpGame>, player_two: Pubkey) -> Result<()> {
    ctx.accounts
        .game
        .start([ctx.accounts.player_one.key(), player_two])
}

#[derive(
    AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Clone, Copy, PartialEq, Eq,
)]
pub enum Sign {
    X,
    O,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    row: u8,
    column: u8,
}

#[error_code]
pub enum TicTacToeError {
    TileOutOfBounds,
    TileAlreadySet,
    GameAlreadyOver,
    NotPlayersTurn,
    GameAlreadyStarted,
}

#[account]
pub struct Game {
    players: [Pubkey; 2],
    turn: u8,
    board: [[Option<Sign>; 3]; 3],
    state: GameState,
}

#[derive(Accounts)]
pub struct SetUpGame<'info> {
    #[account(init, space=8 + Game::MAXIMUM_SIZE, payer = player_one)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl Game {
    pub const MAXIMUM_SIZE: usize = (32 * 2) + 1 + (9 * (1 + 1)) + (32 + 1);
    pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
        require_eq!(self.turn, 0, TicTacToeError::GameAlreadyStarted);
        self.players = players;
        self.turn = 1;
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.state == GameState::Active
    }

    fn curren_player_index(&self) -> usize {
        ((self.turn - 1) % 2) as usize
    }

    pub fn current_player(&self) -> Pubkey {
        self.players[self.curren_player_index()]
    }

    fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;
        self.board[first.0][first.1].is_some()
            && self.board[first.0][first.1] == self.board[second.0][second.1]
            && self.board[first.0][first.1] == self.board[third.0][third.1]
    }

    fn update_state(&mut self) {
        for i in 0..=2 {
            let horizontal = [(i, 0), (i, 1), (i, 2)];
            if self.is_winning_trio(horizontal) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }
            let vertical = [(0, i), (1, i), (2, i)];
            if self.is_winning_trio(vertical) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }

            let diagonal_left = [(0, 0), (1, 1), (2, 2)];
            let diagonal_right = [(0, 2), (1, 1), (2, 0)];
            if self.is_winning_trio(diagonal_left) || self.is_winning_trio(diagonal_right) {
                self.state = GameState::Won {
                    winner: self.current_player(),
                };
                return;
            }
        }

        for row in 0..=2 {
            for column in 0..2 {
                if self.board[row][column].is_none() {
                    return;
                }
            }
        }

        self.state = GameState::Tie
    }

    //Explain this, focus on the match
    pub fn play(&mut self, tile: &Tile) -> Result<()> {
        require!(self.is_active(), TicTacToeError::GameAlreadyOver);
        match tile {
            tile @ Tile {
                row: 0..=2,
                column: 0..=2,
            } => match self.board[tile.row as usize][tile.column as usize] {
                Some(_) => return Err(TicTacToeError::TileAlreadySet.into()),
                None => {
                    let sign = if self.curren_player_index() == 0 {
                        Sign::X
                    } else {
                        Sign::O
                    };
                    self.board[tile.row as usize][tile.column as usize] = Some(sign);
                }
            },
            _ => return Err(TicTacToeError::TileOutOfBounds.into()),
        }
        self.update_state();
        if GameState::Active == self.state {
            self.turn += 1
        }
        Ok(())
    }
}
