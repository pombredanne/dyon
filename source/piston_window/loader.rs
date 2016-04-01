fn main() {
    source := "source/piston_window/snake.rs"
    m := unwrap(load(source))

    settings := call_ret(m, "settings", [])
    data := init_data(settings)
    loader := new_loader(interval: settings.reload_interval)
    set(title: call_ret(m, "title", []))
    loop {
        if !next_event() { break }
        if render() {
            call(m, "render", [settings, data])
        }
        if update() {
            call(m, "update", [data, settings, unwrap(update_dt())])
        }
        event(loader: loader, source: source, settings: settings, module: m)
        key := press_keyboard_key()
        if key == some(settings.reset_key) {
            data = init_data(settings)
        } else if key == some(settings.turn_left) {
            settings.pressing_left = true
        } else if key == some(settings.turn_right) {
            settings.pressing_right = true
        } else if key != none() {
            // println("Pressed " + to_string(unwrap(key)))
        }

        key := release_keyboard_key()
        if key == some(settings.turn_left) {
            settings.pressing_left = false
        } else if key == some(settings.turn_right) {
            settings.pressing_right = false
        }
    }
}

fn init_data(settings) -> {
    data := {
        snake_body: init_snake_body(
            parts: settings.snake_parts,
            size: settings.snake_parts_size
        ),
        snake_angle: 0
    }
    data.next_snake_body := data.snake_body
    return clone(data)
}

fn init_snake_body_parts_size(parts, size) -> {
    body := []
    end := [(parts - 1) * size, (parts - 1) * size]
    for i := 0; i < parts; i += 1 {
        push(body, [end[0] - i * size, end[1] - i * size])
    }
    return clone(body)
}

fn new_loader_interval(interval) -> {
    return {
        time: 0,
        last_reload: 0,
        reload_interval: clone(interval),
        got_error: false
    }
}

fn should_reload(loader) -> {
    return !loader.got_error
        && ((loader.last_reload + loader.reload_interval) < loader.time)
}

fn event_loader_source_settings_module(loader, source, settings, m) {
    if update() {
        dt := unwrap(update_dt())
        loader.time += dt
        if should_reload(loader) {
            loader.last_reload = loader.time
            new_m := load(source)
            if is_err(new_m) {
                loader.got_error = true
                println(unwrap_err(new_m))
                println(" ~~~ Hit F1 to reload ~~~ ")
            } else {
                loader.got_error = false
                m = unwrap(new_m)
                settings = call_ret(m, "settings", [])
                loader.reload_interval = clone(settings.reload_interval)
            }
        }
    }
    if press() {
        key := press_keyboard_key()
        if key == some(settings.reload_key) {
            println(" ~~~ Reloading ~~~ ")
            loader.got_error = false
        }
    }
}
