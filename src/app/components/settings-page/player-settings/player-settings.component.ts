import { Component, Input } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { PlayerConfig } from 'src/app/classes/PlayerConfig';
import { PlayerType } from 'src/app/enums/PlayerType';
import { AiSettingsComponent } from './ai-settings/ai-settings.component';

@Component({
  selector: 'app-player-settings',
  templateUrl: './player-settings.component.html',
  styleUrls: ['./player-settings.component.scss']
})
export class PlayerSettingsComponent {

  public playerType = PlayerType;

  @Input()
  public name: string = "";

  @Input()
  public playerConfig: PlayerConfig | null = null;

  constructor(private _matDialog: MatDialog) { }

  public onHumanButtonClicked() {
    this.playerConfig!.playerType = PlayerType.Human;
  }

  public onAIButtonClicked() {
    this.playerConfig!.playerType = PlayerType.Minimax;
  }

  public getConfigurationText(): string {
    let str = 'Configuration: ';

    if (this.playerConfig!.playerType === PlayerType.Human) {
      str += 'Human';
      return str;
    }

    switch (this.playerConfig!.playerType) {
      case PlayerType.Minimax:
        str += `Minimax, Alpha-Beta pruning=${this.playerConfig!.alphaBetaPruning}, Multi-threading=${this.playerConfig!.multiThreading}, Moves ahead=${this.playerConfig!.movesAhead}`;
        break;
    }

    return str;
  }

  public onSettingsButtonClicked() {
    const dialogRef = this._matDialog.open(AiSettingsComponent, {
      width: '500px',
      height: '500px',
      data: { playerConfig: this.playerConfig }
    });
  }

}
