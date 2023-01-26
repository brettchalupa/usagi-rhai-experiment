fn tick() {
	if this.tick_count != () {
		this.tick_count += 1;
	} else {
		this.tick_count = 0;
	}

	if this.x == () {
		this.x = 800.0;
	}

	if this.y == () {
		this.y = 120.0;
	}

	if pressed_left() {
		this.x = this.x - 12.0;
	}
	if pressed_right() {
		this.x = this.x + 12.0;
	}
	if pressed_up() {
		this.y = this.y - 12.0;
	}
	if pressed_down() {
		this.y = this.y + 12.0;
	}

	text("HELLO USAGI", 80.0, 40.0, 40.0, WHITE);
	text("HELLO KOTA SLEEEEP!", 80.0, 80.0, 40.0, WHITE);
	circle(this.x, this.y, 80.0, WHITE);
	text("FPS: " + fps().to_string(), screen_w() - 80.0, 20.0, 20.0, WHITE);
}

state.tick()
