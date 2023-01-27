import { ChessColor } from "../enums/ChessColor";
import { WinState } from "../enums/WinState";
import { ChessPiece } from "./ChessPiece";

export class BoardState {
    public board: (ChessPiece | null)[][];
    public turn: ChessColor;
    public winState: WinState;

    constructor(board: (ChessPiece | null)[][], turn: ChessColor, winState: WinState) {
        this.board = board;
        this.turn = turn;
        this.winState = winState;
    }
}
