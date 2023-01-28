import { Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import { PossibleMove } from 'src/app/classes/PossibleMove';
import { ChessColor } from 'src/app/enums/ChessColor';
import { ChessType } from 'src/app/enums/ChessType';
import { invoke } from '@tauri-apps/api';
import { BoardState } from 'src/app/classes/BoardState';
import { boardStateResponse, parseServiceBoardStateResponse } from 'src/app/utils/parseUtils';
import { isNullOrUndefined } from 'src/app/utils/generalUtils';

@Component({
  selector: 'app-board-page',
  templateUrl: './board-page.component.html',
  styleUrls: ['./board-page.component.scss']
})
export class BoardPageComponent implements OnInit {
  private selectedCellColor = 'lightblue';
  private possibleMoveColor = 'red';
  private whiteCellColor = '#EEEED2';
  private coloredCellColor = '#769656';
  public chessBoardCellSize = 60;

  public boardState: BoardState | null = null;
  public log: string[] = [];

  public selectedX: number = -1;
  public selectedY: number = -1;

  @ViewChild('chessBoard')
  public chessBoard: ElementRef | undefined;

  constructor() {}

  ngOnInit(): void {
    invoke<boardStateResponse>('get_start_board_state').then(state => {
      this.boardState = parseServiceBoardStateResponse(state);
    }, (err) => console.log(err));
  }

  public onBoardClick(e: MouseEvent) {
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
        this.log.push(`${this.boardPosToChessPos(this.selectedX, this.selectedY)} to ${this.boardPosToChessPos(cellX, cellY)}`);
        const newFEN = selectedCellPossibleMoves[0].fen;
        this.log.push(newFEN);

        this.deselect();
        this.boardState!.turn = ChessColor.None;

        invoke<boardStateResponse>('fen_to_board_state', {'fen': newFEN}).then(state => {
          this.boardState = parseServiceBoardStateResponse(state);
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

  public chessTypeToImg(type: ChessType, color: ChessColor): string {
    if (isNullOrUndefined(type) || isNullOrUndefined(color)) {
      return 'none';
    }

    let pieceString = '';

    switch (color) {
      case ChessColor.White:
        pieceString += 'w';
        break;
      case ChessColor.Black:
        pieceString += 'b';
        break;
    }

    switch (type) {
      case ChessType.Pawn:
        pieceString += 'p';
        break;
      case ChessType.Rook:
        pieceString += 'r';
        break;
      case ChessType.Knight:
        pieceString += 'n';
        break;
      case ChessType.Bishop:
        pieceString += 'b';
        break;
      case ChessType.Queen:
        pieceString += 'q';
        break;
      case ChessType.King:
        pieceString += 'k';
        break;
    }

    return `url(https://images.chesscomfiles.com/chess-themes/pieces/neo/150/${pieceString}.png)`
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
}
