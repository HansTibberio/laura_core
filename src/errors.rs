/*
    Laura-Core: a fast and efficient move generator for chess engines.

    Copyright (C) 2024-2026 HansTibberio <hanstiberio@proton.me>

    Laura-Core is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Laura-Core is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Laura-Core. If not, see <https://www.gnu.org/licenses/>.
*/

use core::fmt;

/// Errors that can occur while parsing castling rights from a FEN string.
///
/// This error type is returned when converting a string slice into
/// [`CastleRights`] using the [`FromStr`] trait.
///
/// According to the FEN specification, castling rights must be represented
/// either by one or more of the characters `K`, `Q`, `k`, `q`, or by a single
/// dash (`-`) to indicate that no castling rights are available.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CastleRightsParseError {
    /// An invalid character was encountered while parsing castling rights (not KQkq-)
    InvalidChar(char),

    /// '-' must be the only character
    InvalidDashUsage,
}

impl fmt::Display for CastleRightsParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CastleRightsParseError::InvalidChar(ch) => {
                write!(f, "invalid castling character '{}'", ch)
            }
            CastleRightsParseError::InvalidDashUsage => {
                write!(f, "'-' must be the only character in castling rights")
            }
        }
    }
}

/// Errors that can occur while parsing a FEN string into a [`Board`].
///
/// `BoardParseError` represents all possible failures that may happen during
/// the parsing and validation of a FEN position, including structural errors,
/// invalid fields, and semantic violations of the FEN specification.
///
/// The error type is intentionally fine-grained to allow precise diagnostics
/// and easier debugging when loading positions from FEN, EPD, or test suites.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardParseError {
    /// The FEN string does not contain all required fields.
    FenTooShort,

    /// The side-to-move field is missing.
    MissingSideToMove,

    /// The castling rights field is missing.
    MissingCastlingRights,

    /// The en passant target square field is missing.
    MissingEnPassant,

    /// The halfmove clock field is missing.
    MissingHalfmoveClock,

    /// The fullmove number field is missing.
    MissingFullmoveNumber,

    /// A rank in the board description does not contain exactly 8 squares.
    InvalidRowLength,

    /// The board layout is invalid or malformed.
    InvalidBoardLayout,

    /// An invalid piece character was found in the board layout.
    InvalidPiece(PieceParseError),

    /// The side-to-move field is invalid (expected `'w'` or `'b'`).
    InvalidSideToMove,

    /// The castling rights field is invalid.
    InvalidCastlingRights(CastleRightsParseError),

    /// The en passant target square is syntactically invalid.
    InvalidEnPassantSquare(SquareParseError),

    /// The en passant square has an invalid rank (must be rank 3 or 6).
    InvalidEnPassantRank,

    /// The halfmove clock is not a valid number.
    InvalidHalfmoveClock,

    /// The halfmove clock exceeds the maximum allowed value (100).
    HalfmoveClockOverflow,

    /// The fullmove number is not a valid number.
    InvalidFullmoveNumber,

    /// The fullmove number is zero or negative.
    FullmoveMustBePositive,
}

impl fmt::Display for BoardParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardParseError::FenTooShort => f.write_str("FEN string is too short"),

            BoardParseError::MissingSideToMove => f.write_str("Missing side to move in FEN"),

            BoardParseError::MissingCastlingRights => f.write_str("Missing castling rights in FEN"),

            BoardParseError::MissingEnPassant => f.write_str("Missing en passant square in FEN"),

            BoardParseError::MissingHalfmoveClock => f.write_str("Missing halfmove clock in FEN"),

            BoardParseError::MissingFullmoveNumber => f.write_str("Missing fullmove number in FEN"),

            BoardParseError::InvalidRowLength => {
                f.write_str("A FEN rank does not contain exactly 8 squares")
            }

            BoardParseError::InvalidBoardLayout => f.write_str("Invalid board layout in FEN"),

            BoardParseError::InvalidPiece(err) => {
                write!(f, "{}", err)
            }

            BoardParseError::InvalidSideToMove => {
                f.write_str("Invalid side to move (expected 'w' or 'b')")
            }

            BoardParseError::InvalidCastlingRights(err) => {
                write!(f, "Invalid castling rights: {}", err)
            }

            BoardParseError::InvalidEnPassantSquare(err) => {
                write!(f, "{}", err)
            }

            BoardParseError::InvalidEnPassantRank => {
                f.write_str("Invalid en passant rank (must be rank 3 or 6)")
            }

            BoardParseError::InvalidHalfmoveClock => f.write_str("Invalid halfmove clock value"),

            BoardParseError::HalfmoveClockOverflow => {
                f.write_str("Halfmove clock exceeds the maximum allowed value (100)")
            }

            BoardParseError::InvalidFullmoveNumber => f.write_str("Invalid fullmove number value"),

            BoardParseError::FullmoveMustBePositive => {
                f.write_str("Fullmove number must be greater than zero")
            }
        }
    }
}

/// Errors that can occur when parsing a chess piece from a character.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceParseError {
    /// The character does not correspond to any valid chess piece.
    InvalidChar(char),
}

impl fmt::Display for PieceParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceParseError::InvalidChar(c) => {
                write!(f, "Invalid piece character '{}'", c)
            }
        }
    }
}

/// Errors that can occur when parsing a square from algebraic notation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SquareParseError {
    /// The input does not have exactly two characters.
    InvalidLength,

    /// The square name is not a valid algebraic square (a1–h8).
    InvalidName,
}

impl fmt::Display for SquareParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SquareParseError::InvalidLength => {
                write!(f, "Invalid square length (expected exactly 2 characters)")
            }
            SquareParseError::InvalidName => write!(f, "Invalid square name"),
        }
    }
}
