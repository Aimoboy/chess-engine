import { Component, Inject } from '@angular/core';
import { MatCheckboxChange } from '@angular/material/checkbox';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { PlayerConfig } from 'src/app/classes/PlayerConfig';

type dialogDataType = {
  playerConfig: PlayerConfig
}

@Component({
  selector: 'app-ai-settings',
  templateUrl: './ai-settings.component.html',
  styleUrls: ['./ai-settings.component.scss']
})
export class AiSettingsComponent {

  constructor(@Inject(MAT_DIALOG_DATA) public dialogData: dialogDataType, private _dialogRef: MatDialogRef<AiSettingsComponent>) { }

  public onAlphaBetaPruningChange(event: MatCheckboxChange) {
    this.dialogData.playerConfig.alphaBetaPruning = event.checked;
  }

  public onMultiThreadingChange(event: MatCheckboxChange) {
    this.dialogData.playerConfig.multiThreading = event.checked;
  }

  public onCloseButtonClicked() {
    this._dialogRef.close();
  }

  public onMovesAheadChange(event: any) {
    this.dialogData.playerConfig.movesAhead = Math.max(event.target.value, 1);
  }

}
