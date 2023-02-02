import { PlayerType } from "../enums/PlayerType";

export class PlayerConfig {
    playerType: PlayerType;
    movesAhead: number;
    alphaBetaPruning: boolean;
    multiThreading: boolean;

    constructor(playerType: PlayerType, movesAhead: number, alphaBetaPruning: boolean, multiThreading: boolean) {
        this.playerType = playerType;
        this.movesAhead = movesAhead;
        this.alphaBetaPruning = alphaBetaPruning;
        this.multiThreading = multiThreading;
    }

    public static defaultPlayerConfig(): PlayerConfig {
        return new PlayerConfig(PlayerType.Human, 4, true, true);
    }
}
