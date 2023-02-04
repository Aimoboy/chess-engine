import { ChessColor } from "../enums/ChessColor";
import { ChessType } from "../enums/ChessType";

export function isNullOrUndefined(item: any): boolean {
    if (item === null || item === undefined) {
        return true;
    }

    return false;
}

export function getChessPiecePictureFromTypeAndColor(type: ChessType, color: ChessColor): string {
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

    return `url(../../../assets/pieces/${pieceString}.png)`;
  }
