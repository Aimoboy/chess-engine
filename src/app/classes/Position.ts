
export class PossibleMove {
    public x: number;
    public y: number;
    public fen: string;

    constructor(x: number, y: number, fen: string) {
        this.x = x;
        this.y = y;
        this.fen = fen;
    }
}
