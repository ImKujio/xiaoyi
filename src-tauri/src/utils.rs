use display_info::DisplayInfo;
use tauri::{PhysicalPosition, Position};
use winapi::shared::windef::POINT;
use winapi::um::winuser::GetCursorPos;

fn cur_display_info(x: i32, y: i32) -> (i32, i32) {
    let info = DisplayInfo::from_point(x, y).unwrap();
    return (info.width as i32, info.height as i32);
}

fn cur_cursor_pos() -> (i32, i32) {
    let mut pos = POINT { x: -1, y: -1 };
    let _ = unsafe { GetCursorPos(&mut pos) };
    return (pos.x, pos.y);
}

pub fn window_pos(size: (i32, i32), center: bool) -> Position {
    let pos = cur_cursor_pos();
    let dis = cur_display_info(pos.0, pos.1);
    if pos.0 + size.0 > dis.0 && pos.1 + size.1 > dis.1 {
        return Position::Physical(PhysicalPosition { x: pos.0 - size.0, y: pos.1 - size.1 });
    } else if pos.0 + size.0 > dis.0 {
        return Position::Physical(PhysicalPosition { x: pos.0 - size.0, y: pos.1 });
    } else if pos.1 + size.1 > dis.1 {
        return Position::Physical(PhysicalPosition { x: pos.0, y: pos.1 - size.1 });
    } else {
        return Position::Physical(PhysicalPosition { x: pos.0, y: pos.1 });
    }
}