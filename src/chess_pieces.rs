enum Color {
    WHITE,
    BLACK,
}
trait ChessPiece {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn valid_moves(&self) -> Vec<(i32,i32)>;
}
struct Pawn {
    _has_moved: bool,
    _x: i32, _y: i32,
}
impl ChessPiece for Pawn {
    fn x(&self) -> i32 {
        self._x
    }
    fn y(&self) -> i32 {
        self._y
    }
    fn valid_moves(&self) -> Vec<(i32,i32)> {
        let moves = Vec::<(i32,i32)>::new();
        if self._has_moved {
            add_if_on_board(self._x, self._y)
        }
    }
}
fn add_if_on_board(x:i32, y:i32, v: &mut Vec<(i32,i32)>) {
    if x >= 0 && x <= 7 && y >= 0 && y <= 7 {
        v.push((x,y))
    }
}
fn y_direction(color: Color) -> i32 { // returns 1 or -1
    match color {
        BLACK => -1,
        WHITE => 1,
    }
}
