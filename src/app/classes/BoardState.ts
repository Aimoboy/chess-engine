import { ChessColor } from "../enums/ChessColor";
import { WinState } from "../enums/WinState";
import { ChessPiece } from "./ChessPiece";
import { PossibleMove } from "./PossibleMove";

export class BoardState {
    public board: (ChessPiece | null)[][];
    public possibleMoves: PossibleMove[][][];
    public turn: ChessColor;
    public winState: WinState;
    public history: string[];

    constructor(board: (ChessPiece | null)[][], possibleMoves: PossibleMove[][][], turn: ChessColor, winState: WinState, history: string[]) {
        this.board = board;
        this.possibleMoves = possibleMoves;
        this.turn = turn;
        this.winState = winState;
        this.history = history;
    }
}
