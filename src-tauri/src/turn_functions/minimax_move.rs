use crate::board_types::bitboard::Constants;
use crate::traits::chess_board_contract::ChessBoardContract;
use crate::{Player, EvaluationFunction};

use crate::enums::{
    chess_color::ChessColor,
    chess_error::ChessError
};

pub fn minimax_move<T: ChessBoardContract + Clone + Send + Sync>(board: &T,
                                                                 board_history: &Vec<T>,
                                                                 turn: ChessColor,
                                                                 player: &Player<T>,
                                                                 eval_func: EvaluationFunction<T>,
                                                                 constants: &Constants,
                                                                 alpha_beta_pruning: bool,
                                                                 multi_threading: bool) -> Result<String, ChessError> {

    println!("Looking {} moves ahead...", player.moves_ahead);
    let start_time = std::time::Instant::now();

    let possible_moves = board.generate_moves(turn, constants)?;

    let evaluated_moves: Vec<(i32, String)> = match multi_threading {
        false => {

            possible_moves.iter().map(|(mov_str, mov_board)| -> Result<(i32, String), ChessError> {

                let mut new_board_history = board_history.clone();
                new_board_history.push(mov_board.clone());
        
                let eval = minimax_move_helper(
                    mov_board,
                    turn.opposite_color(),
                    eval_func,
                    &new_board_history,
                    constants,
                    player.moves_ahead - 1,
                    i32::MIN,
                    i32::MAX,
                    alpha_beta_pruning
                );

                match eval {
                    Ok(res) => {
                        Ok((res, mov_str.clone()))
                    },
                    Err(err) => Err(err)
                }
        
            }).collect()

        },
        true => {

            let thread_num = num_cpus::get();
            let queue: work_queue::Queue<(String, T)> = work_queue::Queue::new(thread_num, 128);

            for mov in possible_moves {
                queue.push(mov.clone());
            }

            std::thread::scope(|s| -> Result<Vec<(i32, String)>, ChessError> {

                let handles = queue.local_queues().map(|mut local_queue| {
                    let new_depth = player.moves_ahead - 1;
                    s.spawn(move || {
                        let mut results: Vec<Result<(i32, String), ChessError>> = Vec::new();

                        while let Some((mov_str, mov_board)) = local_queue.pop() {
                            let mut new_history = board_history.clone();
                            new_history.push(mov_board.clone());

                            let eval = minimax_move_helper(
                                &mov_board,
                                turn.opposite_color(),
                                eval_func,
                                &new_history,
                                constants,
                                new_depth,
                                i32::MIN,
                                i32::MAX,
                                alpha_beta_pruning
                            );

                            match eval {
                                Err(err) => results.push(Err(err)),
                                Ok(val) => results.push(Ok((val, mov_str)))
                            }
                        }

                        results
                    })
                }).collect::<Vec<_>>();

                handles.into_iter()
                    .map(|h| h.join().unwrap())
                    .flatten()
                    .fold(Ok(Vec::new()), |acc, item| {
                    let mut acc_val: Vec<(i32, String)> = acc?;
                    let item_val = item?;
                    acc_val.push(item_val);

                    Ok(acc_val)
                })

            })
        }
    }?;

    let maximizing_player = turn == ChessColor::White;
    let best_move = match maximizing_player {
        true => evaluated_moves.iter().max_by_key(|(value, _)| value),
        false => evaluated_moves.iter().min_by_key(|(value, _)| value),
    };

    match best_move {
        Some((_, mov_str)) => {
            println!("Finished in {} seconds, making the following move: {}", start_time.elapsed().as_millis() as f32 / 1000., mov_str);
            Ok(mov_str.clone())
        },
        None => Err(ChessError::NoMovesFound)
    }
}

fn minimax_move_helper<T: ChessBoardContract + Clone>(board: &T,
                                                      turn: ChessColor,
                                                      eval_func: EvaluationFunction<T>,
                                                      board_history: &Vec<T>,
                                                      constants: &Constants,
                                                      depth: i32,
                                                      alpha: i32,
                                                      beta: i32,
                                                      alpha_beta_pruning: bool) -> Result<i32, ChessError> {
    
    let maximizing_player = turn == ChessColor::White;
    let possible_moves = board.generate_moves(turn, constants)?;
    
    if depth == 0 || possible_moves.len() == 0 {
        return Ok(eval_func(board, board_history, depth, constants)?);
    }

    let mut ret_value = match maximizing_player {
        true => i32::MIN,
        false => i32::MAX
    };

    let mut new_alpha = alpha;
    let mut new_beta = beta;

    for mov in possible_moves {
        let (_, mov_board) = mov;
        let mut new_board_history = board_history.clone();
        new_board_history.push(board.clone());

        let eval = minimax_move_helper(
            &mov_board,
            turn.opposite_color(),
            eval_func,
            &new_board_history,
            constants,
            depth - 1,
            new_alpha,
            new_beta,
            alpha_beta_pruning
        )?;

        if maximizing_player {
            if eval > ret_value {
                ret_value = eval;
            }
            if eval > new_alpha {
                new_alpha = eval;
            }
        } else {
            if eval < ret_value {
                ret_value = eval;
            }
            if eval < new_beta {
                new_beta = eval;
            }
        }

        if alpha_beta_pruning && new_beta <= new_alpha {
            break;
        }
    }
    
    Ok(ret_value)
}
