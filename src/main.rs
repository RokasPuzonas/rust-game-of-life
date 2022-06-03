mod game_of_life;

use std::ffi::CString;

use game_of_life::*;
use raylib::prelude::*;

fn draw_label(d: &mut RaylibDrawHandle, x: f32, y: f32, text: &str) {
	d.gui_label(rrect(x, y, 50., 0.), Some(&CString::new(text).unwrap()));
}

fn to_tile_space(x: i32, y: i32, offset_x: i32, offset_y: i32, tile_size: i32) -> (i32, i32) {
	(
		((x - offset_x) as f32 / tile_size as f32).floor() as i32,
		((y - offset_y) as f32 / tile_size as f32).floor() as i32,
	)
}

fn main() {
	let (mut rl, thread) = raylib::init()
		.size(640, 480)
		.resizable()
		.title("Game Of Life")
		.build();

	rl.set_target_fps(60);

	let mut game = GameOfLife::new();
	let tile_size = 16;
	let edge_thickness = 1.0;
	let mut dead_color = Color::BLACK;
	let mut alive_color = Color::WHITE;
	let mut edge_color = Color::GRAY;
	let overlay_color = Color { r: 0, g: 0, b: 0, a: 180 };

	let mut running = false;
	let mut update_rate = 10; // in times per second
	let mut timer = 0;

	let mut camera_x = 0;
	let mut camera_y = 0;
	let mut last_mouse = (0, 0);

	let mut show_options = true;

	while !rl.window_should_close() {
		let screen_width = rl.get_screen_width();
		let screen_height = rl.get_screen_height();
		let offset_x = camera_x + screen_width/2;
		let offset_y = camera_y + screen_height/2;

		if rl.is_key_pressed(KeyboardKey::KEY_TAB) {
			show_options = !show_options
		}
		if !show_options {
			if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
				running = !running;
			}
			if running {
				if timer == 0 {
					game.tick();
					timer = 60 / update_rate;
				}
				timer -= 1;
			}
			if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && !running {
				game.tick();
			}

			let mx = rl.get_mouse_x();
			let my = rl.get_mouse_y();
			let (tile_x, tile_y) = to_tile_space(mx, my, offset_x, offset_y, tile_size);
			if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
				let (last_tile_x, last_tile_y) = to_tile_space(last_mouse.0, last_mouse.1, offset_x, offset_y, tile_size);
				if !(tile_x == last_tile_x && tile_y == last_tile_y) {
					game.toggle_tile((tile_x, tile_y));
					running = false;
				}
			}
			if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
				game.toggle_tile((tile_x, tile_y));
				running = false;
			}
			if rl.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON) {
				let dx = rl.get_mouse_x() - last_mouse.0;
				let dy = rl.get_mouse_y() - last_mouse.1;
				camera_x += dx;
				camera_y += dy;
			}
			last_mouse = (mx, my);
		}

		let mut d = rl.begin_drawing(&thread);
		d.clear_background(dead_color);
		for tile_dy in 0..screen_height/tile_size {
			for tile_dx in 0..screen_width/tile_size {
				let tile_x = tile_dx - offset_x / tile_size;
				let tile_y = tile_dy - offset_y / tile_size;
				if game.get_tile((tile_x, tile_y)) {
					let x = tile_dx * tile_size + (offset_x % tile_size);
					let y = tile_dy * tile_size + (offset_y % tile_size);
					d.draw_rectangle(x, y, tile_size, tile_size, alive_color);
				}
			}
		}

		for y in (0..screen_height+tile_size).step_by(tile_size as usize) {
			let y = y + offset_y % tile_size;
			d.draw_line_ex(rvec2(0.0, y), rvec2(screen_width, y), edge_thickness, edge_color);
		}
		for x in (0..screen_width+tile_size).step_by(tile_size as usize) {
			let x = x + offset_x % tile_size;
			d.draw_line_ex(rvec2(x, 0.0), rvec2(x, screen_height), edge_thickness, edge_color);
		}

		if show_options {
			d.draw_rectangle(0, 0, screen_width, screen_height, overlay_color);
			if d.gui_window_box(rrect(10., 10., 300., 220.), Some(&CString::new("Options").unwrap())) {
				show_options = false;
			}

			draw_label(&mut d, 15., 45., "Tab - Toggle options");
			draw_label(&mut d, 15., 60., "Enter - Toggle simulation");
			draw_label(&mut d, 15., 75., "Space - Step simulation");
			draw_label(&mut d, 15., 90., "Left mouse click - Toggle cell");
			draw_label(&mut d, 15., 105., "Middle mouse drag - Move camera");

			draw_label(&mut d, 15., 132.5, "Simulation speed");
			update_rate = d.gui_slider(rrect(150., 125., 80., 15.), Some(&CString::new("1 FPS").unwrap()), Some(&CString::new("30 FPS").unwrap()), update_rate as f32, 1., 30.) as i32;

			draw_label(&mut d, 15., 160., "\"Dead\" color");
			dead_color = d.gui_color_picker(rrect(20., 170., 40., 40.), dead_color);

			draw_label(&mut d, 110., 160., "\"Alive\" color");
			alive_color = d.gui_color_picker(rrect(115., 170., 40., 40.), alive_color);

			draw_label(&mut d, 205., 160., "Edge color");
			edge_color = d.gui_color_picker(rrect(210., 170., 40., 40.), edge_color);
		}
	}
}
