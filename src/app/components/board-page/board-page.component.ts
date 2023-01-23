import { Component, ElementRef, ViewChild } from '@angular/core';

@Component({
  selector: 'app-board-page',
  templateUrl: './board-page.component.html',
  styleUrls: ['./board-page.component.scss']
})
export class BoardPageComponent {

  public chessBoardCellSize = 60;

  public selectedX: number | undefined;
  public selectedY: number | undefined;

  @ViewChild('chessBoard')
  public chessBoard: ElementRef | undefined;

  public onBoardClick(e: MouseEvent) {
    let bounding = this.chessBoard?.nativeElement.getBoundingClientRect();

    let xBoardPos = e.clientX - bounding.x;
    let yBoardPos = e.clientY - bounding.y;

    let cellX = Math.floor(xBoardPos / this.chessBoardCellSize);
    let cellY = Math.floor(yBoardPos / this.chessBoardCellSize);

    this.selectedX = cellX;
    this.selectedY = cellY;
  }

  public boardPosToChessPos(x: number, y: number): String {
    let letter = String.fromCharCode(97 + x);
    let number = 8 - y;

    return `${letter} ${number}`
  }

}
