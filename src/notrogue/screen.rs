use std::{borrow::BorrowMut, cell::RefCell, thread::LocalKey};

use notcurses::{NotcursesError, NotcursesResult};

use crate::{
    resource::get_resource,
    screen::{button::ButtonContainer, ScreenTrait},
};

use super::{util::get_center_xy, NotRogue};

thread_local! {
    pub static GAME: RefCell<Option<NotRogue>> = RefCell::new(None);
}

pub struct NotRogueScreen {
    buttons: ButtonContainer,
}

impl NotRogueScreen {
    pub fn new() -> Self {
        NotRogueScreen {
            buttons: ButtonContainer {
                buttons: Vec::new(),
            },
        }
    }
}

impl ScreenTrait for NotRogueScreen {
    fn on_create(&self) {
        GAME.set(Some(NotRogue::new()));
    }

    fn reset_data(&self) {
        GAME.set(None);
    }

    fn on_render(
        &self,
        nc: &mut notcurses::Notcurses,
        cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        GAME.with_borrow(|game| -> NotcursesResult<()> {
            if let Some(game) = game {
                game.on_render(nc, cli)?;
            }
            Err(NotcursesError::Message(String::from(
                "game is not initialized",
            )))
        })?;
        Ok(())
    }

    fn on_press_key(
        &self,
        _event: &notcurses::Input,
        _nc: &mut notcurses::Notcurses,
        _cli: &mut notcurses::Plane,
    ) -> notcurses::NotcursesResult<()> {
        Ok(())
    }
}
