#![windows_subsystem = "windows"]
#![allow(non_snake_case)]
use std::{cell::RefCell, ffi::c_void, path::PathBuf};
type H = *mut c_void;
#[repr(C)]
#[derive(Default, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
#[repr(C)]
struct Msg {
    hwnd: H,
    message: u32,
    wparam: usize,
    lparam: isize,
    time: u32,
    pt: Point,
    private: u32,
}
#[repr(C)]
struct Wc {
    style: u32,
    proc: Option<unsafe extern "system" fn(H, u32, usize, isize) -> isize>,
    cls: i32,
    wnd: i32,
    instance: H,
    icon: H,
    cursor: H,
    brush: H,
    menu: *const u16,
    name: *const u16,
}
#[repr(C)]
struct Paint {
    dc: H,
    erase: i32,
    rect: Rect,
    restore: i32,
    update: i32,
    reserved: [u8; 32],
}
#[repr(C)]
struct LogFont {
    height: i32,
    width: i32,
    escapement: i32,
    orientation: i32,
    weight: i32,
    italic: u8,
    underline: u8,
    strikeout: u8,
    charset: u8,
    out_precision: u8,
    clip_precision: u8,
    quality: u8,
    pitch: u8,
    face: [u16; 32],
}
#[repr(C)]
struct NotifyIconData {
    size: u32,
    hwnd: H,
    id: u32,
    flags: u32,
    callback: u32,
    icon: H,
    tip: [u16; 128],
    state: u32,
    state_mask: u32,
    info: [u16; 256],
    version: u32,
    info_title: [u16; 64],
    info_flags: u32,
    guid: [u8; 16],
    balloon_icon: H,
}
#[link(name = "shell32")]
extern "system" {
    fn Shell_NotifyIconW(action: u32, data: *const NotifyIconData) -> i32;
}
#[link(name = "user32")]
extern "system" {
    fn RegisterClassW(c: *const Wc) -> u16;
    fn CreateWindowExW(
        ex: u32,
        class: *const u16,
        title: *const u16,
        style: u32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        parent: H,
        menu: H,
        instance: H,
        param: H,
    ) -> H;
    fn DefWindowProcW(h: H, m: u32, w: usize, l: isize) -> isize;
    fn ShowWindow(h: H, n: i32) -> i32;
    fn DestroyWindow(h: H) -> i32;
    fn IsWindow(h: H) -> i32;
    fn IsWindowVisible(h: H) -> i32;
    fn InvalidateRect(h: H, rect: *const Rect, erase: i32) -> i32;
    fn IsIconic(h: H) -> i32;
    fn GetDC(h: H) -> H;
    fn ReleaseDC(h: H, dc: H) -> i32;
    fn GetMessageW(m: *mut Msg, h: H, a: u32, b: u32) -> i32;
    fn TranslateMessage(m: *const Msg) -> i32;
    fn DispatchMessageW(m: *const Msg) -> isize;
    fn PostQuitMessage(n: i32);
    fn SendMessageW(h: H, m: u32, w: usize, l: isize) -> isize;
    fn SetWindowTextW(h: H, s: *const u16) -> i32;
    fn GetWindowTextW(h: H, s: *mut u16, n: i32) -> i32;
    fn GetDlgItem(h: H, id: i32) -> H;
    fn LoadCursorW(h: H, name: *const u16) -> H;
    fn LoadIconW(h: H, name: *const u16) -> H;
    fn BeginPaint(h: H, p: *mut Paint) -> H;
    fn EndPaint(h: H, p: *const Paint) -> i32;
    fn FillRect(dc: H, r: *const Rect, b: H) -> i32;
    fn GetClientRect(h: H, r: *mut Rect) -> i32;
    fn DrawTextW(dc: H, t: *const u16, n: i32, r: *mut Rect, flags: u32) -> i32;
    fn SetLayeredWindowAttributes(h: H, key: u32, alpha: u8, flags: u32) -> i32;
    fn GetLayeredWindowAttributes(h: H, key: *mut u32, alpha: *mut u8, flags: *mut u32) -> i32;
    fn SetWindowLongPtrW(h: H, index: i32, v: isize) -> isize;
    fn GetWindowLongPtrW(h: H, index: i32) -> isize;
    fn GetWindowRect(h: H, r: *mut Rect) -> i32;
    fn SetWindowPos(h: H, after: H, x: i32, y: i32, w: i32, height: i32, flags: u32) -> i32;
    fn MessageBoxW(h: H, t: *const u16, c: *const u16, f: u32) -> i32;
    fn SetProcessDPIAware() -> i32;
    fn AdjustWindowRectEx(rect: *mut Rect, style: u32, menu: i32, ex: u32) -> i32;
    fn SystemParametersInfoW(action: u32, param: u32, data: *mut c_void, flags: u32) -> i32;
    fn ScreenToClient(h: H, point: *mut Point) -> i32;
    fn RegisterHotKey(h: H, id: i32, mods: u32, key: u32) -> i32;
    fn UnregisterHotKey(h: H, id: i32) -> i32;
    fn RegisterWindowMessageW(name: *const u16) -> u32;
    fn SetForegroundWindow(h: H) -> i32;
    fn CreatePopupMenu() -> H;
    fn AppendMenuW(menu: H, flags: u32, id: usize, text: *const u16) -> i32;
    fn TrackPopupMenu(
        menu: H,
        flags: u32,
        x: i32,
        y: i32,
        reserved: i32,
        h: H,
        rect: *const Rect,
    ) -> i32;
    fn DestroyMenu(menu: H) -> i32;
    fn GetCursorPos(point: *mut Point) -> i32;
    fn PostMessageW(h: H, msg: u32, w: usize, l: isize) -> i32;
}
#[link(name = "gdi32")]
extern "system" {
    fn EnumFontFamiliesExW(
        dc: H,
        font: *const LogFont,
        callback: Option<
            unsafe extern "system" fn(*const LogFont, *const c_void, u32, isize) -> i32,
        >,
        param: isize,
        flags: u32,
    ) -> i32;
    fn CreateSolidBrush(c: u32) -> H;
    fn DeleteObject(h: H) -> i32;
    fn SelectObject(dc: H, obj: H) -> H;
    fn CreateFontW(
        height: i32,
        width: i32,
        esc: i32,
        orient: i32,
        weight: i32,
        italic: u32,
        underline: u32,
        strike: u32,
        charset: u32,
        out: u32,
        clip: u32,
        quality: u32,
        pitch: u32,
        face: *const u16,
    ) -> H;
    fn SetTextColor(dc: H, c: u32) -> u32;
    fn SetBkMode(dc: H, m: i32) -> i32;
    fn GetStockObject(n: i32) -> H;
    fn GetTextExtentPoint32W(dc: H, text: *const u16, len: i32, size: *mut Point) -> i32;
}
#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(s: *const u16) -> H;
}
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
unsafe extern "system" fn font_callback(
    font: *const LogFont,
    _metric: *const c_void,
    _kind: u32,
    param: isize,
) -> i32 {
    let face = &(*font).face;
    let end = face.iter().position(|c| *c == 0).unwrap_or(face.len());
    let name = String::from_utf16_lossy(&face[..end]);
    if !name.is_empty() && !name.starts_with('@') {
        (*(param as *mut std::collections::BTreeSet<String>)).insert(name);
    }
    1
}
unsafe fn installed_fonts() -> Vec<String> {
    let mut names = std::collections::BTreeSet::new();
    let dc = GetDC(std::ptr::null_mut());
    if !dc.is_null() {
        let mut font: LogFont = std::mem::zeroed();
        font.charset = 1;
        EnumFontFamiliesExW(
            dc,
            &font,
            Some(font_callback),
            &mut names as *mut _ as isize,
            0,
        );
        ReleaseDC(std::ptr::null_mut(), dc);
    }
    names.into_iter().collect()
}
const KEY: u32 = 0x010001;
#[repr(C)]
struct ChooseColor {
    size: u32,
    owner: H,
    instance: H,
    result: u32,
    custom: *mut u32,
    flags: u32,
    data: isize,
    hook: H,
    template: *const u16,
}
#[repr(C)]
struct DrawItem {
    kind: u32,
    id: u32,
    item: u32,
    action: u32,
    state: u32,
    window: H,
    dc: H,
    rect: Rect,
    data: usize,
}
#[repr(C)]
struct InitControls {
    size: u32,
    classes: u32,
}
#[link(name = "comdlg32")]
extern "system" {
    fn ChooseColorW(data: *mut ChooseColor) -> i32;
}
#[link(name = "comctl32")]
extern "system" {
    fn InitCommonControlsEx(data: *const InitControls) -> i32;
}
#[link(name = "advapi32")]
extern "system" {
    fn RegOpenKeyExW(key: H, path: *const u16, options: u32, access: u32, result: *mut H) -> i32;
    fn RegCreateKeyExW(
        key: H,
        path: *const u16,
        reserved: u32,
        class: *mut u16,
        options: u32,
        access: u32,
        security: H,
        result: *mut H,
        disposition: *mut u32,
    ) -> i32;
    fn RegSetValueExW(
        key: H,
        name: *const u16,
        reserved: u32,
        kind: u32,
        data: *const u8,
        size: u32,
    ) -> i32;
    fn RegQueryValueExW(
        key: H,
        name: *const u16,
        reserved: *mut u32,
        kind: *mut u32,
        data: *mut u8,
        size: *mut u32,
    ) -> i32;
    fn RegDeleteValueW(key: H, name: *const u16) -> i32;
    fn RegCloseKey(key: H) -> i32;
    fn RegDeleteKeyW(key: H, path: *const u16) -> i32;
}
fn startup_key() -> String {
    if std::env::args().any(|a| a == "--smoke-test") {
        format!("Software\\DesktopInk-Test-{}", std::process::id())
    } else {
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run".into()
    }
}
unsafe extern "system" fn accept_test_color(h: H, m: u32, _w: usize, _l: isize) -> usize {
    if m == 0x110 {
        PostMessageW(h, 0x111, 1, 0);
    }
    0
}
fn rgb_text(c: u32) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        c & 255,
        (c >> 8) & 255,
        (c >> 16) & 255
    )
}
fn parse_color(s: &str) -> Option<u32> {
    let s = s.trim().trim_start_matches('#');
    if s.len() != 6 {
        return None;
    }
    let c = u32::from_str_radix(s, 16).ok()?;
    Some(((c & 255) << 16) | (c & 0xff00) | ((c >> 16) & 255))
}
unsafe fn redraw(h: H) {
    InvalidateRect(h, std::ptr::null(), 1);
}
unsafe fn pick_color(h: H, id: i32) {
    let mut custom = [0u32; 16];
    let mut dialog: ChooseColor = std::mem::zeroed();
    dialog.size = std::mem::size_of::<ChooseColor>() as u32;
    dialog.owner = h;
    dialog.result = parse_color(&text(h, id)).unwrap_or(0xffffff);
    dialog.custom = custom.as_mut_ptr();
    dialog.flags = 1 | 2;
    if std::env::args().any(|a| a == "--smoke-test") {
        dialog.flags |= 0x10;
        dialog.hook = accept_test_color as *const () as H;
    }
    if ChooseColorW(&mut dialog) != 0 {
        set(h, id, &rgb_text(dialog.result));
        if id == 108 {
            SendMessageW(GetDlgItem(h, 109), 0xf1, 0, 0);
        }
        redraw(GetDlgItem(h, if id == 105 { 207 } else { 208 }));
    }
}
unsafe fn draw_color(h: H, d: &DrawItem) {
    let id = if d.id == 207 { 105 } else { 108 };
    let c = parse_color(&text(h, id)).unwrap_or(0xffffff);
    let b = CreateSolidBrush(c);
    FillRect(d.dc, &d.rect, b);
    DeleteObject(b);
    SetBkMode(d.dc, 1);
    let luminance = (c & 255) * 299 + ((c >> 8) & 255) * 587 + ((c >> 16) & 255) * 114;
    SetTextColor(d.dc, if luminance > 128000 { 0 } else { 0xffffff });
    let mut r = d.rect;
    DrawTextW(
        d.dc,
        wide(if id == 105 {
            "选择字体颜色…"
        } else {
            "选择背景颜色…"
        })
        .as_ptr(),
        -1,
        &mut r,
        1 | 4 | 0x20,
    );
}
unsafe fn startup_enabled() -> bool {
    let mut key = std::ptr::null_mut();
    let path = wide(&startup_key());
    if RegOpenKeyExW(
        0x80000001u32 as i32 as isize as H,
        path.as_ptr(),
        0,
        0x20019,
        &mut key,
    ) != 0
    {
        return false;
    }
    let mut kind = 0;
    let mut size = 4096;
    let mut value = vec![0u16; 2048];
    let result = RegQueryValueExW(
        key,
        wide("DesktopInk").as_ptr(),
        std::ptr::null_mut(),
        &mut kind,
        value.as_mut_ptr() as *mut u8,
        &mut size,
    );
    RegCloseKey(key);
    result == 0
        && kind == 1
        && String::from_utf16_lossy(&value[..(size as usize / 2).min(value.len())])
            .trim_end_matches('\0')
            == startup_command()
}
fn startup_command() -> String {
    format!(
        "\"{}\" --background",
        std::env::current_exe().unwrap_or_default().display()
    )
}
unsafe fn set_startup(enabled: bool) -> Result<(), i32> {
    let mut key = std::ptr::null_mut();
    let path = wide(&startup_key());
    let result = RegCreateKeyExW(
        0x80000001u32 as i32 as isize as H,
        path.as_ptr(),
        0,
        std::ptr::null_mut(),
        0,
        0x20006,
        std::ptr::null_mut(),
        &mut key,
        std::ptr::null_mut(),
    );
    if result != 0 {
        return Err(result);
    }
    let result = if enabled {
        let command = wide(&startup_command());
        RegSetValueExW(
            key,
            wide("DesktopInk").as_ptr(),
            0,
            1,
            command.as_ptr() as *const u8,
            (command.len() * 2) as u32,
        )
    } else {
        RegDeleteValueW(key, wide("DesktopInk").as_ptr())
    };
    RegCloseKey(key);
    if result == 0 || (!enabled && result == 2) {
        Ok(())
    } else {
        Err(result)
    }
}
#[derive(Clone, Debug, PartialEq)]
struct Note {
    text: String,
    font: String,
    size: i32,
    color: u32,
    background: u32,
    clear_background: bool,
    opacity: u8,
    topmost: bool,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}
impl Default for Note {
    fn default() -> Self {
        Self {
            text: "在桌面写下你的文字\r\n拖动文字可调整位置".into(),
            font: "Microsoft YaHei UI".into(),
            size: 28,
            color: 0xffffff,
            background: 0x292019,
            clear_background: true,
            opacity: 255,
            topmost: false,
            x: 80,
            y: 100,
            width: 480,
            height: 180,
        }
    }
}
#[derive(Default)]
struct App {
    notes: Vec<Note>,
    windows: Vec<H>,
    manager: H,
    selected: Option<usize>,
    locked: bool,
    loading: bool,
    tray: bool,
    taskbar_created: u32,
    ui_font: H,
}
const TRAY_MESSAGE: u32 = 0x8001;
unsafe fn tray_data(h: H) -> NotifyIconData {
    let mut data: NotifyIconData = std::mem::zeroed();
    data.size = std::mem::size_of::<NotifyIconData>() as u32;
    data.hwnd = h;
    data.id = 1;
    data.flags = 1 | 2 | 4;
    data.callback = TRAY_MESSAGE;
    data.icon = LoadIconW(GetModuleHandleW(std::ptr::null()), 1 as *const u16);
    let tip = wide("Desktop Ink · 点击打开，右键退出");
    data.tip[..tip.len()].copy_from_slice(&tip);
    data
}
unsafe fn add_tray(h: H) -> bool {
    if APP.with(|a| a.borrow().tray) {
        return true;
    }
    let ok = Shell_NotifyIconW(0, &tray_data(h)) != 0;
    APP.with(|a| a.borrow_mut().tray = ok);
    ok
}
unsafe fn restore_manager(h: H) {
    ShowWindow(h, 9);
    SetForegroundWindow(h);
}
unsafe fn tray_menu(h: H) {
    let menu = CreatePopupMenu();
    if menu.is_null() {
        return;
    }
    AppendMenuW(menu, 0, 301, wide("打开管理窗口").as_ptr());
    AppendMenuW(menu, 0x800, 0, std::ptr::null());
    AppendMenuW(menu, 0, 206, wide("退出程序").as_ptr());
    let mut p = Point::default();
    GetCursorPos(&mut p);
    SetForegroundWindow(h);
    let command = TrackPopupMenu(menu, 0x100 | 2, p.x, p.y, 0, h, std::ptr::null());
    DestroyMenu(menu);
    PostMessageW(h, 0, 0, 0);
    if command == 301 {
        restore_manager(h)
    } else if command == 206 {
        SendMessageW(h, 0x111, 206, 0);
    }
}
thread_local! {static APP:RefCell<App>=RefCell::new(App::default());}
fn config_path() -> PathBuf {
    if std::env::args().any(|a| a == "--smoke-test") {
        std::env::temp_dir().join(format!("desktop-ink-test-{}.settings", std::process::id()))
    } else {
        std::env::current_exe()
            .unwrap_or_default()
            .with_file_name("desktop-ink.settings")
    }
}
fn hex(s: &str) -> String {
    s.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}
fn unhex(s: &str) -> Option<String> {
    if s.len() % 2 != 0 {
        return None;
    }
    let b: Option<Vec<u8>> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok())
        .collect();
    String::from_utf8(b?).ok()
}
fn encode(notes: &[Note], locked: bool) -> String {
    let mut s = format!("DesktopInk3\n{}\n", locked as u8);
    for n in notes {
        s.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            n.x,
            n.y,
            n.width,
            n.height,
            n.size,
            n.color,
            hex(&n.font),
            hex(&n.text),
            n.background,
            n.clear_background as u8,
            n.opacity,
            n.topmost as u8
        ));
    }
    s
}
fn decode(s: &str) -> Option<(Vec<Note>, bool)> {
    let mut lines = s.lines();
    let version = lines.next()?;
    if version != "DesktopInk1" && version != "DesktopInk2" && version != "DesktopInk3" {
        return None;
    }
    let locked = lines.next()? == "1";
    let mut notes = Vec::new();
    for line in lines {
        let p: Vec<_> = line.split('\t').collect();
        if p.len()
            != if version == "DesktopInk1" {
                8
            } else if version == "DesktopInk2" {
                11
            } else {
                12
            }
        {
            return None;
        }
        notes.push(Note {
            x: p[0].parse().ok()?,
            y: p[1].parse().ok()?,
            width: p[2].parse::<i32>().ok()?.clamp(100, 3000),
            height: p[3].parse::<i32>().ok()?.clamp(50, 2000),
            size: p[4].parse::<i32>().ok()?.clamp(8, 200),
            color: p[5].parse().ok()?,
            font: unhex(p[6])?,
            text: unhex(p[7])?,
            background: if version != "DesktopInk1" {
                p[8].parse().ok()?
            } else {
                0x292019
            },
            clear_background: version == "DesktopInk1" || p[9] == "1",
            opacity: if version != "DesktopInk1" {
                p[10].parse::<u8>().ok()?.max(26)
            } else {
                255
            },
            topmost: version == "DesktopInk3" && p[11] == "1",
        });
    }
    Some((notes, locked))
}
unsafe fn popup(h: H, s: &str) {
    MessageBoxW(h, wide(s).as_ptr(), wide("Desktop Ink").as_ptr(), 0x10);
}
unsafe fn save() {
    let result = APP.with(|a| {
        let a = a.borrow();
        std::fs::write(config_path(), encode(&a.notes, a.locked))
    });
    if let Err(e) = result {
        popup(APP.with(|a| a.borrow().manager), &format!("保存失败：{e}"));
    }
}
unsafe fn control(
    parent: H,
    class: &str,
    text: &str,
    id: i32,
    style: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> H {
    // Compact the editor section while retaining fixed gaps between controls.
    let y = if x >= 268 && y >= 240 { y - 52 } else { y };
    let handle = CreateWindowExW(
        if class == "EDIT" || class == "LISTBOX" {
            0x200
        } else {
            0
        },
        wide(class).as_ptr(),
        wide(text).as_ptr(),
        0x50000000 | style,
        x,
        y,
        w,
        h,
        parent,
        id as H,
        GetModuleHandleW(std::ptr::null()),
        std::ptr::null_mut(),
    );
    let font = APP.with(|a| a.borrow().ui_font);
    SendMessageW(
        handle,
        0x30,
        if font.is_null() {
            GetStockObject(17)
        } else {
            font
        } as usize,
        1,
    );
    handle
}
unsafe fn text(h: H, id: i32) -> String {
    let c = GetDlgItem(h, id);
    let len = SendMessageW(c, 0x0e, 0, 0).max(0).min(65536) as usize;
    let mut s = vec![0u16; len + 1];
    let n = GetWindowTextW(c, s.as_mut_ptr(), s.len() as i32);
    String::from_utf16_lossy(&s[..n.max(0) as usize])
}
unsafe fn set(h: H, id: i32, s: &str) {
    SetWindowTextW(GetDlgItem(h, id), wide(s).as_ptr());
}
unsafe fn selected() -> Option<usize> {
    APP.with(|a| a.borrow().selected)
}
unsafe fn fill_editor() {
    let data = APP.with(|a| {
        let mut a = a.borrow_mut();
        a.loading = true;
        (a.manager, a.selected.and_then(|i| a.notes.get(i).cloned()))
    });
    if let (h, Some(n)) = data {
        set(h, 102, &n.text);
        set(h, 103, &n.font);
        set(h, 104, &n.size.to_string());
        set(
            h,
            105,
            &format!(
                "#{:02X}{:02X}{:02X}",
                n.color & 255,
                (n.color >> 8) & 255,
                (n.color >> 16) & 255
            ),
        );
        set(h, 106, &n.width.to_string());
        set(h, 107, &n.height.to_string());
        set(h, 108, &rgb_text(n.background));
        SendMessageW(GetDlgItem(h, 109), 0xf1, n.clear_background as usize, 0);
        let transparency = ((255 - n.opacity as i32) * 100 + 127) / 255;
        SendMessageW(GetDlgItem(h, 110), 0x405, 1, transparency as isize);
        set(h, 111, &format!("{}%", transparency));
        SendMessageW(GetDlgItem(h, 112), 0xf1, n.topmost as usize, 0);
        redraw(GetDlgItem(h, 207));
        redraw(GetDlgItem(h, 208));
    }
    APP.with(|a| a.borrow_mut().loading = false);
}
unsafe fn refresh_list() {
    let (h, notes, sel, locked) = APP.with(|a| {
        let a = a.borrow();
        (a.manager, a.notes.clone(), a.selected, a.locked)
    });
    let list = GetDlgItem(h, 101);
    SendMessageW(list, 0x184, 0, 0);
    for (i, n) in notes.iter().enumerate() {
        let label = format!("{}  {}", i + 1, n.text.lines().next().unwrap_or("空文字"));
        SendMessageW(list, 0x180, 0, wide(&label).as_ptr() as isize);
    }
    if let Some(i) = sel {
        SendMessageW(list, 0x186, i, 0);
    }
    set(
        h,
        204,
        if locked {
            "解锁 / 允许拖动"
        } else {
            "锁定 / 点击穿透"
        },
    );
    fill_editor();
}
unsafe fn rebuild() {
    let old = APP.with(|a| std::mem::take(&mut a.borrow_mut().windows));
    for w in old {
        DestroyWindow(w);
    }
    let (notes, locked) = APP.with(|a| {
        let a = a.borrow();
        (a.notes.clone(), a.locked)
    });
    for (i, n) in notes.iter().enumerate() {
        let ex = 0x80000
            | 0x80
            | 0x08000000
            | if locked { 0x20 } else { 0 }
            | if n.topmost { 8 } else { 0 };
        let h = CreateWindowExW(
            ex,
            wide("DesktopInkNote").as_ptr(),
            wide("Desktop Ink 文字").as_ptr(),
            0x80000000,
            n.x,
            n.y,
            n.width,
            n.height,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            GetModuleHandleW(std::ptr::null()),
            std::ptr::null_mut(),
        );
        SetWindowLongPtrW(h, -21, (i + 1) as isize);
        SetLayeredWindowAttributes(h, KEY, n.opacity, if n.clear_background { 3 } else { 2 });
        APP.with(|a| a.borrow_mut().windows.push(h));
        ShowWindow(h, 4);
    }
    refresh_list();
}
unsafe extern "system" fn note_proc(h: H, m: u32, w: usize, l: isize) -> isize {
    let idx = GetWindowLongPtrW(h, -21) - 1;
    match m {
        0x84 => {
            return if APP.with(|a| a.borrow().locked) {
                -1
            } else {
                2
            }
        }
        0x21 => return 3,
        0x232 => {
            if idx >= 0 {
                let mut r = Rect::default();
                GetWindowRect(h, &mut r);
                APP.with(|a| {
                    if let Some(n) = a.borrow_mut().notes.get_mut(idx as usize) {
                        n.x = r.left;
                        n.y = r.top;
                    }
                });
                save();
            }
            return 0;
        }
        0x0f => {
            let mut p: Paint = std::mem::zeroed();
            let dc = BeginPaint(h, &mut p);
            let mut r = Rect::default();
            GetClientRect(h, &mut r);
            let background = APP.with(|a| {
                a.borrow()
                    .notes
                    .get(idx as usize)
                    .map(|n| {
                        if n.clear_background {
                            KEY
                        } else {
                            n.background
                        }
                    })
                    .unwrap_or(KEY)
            });
            let brush = CreateSolidBrush(background);
            FillRect(dc, &r, brush);
            DeleteObject(brush);
            let n = APP.with(|a| a.borrow().notes.get(idx as usize).cloned());
            if let Some(n) = n {
                let font = CreateFontW(
                    -n.size,
                    0,
                    0,
                    0,
                    400,
                    0,
                    0,
                    0,
                    1,
                    0,
                    0,
                    4,
                    0,
                    wide(&n.font).as_ptr(),
                );
                let old = SelectObject(dc, font);
                SetBkMode(dc, 1);
                r.left = 8;
                r.top = 8;
                r.right -= 8;
                r.bottom -= 8;
                let t = wide(&n.text);
                let mut shadow = r;
                shadow.left += 1;
                shadow.top += 1;
                SetTextColor(dc, 0);
                DrawTextW(dc, t.as_ptr(), -1, &mut shadow, 0x10 | 0x800);
                SetTextColor(dc, n.color);
                DrawTextW(dc, t.as_ptr(), -1, &mut r, 0x10 | 0x800);
                SelectObject(dc, old);
                DeleteObject(font);
            }
            EndPaint(h, &p);
            return 0;
        }
        _ => {}
    }
    DefWindowProcW(h, m, w, l)
}
unsafe fn apply(h: H) {
    let Some(i) = selected() else { return };
    let size = text(h, 104).parse::<i32>();
    let width = text(h, 106).parse::<i32>();
    let height = text(h, 107).parse::<i32>();
    let c = text(h, 105);
    let c = c.trim().trim_start_matches('#');
    let color = if c.len() == 6 {
        u32::from_str_radix(c, 16).ok()
    } else {
        None
    };
    let (Ok(size), Ok(width), Ok(height), Some(rgb)) = (size, width, height, color) else {
        popup(h, "字号、宽度和高度请输入数字，颜色格式为 #RRGGBB。");
        return;
    };
    if !(8..=200).contains(&size)
        || !(100..=3000).contains(&width)
        || !(50..=2000).contains(&height)
    {
        popup(h, "字号范围 8–200，宽度 100–3000，高度 50–2000。");
        return;
    }
    let content = text(h, 102);
    let font = text(h, 103);
    let Some(background) = parse_color(&text(h, 108)) else {
        popup(h, "背景颜色格式应为 #RRGGBB。");
        return;
    };
    let clear_background = SendMessageW(GetDlgItem(h, 109), 0xf0, 0, 0) == 1;
    let topmost = SendMessageW(GetDlgItem(h, 112), 0xf0, 0, 0) == 1;
    let transparency = SendMessageW(GetDlgItem(h, 110), 0x400, 0, 0).clamp(0, 90) as u32;
    APP.with(|a| {
        let mut a = a.borrow_mut();
        let n = &mut a.notes[i];
        n.text = content;
        n.font = if font.trim().is_empty() {
            "Microsoft YaHei UI".into()
        } else {
            font
        };
        n.size = size;
        n.width = width;
        n.height = height;
        n.color = ((rgb & 255) << 16) | (rgb & 0xff00) | ((rgb >> 16) & 255);
        n.background = background;
        n.clear_background = clear_background;
        n.topmost = topmost;
        n.opacity = ((100 - transparency) * 255 + 50).div_euclid(100) as u8;
    });
    rebuild();
    save();
}
unsafe extern "system" fn manager_proc(h: H, m: u32, w: usize, l: isize) -> isize {
    let restarted = APP.with(|a| {
        let a = a.borrow();
        a.taskbar_created != 0 && m == a.taskbar_created
    });
    if restarted {
        APP.with(|a| a.borrow_mut().tray = false);
        if !add_tray(h) && IsWindowVisible(h) == 0 {
            ShowWindow(h, 6);
        }
        return 0;
    }
    match m {
        0x2b => {
            let d = &*(l as *const DrawItem);
            if d.id == 207 || d.id == 208 {
                draw_color(h, d);
                return 1;
            }
        }
        0x114 => {
            if l == GetDlgItem(h, 110) as isize {
                let value = SendMessageW(GetDlgItem(h, 110), 0x400, 0, 0);
                set(h, 111, &format!("{}%", value));
            }
            return 0;
        }
        TRAY_MESSAGE => {
            if w == 1 {
                match l as u32 {
                    0x202 | 0x203 => restore_manager(h),
                    0x205 => tray_menu(h),
                    _ => {}
                }
            }
            return 0;
        }
        0x111 => {
            let id = (w & 0xffff) as i32;
            let event = w >> 16;
            if (id == 105 || id == 108) && event == 0x300 {
                redraw(GetDlgItem(h, if id == 105 { 207 } else { 208 }));
            }
            match id {
                207 => pick_color(h, 105),
                208 => pick_color(h, 108),
                210 => {
                    let enabled = SendMessageW(GetDlgItem(h, 210), 0xf0, 0, 0) == 1;
                    if let Err(code) = set_startup(enabled) {
                        SendMessageW(GetDlgItem(h, 210), 0xf1, (!enabled) as usize, 0);
                        popup(h, &format!("开机自启动设置失败，错误码 {code}"));
                    }
                }
                101 if event == 1 => {
                    let i = SendMessageW(GetDlgItem(h, 101), 0x188, 0, 0);
                    APP.with(|a| {
                        a.borrow_mut().selected = if i >= 0 { Some(i as usize) } else { None }
                    });
                    fill_editor();
                }
                201 => {
                    APP.with(|a| {
                        let mut a = a.borrow_mut();
                        let offset = a.notes.len() as i32 * 24;
                        let mut n = Note::default();
                        n.x += offset % 400;
                        n.y += offset % 400;
                        a.notes.push(n);
                        a.selected = Some(a.notes.len() - 1);
                    });
                    rebuild();
                    save();
                }
                202 => {
                    if let Some(i) = selected() {
                        APP.with(|a| {
                            let mut a = a.borrow_mut();
                            a.notes.remove(i);
                            a.selected = if a.notes.is_empty() {
                                None
                            } else {
                                Some(i.min(a.notes.len() - 1))
                            };
                        });
                        rebuild();
                        save();
                    }
                }
                203 => apply(h),
                206 => {
                    save();
                    DestroyWindow(h);
                }
                204 => {
                    APP.with(|a| {
                        let mut a = a.borrow_mut();
                        a.locked = !a.locked;
                    });
                    rebuild();
                    save();
                }
                205 => {
                    if let Some(i) = selected() {
                        APP.with(|a| {
                            let mut a = a.borrow_mut();
                            a.notes[i].x = 80;
                            a.notes[i].y = 100;
                        });
                        rebuild();
                        save();
                    }
                }
                _ => {}
            }
            return 0;
        }
        0x312 => {
            restore_manager(h);
            return 0;
        }
        0x10 => {
            save();
            if add_tray(h) {
                ShowWindow(h, 0);
            } else {
                ShowWindow(h, 6);
            }
            return 0;
        }
        0x02 => {
            if APP.with(|a| a.borrow().tray) {
                Shell_NotifyIconW(2, &tray_data(h));
                APP.with(|a| a.borrow_mut().tray = false);
            }
            UnregisterHotKey(h, 1);
            PostQuitMessage(0);
            return 0;
        }
        _ => {}
    }
    DefWindowProcW(h, m, w, l)
}
unsafe fn register(
    name: &str,
    proc: unsafe extern "system" fn(H, u32, usize, isize) -> isize,
    brush: H,
) {
    let class = wide(name);
    let wc = Wc {
        style: 3,
        proc: Some(proc),
        cls: 0,
        wnd: 0,
        instance: GetModuleHandleW(std::ptr::null()),
        icon: LoadIconW(GetModuleHandleW(std::ptr::null()), 1 as *const u16),
        cursor: LoadCursorW(std::ptr::null_mut(), 32512 as *const u16),
        brush,
        menu: std::ptr::null(),
        name: class.as_ptr(),
    };
    RegisterClassW(&wc);
}
fn main() {
    if std::env::args().any(|a| a == "--smoke-test") {
        std::panic::set_hook(Box::new(|info| {
            let _ = std::fs::write(
                std::env::current_exe()
                    .unwrap()
                    .with_file_name("smoke-test-panic.txt"),
                info.to_string(),
            );
        }));
    }
    if std::env::args().any(|a| a == "--self-test") {
        let n = Note::default();
        let s = encode(&[n.clone()], true);
        assert_eq!(decode(&s), Some((vec![n], true)));
        assert!(decode("bad").is_none());
        let legacy = format!(
            "DesktopInk1\n0\n80\t100\t480\t180\t28\t16777215\t{}\t{}\n",
            hex("Microsoft YaHei UI"),
            hex("旧版中文文字")
        );
        let old = decode(&legacy).unwrap().0.remove(0);
        assert_eq!(old.text, "旧版中文文字");
        assert_eq!(old.opacity, 255);
        assert!(old.clear_background);
        assert!(!old.topmost);
        let mut styled = Note::default();
        styled.background = 0x403020;
        styled.clear_background = false;
        styled.opacity = 26;
        styled.topmost = true;
        assert_eq!(
            decode(&encode(&[styled.clone()], false)),
            Some((vec![styled], false))
        );
        std::fs::write(
            std::env::current_exe()
                .unwrap()
                .with_file_name("self-test-result.txt"),
            "PASS: Unicode configuration round trip and invalid input rejection\n",
        )
        .unwrap();
        return;
    }
    unsafe {
        SetProcessDPIAware();
        let loaded = std::fs::read_to_string(config_path())
            .ok()
            .and_then(|s| decode(&s));
        APP.with(|a| {
            let mut a = a.borrow_mut();
            let (n, locked) = loaded.unwrap_or((vec![Note::default()], false));
            a.notes = n;
            a.locked = locked;
            a.selected = if a.notes.is_empty() { None } else { Some(0) };
        });
        register("DesktopInkManager", manager_proc, 6 as H);
        register("DesktopInkNote", note_proc, std::ptr::null_mut());
        APP.with(|a| {
            a.borrow_mut().ui_font = CreateFontW(
                -16,
                0,
                0,
                0,
                400,
                0,
                0,
                0,
                1,
                0,
                0,
                4,
                0,
                wide("Microsoft YaHei UI").as_ptr(),
            )
        });
        let mut frame = Rect {
            left: 0,
            top: 0,
            right: 794,
            bottom: 680,
        };
        AdjustWindowRectEx(&mut frame, 0x00ca0000, 0, 0);
        let mut area = Rect {
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1080,
        };
        SystemParametersInfoW(0x30, 0, &mut area as *mut _ as *mut c_void, 0);
        let outer_width = frame.right - frame.left;
        let outer_height = frame.bottom - frame.top;
        let origin_x = area.left + ((area.right - area.left - outer_width) / 2).max(0);
        let origin_y = area.top + ((area.bottom - area.top - outer_height) / 2).max(0);
        let h = CreateWindowExW(
            0,
            wide("DesktopInkManager").as_ptr(),
            wide("Desktop Ink · 桌面文字").as_ptr(),
            0x00ca0000,
            origin_x,
            origin_y,
            outer_width,
            outer_height,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            GetModuleHandleW(std::ptr::null()),
            std::ptr::null_mut(),
        );
        if h.is_null() {
            return;
        }
        APP.with(|a| a.borrow_mut().manager = h);
        control(h, "STATIC", "桌面文字", 0, 0, 22, 18, 180, 26);
        control(h, "LISTBOX", "", 101, 0x200001 | 0x100000, 22, 52, 220, 385);
        control(h, "BUTTON", "添加", 201, 0, 22, 450, 104, 34);
        control(h, "BUTTON", "删除", 202, 0, 138, 450, 104, 34);
        control(h, "STATIC", "文字内容（支持多行）", 0, 0, 268, 18, 460, 26);
        control(
            h,
            "EDIT",
            "",
            102,
            0x4 | 0x40 | 0x1000 | 0x100000,
            268,
            52,
            500,
            140,
        );
        control(
            h,
            "STATIC",
            "本机字体（可选择或输入）",
            0,
            0,
            268,
            248,
            310,
            22,
        );
        let fonts = control(
            h,
            "COMBOBOX",
            "",
            103,
            0x2 | 0x100000 | 0x40 | 0x200000,
            268,
            274,
            310,
            300,
        );
        for font in installed_fonts() {
            SendMessageW(fonts, 0x143, 0, wide(&font).as_ptr() as isize);
        }
        control(h, "STATIC", "字号（像素）", 0, 0, 598, 248, 150, 22);
        control(h, "EDIT", "", 104, 0x80, 598, 274, 170, 30);
        control(h, "STATIC", "字体颜色 #RRGGBB", 0, 0, 268, 320, 180, 22);
        control(h, "EDIT", "", 105, 0x80, 268, 346, 160, 30);
        control(h, "STATIC", "宽度", 0, 0, 448, 320, 140, 22);
        control(h, "EDIT", "", 106, 0x80, 448, 346, 140, 30);
        control(h, "STATIC", "高度", 0, 0, 608, 320, 140, 22);
        control(h, "EDIT", "", 107, 0x80, 608, 346, 160, 30);
        control(h, "BUTTON", "", 207, 0xb, 268, 394, 220, 34);
        control(h, "BUTTON", "", 208, 0xb, 508, 394, 260, 34);
        control(h, "EDIT", "", 108, 0x80, 268, 442, 160, 30);
        control(h, "BUTTON", "背景完全透明", 109, 3, 448, 442, 260, 30);
        control(
            h,
            "STATIC",
            "整体透明度（0% 不透明，90% 最透明）",
            0,
            0,
            268,
            486,
            440,
            24,
        );
        let init = InitControls {
            size: std::mem::size_of::<InitControls>() as u32,
            classes: 4,
        };
        InitCommonControlsEx(&init);
        control(h, "msctls_trackbar32", "", 110, 0x10000, 268, 512, 420, 32);
        SendMessageW(GetDlgItem(h, 110), 0x406, 1, (90 << 16) as isize);
        control(h, "STATIC", "0%", 111, 0, 706, 516, 62, 24);
        control(h, "BUTTON", "应用并保存", 203, 1, 268, 558, 160, 38);
        control(h, "BUTTON", "移回主屏", 205, 0, 448, 558, 140, 38);
        control(h, "BUTTON", "锁定 / 点击穿透", 204, 0, 268, 610, 320, 38);
        control(h, "BUTTON", "退出程序", 206, 0, 608, 610, 160, 38);
        control(h, "BUTTON", "开机自启动", 210, 3, 22, 488, 222, 28);
        SendMessageW(GetDlgItem(h, 210), 0xf1, startup_enabled() as usize, 0);
        control(h, "BUTTON", "浮层置顶", 112, 3, 22, 548, 222, 28);
        control(
            h,
            "STATIC",
            "登录后自动隐藏到托盘",
            212,
            0,
            22,
            520,
            222,
            24,
        );
        control(
            h,
            "STATIC",
            "开启：位于普通窗口上方\r\n关闭：可被其他应用覆盖",
            213,
            0,
            22,
            580,
            222,
            40,
        );
        control(h,"STATIC","－ 最小化到任务栏；× 隐藏到托盘，桌面文字保留。\r\n颜色、透明度和置顶设置后点击“应用并保存”。",211,0,22,632,740,44);
        APP.with(|a| {
            a.borrow_mut().taskbar_created = RegisterWindowMessageW(wide("TaskbarCreated").as_ptr())
        });
        add_tray(h);
        let hotkey = RegisterHotKey(h, 1, 0x4000 | 2 | 4, 0x44);
        if hotkey != 0 {
            SetWindowTextW(
                h,
                wide("Desktop Ink · 桌面文字（Ctrl+Shift+D 显示管理窗口）").as_ptr(),
            );
        }
        rebuild();
        if std::env::args().any(|a| a == "--smoke-test") {
            let mut client = Rect::default();
            GetClientRect(h, &mut client);
            for id in [
                101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 201, 202, 203, 204,
                205, 206, 207, 208, 210, 211, 212, 213,
            ] {
                let child = GetDlgItem(h, id);
                assert!(!child.is_null());
                let mut rect = Rect::default();
                GetWindowRect(child, &mut rect);
                let mut end = Point {
                    x: rect.right,
                    y: rect.bottom,
                };
                ScreenToClient(h, &mut end);
                assert!(
                    end.x <= client.right && end.y <= client.bottom,
                    "control {id} exceeds client area"
                );
            }
            for id in [112, 210] {
                let child = GetDlgItem(h, id);
                let dc = GetDC(child);
                let old = SelectObject(dc, SendMessageW(child, 0x31, 0, 0) as H);
                let caption = wide(&text(h, id));
                let mut size = Point::default();
                GetTextExtentPoint32W(dc, caption.as_ptr(), caption.len() as i32 - 1, &mut size);
                let mut bounds = Rect::default();
                GetClientRect(child, &mut bounds);
                assert!(size.x + 28 <= bounds.right, "checkbox {id} text clipped");
                SelectObject(dc, old);
                ReleaseDC(child, dc);
            }
            assert!(!LoadIconW(GetModuleHandleW(std::ptr::null()), 1 as *const u16).is_null());
            let count = SendMessageW(fonts, 0x146, 0, 0);
            assert!(count > 0);
            SendMessageW(fonts, 0x14e, 0, 0);
            let chosen_font = text(h, 103);
            assert!(!chosen_font.is_empty());
            assert!(!GetDlgItem(h, 102).is_null());
            assert_eq!(APP.with(|a| a.borrow().windows.len()), 1);
            SendMessageW(h, 0x111, 201, 0);
            assert_eq!(APP.with(|a| a.borrow().windows.len()), 2);
            set(h, 102, "测试文字\r\nHello Rust");
            set(h, 103, &chosen_font);
            set(h, 104, "36");
            set(h, 105, "#12AB34");
            set(h, 106, "600");
            set(h, 107, "240");
            assert!(!GetDlgItem(h, 110).is_null());
            set(h, 108, "#203040");
            pick_color(h, 108);
            assert_eq!(text(h, 108), "#203040");
            pick_color(h, 105);
            assert_eq!(text(h, 105), "#12AB34");
            SendMessageW(GetDlgItem(h, 109), 0xf1, 0, 0);
            SendMessageW(GetDlgItem(h, 110), 0x405, 1, 40);
            SendMessageW(h, 0x114, 0, GetDlgItem(h, 110) as isize);
            assert_eq!(text(h, 111), "40%");
            SendMessageW(h, 0x111, 203, 0);
            APP.with(|a| {
                let a = a.borrow();
                let n = &a.notes[1];
                assert_eq!(n.text, "测试文字\r\nHello Rust");
                assert_eq!(n.size, 36);
                assert_eq!(n.color, 0x34ab12);
                assert_eq!(n.width, 600);
                assert_eq!(n.font, chosen_font);
                assert_eq!(n.background, 0x403020);
                assert!(!n.clear_background);
                assert_eq!(n.opacity, 153);
            });
            let overlay = APP.with(|a| a.borrow().windows[1]);
            let mut key = 0;
            let mut alpha = 0;
            let mut flags = 0;
            assert_ne!(
                GetLayeredWindowAttributes(overlay, &mut key, &mut alpha, &mut flags),
                0
            );
            assert_eq!(alpha, 153);
            assert_eq!(flags, 2);
            assert_eq!(GetWindowLongPtrW(overlay, -20) & 8, 0);
            SendMessageW(GetDlgItem(h, 112), 0xf1, 1, 0);
            SendMessageW(GetDlgItem(h, 109), 0xf1, 1, 0);
            SendMessageW(GetDlgItem(h, 110), 0x405, 1, 90);
            SendMessageW(h, 0x111, 203, 0);
            let overlay = APP.with(|a| a.borrow().windows[1]);
            GetLayeredWindowAttributes(overlay, &mut key, &mut alpha, &mut flags);
            assert_eq!(alpha, 26);
            assert_eq!(flags, 3);
            assert_ne!(GetWindowLongPtrW(overlay, -20) & 8, 0);
            assert!(!startup_enabled());
            set_startup(true).unwrap();
            assert!(startup_enabled());
            set_startup(false).unwrap();
            assert!(!startup_enabled());
            assert_eq!(
                RegDeleteKeyW(
                    0x80000001u32 as i32 as isize as H,
                    wide(&startup_key()).as_ptr()
                ),
                0
            );
            SendMessageW(h, 0x111, 204, 0);
            let note = APP.with(|a| a.borrow().windows[1]);
            assert_ne!(GetWindowLongPtrW(note, -20) & 0x20, 0);
            SendMessageW(h, 0x111, 204, 0);
            let note = APP.with(|a| a.borrow().windows[1]);
            assert_eq!(GetWindowLongPtrW(note, -20) & 0x20, 0);
            SetWindowPos(note, std::ptr::null_mut(), 320, 280, 0, 0, 0x15);
            SendMessageW(note, 0x232, 0, 0);
            APP.with(|a| {
                let a = a.borrow();
                assert_eq!(a.notes[1].x, 320);
                assert_eq!(a.notes[1].y, 280);
            });
            let restored = decode(&std::fs::read_to_string(config_path()).unwrap()).unwrap();
            APP.with(|a| assert_eq!(restored.0, a.borrow().notes));
            SendMessageW(note, 0x0f, 0, 0);
            SendMessageW(h, 0x111, 202, 0);
            assert_eq!(APP.with(|a| a.borrow().notes.len()), 1);
            SendMessageW(h, 0x111, 202, 0);
            assert_eq!(APP.with(|a| a.borrow().notes.len()), 0);
            SendMessageW(h, 0x111, 201, 0);
            assert_eq!(APP.with(|a| a.borrow().notes.len()), 1);
            ShowWindow(h, 5);
            ShowWindow(h, 6);
            assert_ne!(IsIconic(h), 0);
            assert_ne!(IsWindowVisible(h), 0);
            restore_manager(h);
            SendMessageW(h, 0x10, 0, 0);
            assert_ne!(IsWindow(h), 0);
            assert!(APP.with(|a| a.borrow().tray));
            assert_eq!(IsWindowVisible(h), 0);
            APP.with(|a| {
                for window in &a.borrow().windows {
                    assert_ne!(IsWindowVisible(*window), 0);
                }
            });
            SendMessageW(h, TRAY_MESSAGE, 1, 0x202);
            assert_eq!(IsIconic(h), 0);
            assert_ne!(IsWindowVisible(h), 0);
            SendMessageW(h, 0x10, 0, 0);
            SendMessageW(h, 0x312, 1, 0);
            assert_ne!(IsWindowVisible(h), 0);
            SendMessageW(h, 0x111, 206, 0);
            assert_eq!(IsWindow(h), 0);
            assert!(!APP.with(|a| a.borrow().tray));
            let windows = APP.with(|a| std::mem::take(&mut a.borrow_mut().windows));
            for window in windows {
                DestroyWindow(window);
            }
            let _ = std::fs::remove_file(config_path());
            std::fs::write(std::env::current_exe().unwrap().with_file_name("smoke-test-result.txt"),format!("PASS: native color dialogs open and accept, font/background colors, transparency slider label, layered-window alpha and background mode, 90% transparency save/load, topmost on/off, startup registry enable/read/disable in isolated test key, previous notes/font/tray tests\nInstalled font families: {count}\n")).unwrap();
            return;
        }
        if std::env::args().any(|a| a == "--background") && APP.with(|a| a.borrow().tray) {
            ShowWindow(h, 0);
        } else {
            ShowWindow(h, 5);
        }
        let mut msg: Msg = std::mem::zeroed();
        loop {
            let result = GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0);
            if result <= 0 {
                break;
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
