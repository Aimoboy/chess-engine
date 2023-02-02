import { Component, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { ChessColor } from 'src/app/enums/ChessColor';
import { ChessType } from 'src/app/enums/ChessType';
import { getChessPiecePictureFromTypeAndColor } from 'src/app/utils/generalUtils';

type dialogDataType = {
  chessBoardCellSize: number,
  selectedCellColor: string,
  coloredCellColor: string,
  whiteCellColor: string,
  turn: ChessColor
}

@Component({
  selector: 'app-promotion-selector',
  templateUrl: './promotion-selector.component.html',
  styleUrls: ['./promotion-selector.component.scss']
})
export class PromotionSelectorComponent {
  public promotionChessTypes = [ChessType.Rook, ChessType.Knight, ChessType.Bishop, ChessType.Queen];
  private selected = -1;

  constructor(@Inject(MAT_DIALOG_DATA) public dialogData: dialogDataType, private _dialogRef: MatDialogRef<PromotionSelectorComponent>) { }

  public chessPieceToImg(type: ChessType, color: ChessColor): string {
    return getChessPiecePictureFromTypeAndColor(type, color);
  }

  public getCellColor(index: number): string {
    if (this.selected === index) {
      return this.dialogData.selectedCellColor;
    }

    if (index % 2 === 0) {
      return this.dialogData.coloredCellColor;
    }

    return this.dialogData.whiteCellColor;
  }

  public onPieceClick(index: number) {
    this.selected = index;
  }

  public isButtonDisabled(): boolean {
    return this.selected === -1;
  }

  public onButtonClicked() {
    this._dialogRef.close(this.selected);
  }
}
