use crate::helpers::center;
use esp_idf_svc::hal::delay;
use esp_idf_svc::hal::gpio::{Gpio0, Input, PinDriver};
use esp_idf_svc::hal::task::notification::Notification;
use mousefood::prelude::*;
use mousefood::ratatui::widgets::RatatuiLogo;
use std::io::Result;
use std::marker::PhantomData;

pub struct RatatuiLogoApp<B: Backend> {
    _marker: PhantomData<B>,
}

impl<B: Backend> RatatuiLogoApp<B> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub fn run(
        mut self,
        terminal: &mut Terminal<B>,
        notification: &mut Notification,
        button: &mut PinDriver<Gpio0, Input>,
    ) -> Result<()> {
        button.enable_interrupt().unwrap();
        loop {
            if notification.wait(delay::NON_BLOCK).is_some() {
                return Ok(());
            }
            terminal.draw(|frame| self.draw(frame))?;
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [top_area, footer_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(frame.area());
        let logo_area = center(top_area, Constraint::Length(27), Constraint::Length(2));
        let footer = Line::raw("[S1] to change screen").centered().gray();
        frame.render_widget(RatatuiLogo::small(), logo_area);
        frame.render_widget(footer, footer_area);
    }
}
