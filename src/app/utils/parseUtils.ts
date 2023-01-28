import { BoardState } from "../classes/BoardState";
import { ChessPiece } from "../classes/ChessPiece";
import { PossibleMove } from "../classes/PossibleMove";
import { ChessColor } from "../enums/ChessColor";
import { ChessType } from "../enums/ChessType";
import { WinState } from "../enums/WinState";

export function parseServiceBoardStateResponse(boardState: {fen: string, moves: [string, string, string][], turn: string, win_state: string}): BoardState {
    const board = parseFenToBoard(boardState.fen);
    const possibleMoves = parseBoardStateMovesToPossibleMoves(boardState.moves);
    const turn = parseTurnState(boardState.turn);
    const winState = parseWinState(boardState.win_state);

    return new BoardState(board, possibleMoves, turn, winState);
}

function parseFenToBoard(fen: string): (ChessPiece | null)[][] {
    const fenBoardPart = fen.split(' ')[0];
    let board = emptyBoard();

    let letterNum = 0;
    let numberNum = 7;
    for (const fenCharacter of fenBoardPart) {
        switch (fenCharacter.toLowerCase()) {
            case 'p':
            case 'r':
            case 'n':
            case 'b':
            case 'q':
            case 'k':
                board[letterNum][numberNum] = fenCharacterToChessPiece(fenCharacter);
                letterNum += 1;
                break;
            case '/':
                numberNum -= 1;
                letterNum = 0;
                break;
            default:
                if (fenCharacter < '1' || fenCharacter > '8') {
                    throw new Error('Invalid FEN character, expected a number in the range \'1-8\' but got \'' + fenCharacter + '\'');
                }

                letterNum += parseInt(fenCharacter);
                break;

        }
    }

    return board;
}

function emptyBoard(): (ChessPiece | null)[][] {
    let board: (ChessPiece | null)[][] = [];

    for (let i = 0; i < 8; i++) {
        let tmp: (ChessPiece | null)[] = [];
        for (let j = 0; j < 8; j++) {
          tmp.push(null);
        }
        board.push(tmp);
      }
  
      return board;
}

function fenCharacterToChessPiece(fenCharacter: string): ChessPiece {
    if (fenCharacter.length !== 1) {
        throw new Error('Invalid FEN character length, expected \'1\' but got \'' + fenCharacter.length + '\'');
    }

    let pieceType: ChessType;
    switch (fenCharacter.toLowerCase()) {
        case 'p':
            pieceType = ChessType.Pawn;
            break;
        case 'r':
            pieceType = ChessType.Rook;
            break;
        case 'b':
            pieceType = ChessType.Bishop;
            break;
        case 'n':
            pieceType = ChessType.Knight;
            break;
        case 'q':
            pieceType = ChessType.Queen;
            break;
        case 'k':
            pieceType = ChessType.King;
            break;
        default:
            const validFenPieceCharacters = ['p', 'r', 'b', 'n', 'q', 'k', 'P', 'R', 'B', 'N', 'Q', 'K'];
            throw new Error('Invalid FEN character, expected one of \'' + validFenPieceCharacters + '\' but got \'' + fenCharacter + '\'');
    }

    let pieceColor = ChessColor.Black;
    if (fenCharacter === fenCharacter.toUpperCase()) {
        pieceColor = ChessColor.White;
    }

    return new ChessPiece(pieceType, pieceColor);
}

function parseBoardStateMovesToPossibleMoves(moves: [string, string, string][]): PossibleMove[][][] {
    // First element is the move as a string, example: 'a2 a4'
    // Second element is the resulting fen string of making that move
    // Third element is the resulting winState as a string of making that move
    let possibleMoves = emptyPossibleMoves();

    for (const element of moves) {
        const movString = element[0];
        const fenString = element[1];
        const winStateString = element[2];

        const [xFrom, yFrom, xTo, yTo] = parseStringChessMove(movString);
        const winStateAsEnum = parseWinState(winStateString);
        const possibleMove = new PossibleMove(xTo, yTo, fenString, winStateAsEnum);

        possibleMoves[xFrom][yFrom].push(possibleMove);
    }

    return possibleMoves;
}

function emptyPossibleMoves(): PossibleMove[][][] {
    let possibleMovesBoard = [];

    for (let i = 0; i < 8; i++) {
      let tmp = [];
      for (let j = 0; j < 8; j++) {
        tmp.push([]);
      }
      possibleMovesBoard.push(tmp);
    }

    return possibleMovesBoard;
}

// Chess move could be 'a2 a4' which would be (0, 1), (0, 3) as numbers
function parseStringChessMove(movString: string): [number, number, number, number] {
    if (movString.length !== 5 || movString[2] !== ' ') {
        throw new Error('Invalid move string, expected the format \'a2 a4\', but got \'' + movString + '\'');
    }

    const fromStringPart = movString.slice(0, 2);
    const toStringPart = movString.slice(3, 5);

    const fromNumberPart = parseChessPosStringToNumbers(fromStringPart);
    const toNumberPart = parseChessPosStringToNumbers(toStringPart);

    return [fromNumberPart[0], fromNumberPart[1], toNumberPart[0], toNumberPart[1]];
}

// Chess position could be 'e6' as string and would be (4, 5) as numbers
function parseChessPosStringToNumbers(chessPosString: string): [number, number] {
    const letterPart = chessPosString[0];
    const numberPart = chessPosString[1];

    if (chessPosString.length !== 2) {
        throw new Error('Invalid chess pos length, expected 2 but got \'' + chessPosString.length + '\'');
    }

    if (letterPart < 'a' || letterPart > 'h') {
        throw new Error('Invalid chess pos letter, expected in range a-h but got \'' + letterPart + '\'');
    }

    if (numberPart < '1' || numberPart > '8') {
    throw new Error('Invalid chess pos number, expected in range 1-8 but got \'' + numberPart + '\'');
    }

    const letterNumber = letterPart.charCodeAt(0) - 97;
    const numberNumber = parseInt(numberPart) - 1;

    return [letterNumber, numberNumber];
}

function parseTurnState(turnState: string): ChessColor {
    switch (turnState) {
        case 'White':
            return ChessColor.White;
        case 'Black':
            return ChessColor.Black;
        default:
            const validValues = ['White', 'Black'];
            throw new Error('Error parsing TurnState, expected an element from \'' + validValues + '\' but got \'' + turnState + '\'');
    }
}

function parseWinState(winState: string): WinState {
    switch (winState) {
        case 'NoEnd':
            return WinState.NoEnd;
        case 'Tie':
            return WinState.Tie;
        case 'WhiteWin':
            return WinState.WhiteWin;
        case 'BlackWin':
            return WinState.BlackWin;
        default:
            const validValues = ['NoEnd', 'Tie', 'WhiteWin', 'BlackWin'];
            throw new Error('Error parsing WinState, expected an element from \'' + validValues + '\' but got \'' + winState + '\'');
    }
}
