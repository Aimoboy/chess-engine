import { Component, ElementRef, ViewChild } from '@angular/core';
import { ChessPiece } from 'src/app/classes/ChessPiece';
import { Position } from 'src/app/classes/Position';
import { ChessColor } from 'src/app/enums/ChessColor';
import { ChessType } from 'src/app/enums/ChessType';
import { invoke } from '@tauri-apps/api';

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

  private currentFen: string;
  public board: (ChessPiece | null)[][];
  public possibleMoves: Position[][][] = [];
  public turn: ChessColor = ChessColor.None;

  public log: string[] = [];

  public chessBoardCellSize = 60;

  public selectedX: number = -1;
  public selectedY: number = -1;

  @ViewChild('chessBoard')
  public chessBoard: ElementRef | undefined;

  constructor() {
    this.currentFen = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';
    this.board = this.fenToBoard(this.currentFen);
    this.turn = this.fenToTurn(this.currentFen);

    invoke<string[]>('fen_to_possible_moves', {'fen': this.currentFen}).then((movs) => {
      console.log(movs);

      this.possibleMoves = this.parseMoveListToPossibleMoves(movs);
    }, (err) => console.log(err));
  }

  private parseMoveListToPossibleMoves(movs: string[]): Position[][][] {
    let empty = this.emptyPossibleMoves();

    for (const mov of movs) {
      if (mov.length !== 5) {
        throw new Error('Move wrong length, expected 5 but was ' + mov.length);
      }

      const from = this.chessPosToBoardPos(mov.slice(0, 2));
      const to = this.chessPosToBoardPos(mov.slice(3, 5));

      empty[from[1]][from[0]].push({x: to[0], y: to[1]});
    }

    return empty;
  }

  private emptyPossibleMoves(): Position[][][] {
    let possibleMovesBoard = [];

    for (let i = 0; i < 8; i++) {
      let tmp = [];
      for (let j = 0; j < 8; j++) {
        if (i === 6 && j === 0) {
          tmp.push([new Position(0, 5)])
        } else {
          tmp.push([]);
        }
      }
      possibleMovesBoard.push(tmp);
    }

    return possibleMovesBoard;
  }

  public onBoardClick(e: MouseEvent) {
    // Find clicked cell
    let bounding = this.chessBoard?.nativeElement.getBoundingClientRect();

    let xBoardPos = e.clientX - bounding.x;
    let yBoardPos = e.clientY - bounding.y;

    let cellX = Math.floor(xBoardPos / this.chessBoardCellSize);
    let cellY = Math.floor(yBoardPos / this.chessBoardCellSize);

    // Make move
    if (this.hasCellSelected() && this.selectedCellHasPossibleMoveForCoords(cellX, cellY)) {
      this.log.push(`${this.boardPosToChessPos(this.selectedX, this.selectedY)} to ${this.boardPosToChessPos(cellX, cellY)}`);
      return;
    }

    // Deselect
    if (this.board[cellY][cellX] === null || this.board[cellY][cellX]?.color !== this.turn) {
      this.selectedX = -1;
      this.selectedY = -1;
      return;
    }

    // Select if clicked cell has an owned piece
    if (this.board[cellY][cellX]?.color === this.turn && this.possibleMoves[cellY][cellX].length > 0) {
      this.selectedX = cellX;
      this.selectedY = cellY;
    }
  }

  private boardPosToChessPos(x: number, y: number): string {
    let letter = String.fromCharCode(97 + x);
    let number = 8 - y;

    return `${letter}${number}`
  }

  private chessPosToBoardPos(str: string): [number, number] {
    if (str.length !== 2) {
      throw new Error('Wrong chess pos length')
    }

    if (str[0] < 'a' || str[0] > 'h') {
      throw new Error('Invalid chess pos letter');
    }

    if (str[1] < '1' || str[1] > '8') {
      throw new Error('Invalid chess pos number');
    }

    let letterNum;

    switch (str[0]) {
      case 'a':
        letterNum = 0;
        break;
      case 'b':
        letterNum = 1;
        break;
      case 'c':
        letterNum = 2;
        break;
      case 'd':
        letterNum = 3;
        break;
      case 'e':
        letterNum = 4;
        break;
      case 'f':
        letterNum = 5;
        break;
      case 'g':
        letterNum = 6;
        break;
      case 'h':
        letterNum = 7;
        break;
      default:
        letterNum = -1;
    }

    return [letterNum, 8 - parseInt(str[1])];
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

  private fenToBoard(fenString: string): (ChessPiece | null)[][] {
    let fenBoard = fenString.split(' ')[0];
    let board = [];
    let tmp: (ChessPiece | null)[] = [];

    for (let char of fenBoard) {
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
            throw new Error('Invalid char in FEN string');
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

  private fenToTurn(fenString: string): ChessColor {
    let fenTurn = fenString.split(' ')[1];

    switch (fenTurn) {
      case 'w':
        return ChessColor.White;
      case 'b':
        return ChessColor.Black;
      default:
        throw new Error('Wrong turn character for FEN string');
    }
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
    if (this.board[y][x]?.color === this.turn && this.cellHasAnyPossibleMoves(x, y)) {
      return 'pointer';
    }

    if (this.hasCellSelected() && this.selectedCellHasPossibleMoveForCoords(x, y)) {
      return 'pointer';
    }

    return 'default';
  }

  private hasCellSelected() {
    return this.selectedX !== -1 && this.selectedY !== -1;
  }

  private selectedCellHasPossibleMoveForCoords(x: number, y: number): boolean {
    return this.possibleMoves[this.selectedY][this.selectedX]?.filter(item => item.x === x).filter(item => item.y === y).length > 0;
  }

  private cellHasAnyPossibleMoves(x: number, y: number): boolean {
    return this.possibleMoves[y][x]?.length > 0;
  }

}
