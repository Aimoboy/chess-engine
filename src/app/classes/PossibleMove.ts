import { WinState } from "../enums/WinState";

export class PossibleMove {
    public x: number;
    public y: number;
    public fen: string;
    public winState: WinState;

    constructor(x: number, y: number, fen: string, winState: WinState) {
        this.x = x;
        this.y = y;
        this.fen = fen;
        this.winState = winState;
    }
}
