import { ChessColor } from "../enums/ChessColor";
import { ChessType } from "../enums/ChessType";

export class ChessPiece {
    public type: ChessType;
    public color: ChessColor;

    constructor(typ: ChessType, col: ChessColor) {
        this.type = typ;
        this.color = col;
    }
}
