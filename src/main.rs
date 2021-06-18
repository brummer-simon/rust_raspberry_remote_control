fn set_all(conn: &mut zbus::Connection, r: u8, g: u8, b: u8, brightness: f32) {
    let _ = conn.call_method(
        Some("org.zbus.BlinkService"),
        "/org/zbus/BlinkService",
        Some("org.zbus.BlinkService1"),
        "SetAll",
        &(r, g, b, brightness),
    ).unwrap();
}

fn set_pixel(conn: &mut zbus::Connection, x: u32, r: u8, g: u8, b: u8, brightness: f32)
{
    let _ = conn.call_method(
        Some("org.zbus.BlinkService"),
        "/org/zbus/BlinkService",
        Some("org.zbus.BlinkService1"),
        "SetPixel",
        &(x, r, g, b, brightness),
    ).unwrap();
}

fn set_brightness(conn: &mut zbus::Connection, brightness: f32)
{
    let _ = conn.call_method(
        Some("org.zbus.BlinkService"),
        "/org/zbus/BlinkService",
        Some("org.zbus.BlinkService1"),
        "SetBrightness",
        &(brightness),
    ).unwrap();
}

fn clear(conn: &mut zbus::Connection) {
    let _ = conn.call_method(
        Some("org.zbus.BlinkService"),
        "/org/zbus/BlinkService",
        Some("org.zbus.BlinkService1"),
        "Clear",
        &(),
    ).unwrap();
}

fn show(conn: &mut zbus::Connection) {
    let _ = conn.call_method(
        Some("org.zbus.BlinkService"),
        "/org/zbus/BlinkService",
        Some("org.zbus.BlinkService1"),
        "Show",
        &(),
    ).unwrap();
}

fn main() {
    let mut conn = zbus::Connection::new_session().unwrap();

    set_all(&mut conn, 127, 127, 127, 1.0);
    set_pixel(&mut conn, 1, 127, 127, 127, 1.0);
    set_brightness(&mut conn, 0.5);
    show(&mut conn);
    clear(&mut conn);
    show(&mut conn);
}
