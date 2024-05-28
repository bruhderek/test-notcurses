use std::cell::RefCell;

use notcurses::{Key, MiceEvents, Notcurses, NotcursesResult, Plane};

use crate::{
    resource::add_resources,
    screen::{self, r#impl::startscreen::StartScreen, Screen},
};

thread_local! {
    pub static SCREENS: Vec<screen::Screen> = vec![
        Screen::new(10, 10, Box::new(StartScreen::new()))
    ];

    pub static CURRENT_SCREEN: RefCell<usize> = const { RefCell::new(0) };
}

pub fn initialize_game(nc: &mut Notcurses, cli: &mut Plane) -> NotcursesResult<()> {
    cli.erase();
    add_resources();
    nc.mice_enable(MiceEvents::Button)?;

    Ok(())
}

pub fn start_game_loop(nc: &mut Notcurses, cli: &mut Plane) -> NotcursesResult<()> {
    initialize_game(nc, cli)?;

    loop {
        let event = nc.poll_event()?;
        if event.is_press() {
            SCREENS.with(|screens| {
                screens[CURRENT_SCREEN.with_borrow(|v| *v)]
                    .methods
                    .on_press_key(&event, nc, cli)
                    .expect("press event failed");
            });
        }
        if event.is_key(Key::End) && event.is_press() {
            break;
        }
        cli.erase();
        SCREENS.with(|screens| {
            screens[CURRENT_SCREEN.with_borrow(|v| *v)]
                .methods
                .on_render(nc, cli)
                .expect("render failed");
        });
    }
    Ok(())
}
