import { Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import { PossibleMove } from 'src/app/classes/PossibleMove';
import { ChessColor } from 'src/app/enums/ChessColor';
import { ChessType } from 'src/app/enums/ChessType';
import { invoke } from '@tauri-apps/api';
import { BoardState } from 'src/app/classes/BoardState';
import { boardStateResponse, parseServiceBoardStateResponse } from 'src/app/utils/parseUtils';
import { getChessPiecePictureFromTypeAndColor, isNullOrUndefined } from 'src/app/utils/generalUtils';
import { MatDialog } from '@angular/material/dialog';
import { PromotionSelectorComponent } from './promotion-selector/promotion-selector.component';
import { lastValueFrom } from 'rxjs';

@Component({
  selector: 'app-board-page',
  templateUrl: './board-page.component.html',
  styleUrls: ['./board-page.component.scss']
})
export class BoardPageComponent implements OnInit {
  private selectedCellColor = '#33bbff';
  private possibleMoveColor = '#ff3333';
  private whiteCellColor = '#eeeeD2';
  private coloredCellColor = '#769656';
  public chessBoardCellSize = 60;

  public boardState: BoardState | null = null;
  public log: string[] = [];

  public selectedX: number = -1;
  public selectedY: number = -1;

  @ViewChild('chessBoard')
  public chessBoard: ElementRef | undefined;

  constructor(private _matDialog: MatDialog) {}

  ngOnInit(): void {
    invoke<boardStateResponse>('get_start_board_state').then(state => {
      this.boardState = parseServiceBoardStateResponse(state);
    }, (err) => console.log(err));
  }

  public async onBoardClick(e: MouseEvent) {
    if (isNullOrUndefined(this.boardState)) {
      return;
    }

    // Find clicked cell
    let bounding = this.chessBoard?.nativeElement.getBoundingClientRect();

    let xBoardPos = e.clientX - bounding.x;
    let yBoardPos = e.clientY - bounding.y;

    let cellX = Math.floor(xBoardPos / this.chessBoardCellSize);
    let cellY = 7 - Math.floor(yBoardPos / this.chessBoardCellSize);

    // Make move
    if (this.hasCellSelected()) {
      const selectedCellPossibleMoves = this.getPossibleMovesAtCoords(this.selectedX, this.selectedY, cellX, cellY);

      if (selectedCellPossibleMoves.length > 0) {
        let newFen: string;
        if (selectedCellPossibleMoves.length > 1) {
          const promotionFens = this.separatePromotionFens(selectedCellPossibleMoves);

          const dialogRef = this._matDialog.open(PromotionSelectorComponent, {
            width: '300px',
            height: '150px',
            disableClose: true,
            data: {
              chessBoardCellSize: this.chessBoardCellSize,
              selectedCellColor: this.selectedCellColor,
              coloredCellColor: this.coloredCellColor,
              whiteCellColor: this.whiteCellColor,
              turn: this.boardState!.turn
            }
          });

          let dialogResult = await lastValueFrom(dialogRef.afterClosed());
          newFen = promotionFens[dialogResult.data];
        } else {
          newFen = selectedCellPossibleMoves[0].fen;
        }
        console.log(newFen);

        this.log.push(`${this.boardPosToChessPos(this.selectedX, this.selectedY)} to ${this.boardPosToChessPos(cellX, cellY)}`);
        this.deselect();
        this.boardState!.turn = ChessColor.None;

        invoke<boardStateResponse>('fen_to_board_state', {'fen': newFen, 'history': this.boardState!.history}).then(state => {
          this.boardState = parseServiceBoardStateResponse(state);
          console.log(this.boardState);
        }, err => console.log(err));

        return;
      }
    }

    const clickedCell = this.boardState!.board[cellX][cellY];
    const clickedCellIsEmpty = clickedCell === null;
    const clickedCellHasOwnedPiece = clickedCell?.color === this.boardState!.turn;
    const clickedCellHasOpponentPiece = clickedCell?.color !== this.boardState!.turn;

    // Deselect
    if (clickedCellIsEmpty || clickedCellHasOpponentPiece) {
      this.deselect();
      return;
    }

    // Select
    if (clickedCellHasOwnedPiece) {
      this.select(cellX, cellY);
    }
  }

  public chessPieceToImg(type: ChessType, color: ChessColor): string {
    return getChessPiecePictureFromTypeAndColor(type, color);
  }

  public getCellColor(x: number, y: number): string {
    if (this.selectedX === x && this.selectedY === y) {
      return this.selectedCellColor;
    }

    if (this.selectedCellHasPossibleMoveForCoords(x, y)) {
      return this.possibleMoveColor;
    }

    const cellShouldBeColored = (x + y) % 2 === 1;
    if (cellShouldBeColored) {
      return this.coloredCellColor;
    }

    return this.whiteCellColor;
  }

  public getCellCursor(x: number, y: number): string {
    if (isNullOrUndefined(this.boardState)) {
      return 'default';
    }

    if (this.boardState!.board[x][y]?.color === this.boardState!.turn) {
      return 'pointer';
    }

    if (this.selectedCellHasPossibleMoveForCoords(x, y)) {
      return 'pointer';
    }

    return 'default';
  }

  private boardPosToChessPos(x: number, y: number): string {
    const xString = String.fromCharCode(x + 97);
    const yString = (y + 1).toString(10);

    return xString + yString;
  }

  private hasCellSelected() {
    return this.selectedX !== -1 && this.selectedY !== -1;
  }

  private select(x: number, y: number) {
    this.selectedX = x;
    this.selectedY = y;
  }

  private deselect() {
    this.selectedX = -1;
    this.selectedY = -1;
  }

  private getPossibleMovesAtCoords(xFrom: number, yFrom: number, xTo: number, yTo: number): PossibleMove[] {
    if (isNullOrUndefined(this.boardState)) {
      return [];
    }

    if (xFrom < 0 || xFrom > 8) {
      throw new Error('Invalid from x coordinate, expected a number in the range \'0-7\' but got \'' + xFrom + '\'');
    }

    if (yFrom < 0 || yFrom > 8) {
      throw new Error('Invalid from y coordinate, expected a number in the range \'0-7\' but got \'' + yFrom + '\'');
    }

    if (xTo < 0 || xTo > 8) {
      throw new Error('Invalid to x coordinate, expected a number in the range \'0-7\' but got \'' + xTo + '\'');
    }

    if (yTo < 0 || yTo > 8) {
      throw new Error('Invalid to y coordinate, expected a number in the range \'0-7\' but got \'' + yTo + '\'');
    }

    return this.boardState!.possibleMoves[xFrom][yFrom]?.filter(item => item.x === xTo).filter(item => item.y === yTo);
  }

  private selectedCellHasPossibleMoveForCoords(x: number, y: number): boolean {
    if (!this.hasCellSelected()) {
      return false;
    }

    return this.getPossibleMovesAtCoords(this.selectedX, this.selectedY, x, y).length > 0;
  }

  private cellHasAnyPossibleMoves(x: number, y: number): boolean {
    if (isNullOrUndefined(this.boardState)) {
      return false;
    }

    return this.boardState!.possibleMoves[x][y].length > 0;
  }

  private separatePromotionFens(moves: PossibleMove[]): [string, string, string, string] {
    if (moves.length !== 4) {
      throw new Error('Invalid amount of moves for promotion, expected 4 but got \'' + moves.length + '\'');
    }

    return [this.findPromotionFenForPieceType(moves, ChessType.Rook),
            this.findPromotionFenForPieceType(moves, ChessType.Knight),
            this.findPromotionFenForPieceType(moves, ChessType.Bishop),
            this.findPromotionFenForPieceType(moves, ChessType.Queen)];
  }

  private findPromotionFenForPieceType(moves: PossibleMove[], pieceType: ChessType): string {
    let pieceChar: string;
    switch (pieceType) {
      case ChessType.Rook:
        pieceChar = 'r';
        break;
      case ChessType.Knight:
        pieceChar = 'n';
        break;
      case ChessType.Bishop:
        pieceChar = 'b';
        break;
      case ChessType.Queen:
        pieceChar = 'q';
        break;
      default:
        const validPromotionTypes = [ChessType.Rook, ChessType.Knight, ChessType.Bishop, ChessType.Queen];
        if (!validPromotionTypes.includes(pieceType)) {
          throw new Error('Invalid promotion type, expected one of \'' + validPromotionTypes + '\' but got \'' + pieceType + '\'');
        }
    }

    const promotionFen = moves.map(mov => {
      const count = mov.fen.split(' ')[0].toLowerCase().split(pieceChar).length - 1
      return {fen: mov.fen, count: count};
    }).sort((item1, item2) => {
      return item2.count - item1.count;
    })[0].fen;

    return promotionFen;
  }
}
