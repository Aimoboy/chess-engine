import { Component, ElementRef, ViewChild } from '@angular/core';
import { ChessPiece } from 'src/app/classes/ChessPiece';
import { Position } from 'src/app/classes/Position';
import { ChessColor } from 'src/app/enums/ChessColor';
import { ChessType } from 'src/app/enums/ChessType';

@Component({
  selector: 'app-board-page',
  templateUrl: './board-page.component.html',
  styleUrls: ['./board-page.component.scss']
})
export class BoardPageComponent {
  private selectedCellColor = 'lightblue';
  private possibleMoveColor = 'red';
  private whiteCellColor = '#EEEED2';
  private coloredCellColor = '#769656';

  public board = this.fenToBoard('rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR');
  public possibleMoves: Position[][][] = [];
  public turn: ChessColor = ChessColor.White;

  public log: string[] = [];

  public chessBoardCellSize = 60;

  public selectedX: number = -1;
  public selectedY: number = -1;

  @ViewChild('chessBoard')
  public chessBoard: ElementRef | undefined;

  constructor() {
    for (let i = 0; i < 8; i++) {
      let tmp = [];
      for (let j = 0; j < 8; j++) {
        if (i === 6 && j === 0) {
          tmp.push([new Position(0, 5)])
        } else {
          tmp.push([]);
        }
      }
      this.possibleMoves.push(tmp);
    }
  }

  public onBoardClick(e: MouseEvent) {
    // Find clicked cell
    let bounding = this.chessBoard?.nativeElement.getBoundingClientRect();

    let xBoardPos = e.clientX - bounding.x;
    let yBoardPos = e.clientY - bounding.y;

    let cellX = Math.floor(xBoardPos / this.chessBoardCellSize);
    let cellY = Math.floor(yBoardPos / this.chessBoardCellSize);

    // Make move
    if (this.hasCellSelected() &&
        this.possibleMoves[this.selectedY][this.selectedX].filter(item => item.x === cellX).filter(item => item.y === cellY).length > 0) {
      this.log.push(`${this.boardPosToChessPos(this.selectedX, this.selectedY)} to ${this.boardPosToChessPos(cellX, cellY)}`)
    }

    // Deselect
    if (this.board[cellY][cellX] === null || this.board[cellY][cellX]?.color !== this.turn) {
      this.selectedX = -1;
      this.selectedY = -1;
      return;
    }

    // Select if clicked cell has an owned piece
    if (this.board[cellY][cellX]?.color === this.turn) {
      this.selectedX = cellX;
      this.selectedY = cellY;
    }
  }

  public boardPosToChessPos(x: number, y: number): string {
    let letter = String.fromCharCode(97 + x);
    let number = 8 - y;

    return `${letter}${number}`
  }

  public chessTypeToImg(type: ChessType, color: ChessColor): string {
    let piece = '';

    switch (color) {
      case ChessColor.White:
        piece += 'w';
        break;
      case ChessColor.Black:
        piece += 'b';
        break;
    }

    switch (type) {
      case ChessType.Pawn:
        piece += 'p';
        break;
      case ChessType.Rook:
        piece += 'r';
        break;
      case ChessType.Knight:
        piece += 'n';
        break;
      case ChessType.Bishop:
        piece += 'b';
        break;
      case ChessType.Queen:
        piece += 'q';
        break;
      case ChessType.King:
        piece += 'k';
        break;
    }

    return `https://images.chesscomfiles.com/chess-themes/pieces/neo/150/${piece}.png`
  }

  public fenToBoard(fenString: string) {
    let board = [];
    let tmp: (ChessPiece | null)[] = [];

    for (let char of fenString) {
      switch (char) {
        case 'P':
          tmp.push(new ChessPiece(ChessType.Pawn, ChessColor.White));
          break;
        case 'R':
          tmp.push(new ChessPiece(ChessType.Rook, ChessColor.White));
          break;
        case 'N':
          tmp.push(new ChessPiece(ChessType.Knight, ChessColor.White));
          break;
        case 'B':
          tmp.push(new ChessPiece(ChessType.Bishop, ChessColor.White));
          break;
        case 'Q':
          tmp.push(new ChessPiece(ChessType.Queen, ChessColor.White));
          break;
        case 'K':
          tmp.push(new ChessPiece(ChessType.King, ChessColor.White));
          break;
        case 'p':
          tmp.push(new ChessPiece(ChessType.Pawn, ChessColor.Black));
          break;
        case 'r':
          tmp.push(new ChessPiece(ChessType.Rook, ChessColor.Black));
          break;
        case 'n':
          tmp.push(new ChessPiece(ChessType.Knight, ChessColor.Black));
          break;
        case 'b':
          tmp.push(new ChessPiece(ChessType.Bishop, ChessColor.Black));
          break;
        case 'q':
          tmp.push(new ChessPiece(ChessType.Queen, ChessColor.Black));
          break;
        case 'k':
          tmp.push(new ChessPiece(ChessType.King, ChessColor.Black));
          break;
        case '/':
          board.push(tmp);
          tmp = [];
          break;
        default:
          if (char < '1' || char > '8') {
            throw new Error('Invalid char in fen string');
          }

          const num = parseInt(char);

          for (let i = 0; i < num; i++) {
            tmp.push(null);
          }
          break;
      }
    }

    board.push(tmp);

    return board;
  }

  public getCellColor(x: number, y: number): string {
    if (this.selectedX === x && this.selectedY === y) {
      return this.selectedCellColor;
    }

    if (this.hasCellSelected() &&
        this.possibleMoves[this.selectedY][this.selectedX].filter(item => item.x === x).filter(item => item.y === y).length > 0) {
      return this.possibleMoveColor;
    }

    if ((x + y) % 2 === 1) {
      return this.coloredCellColor;
    }

    return this.whiteCellColor;
  }

  public getCellCursor(x: number, y: number): string {
    if (this.board[y][x]?.color === this.turn) {
      return 'pointer';
    }

    if (this.hasCellSelected() &&
        this.possibleMoves[this.selectedY][this.selectedX].filter(item => item.x === x).filter(item => item.y === y).length > 0) {
      return 'pointer';
    }

    return 'cursor';
  }

  private hasCellSelected() {
    return this.selectedX !== -1 && this.selectedY !== -1;
  }

}
