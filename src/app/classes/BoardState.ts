import { ChessColor } from "../enums/ChessColor";
import { WinState } from "../enums/WinState";
import { ChessPiece } from "./ChessPiece";
import { PossibleMove } from "./Position";

export class BoardState {
    public board: (ChessPiece | null)[][];
    public possibleMoves: PossibleMove[][][];
    public turn: ChessColor;
    public winState: WinState;

    constructor(board: (ChessPiece | null)[][], possibleMoves: PossibleMove[][][], turn: ChessColor, winState: WinState) {
        this.board = board;
        this.possibleMoves = possibleMoves;
        this.turn = turn;
        this.winState = winState;
    }
}
