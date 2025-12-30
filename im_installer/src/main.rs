use vizia::prelude::*;

use crate::pages::select_format_page::SelectFormatPage;

mod pages;

#[derive(Lens)]
pub struct AppData {
    install_vst3: bool,
    install_clap: bool,
    current_page: TabPage,
}

pub enum AppEvent {
    PrevPage,
    NextPage,
}

pub enum TabPage {
    SelectFormat,
    SelectPath,
    Installing,
    Done,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| match app_event {
            AppEvent::PrevPage => match self.current_page {
                TabPage::SelectFormat => self.current_page = TabPage::SelectFormat,
                TabPage::SelectPath => self.current_page = TabPage::SelectFormat,
                TabPage::Installing => self.current_page = TabPage::SelectPath,
                TabPage::Done => self.current_page = TabPage::Installing,
            },
            AppEvent::NextPage => match self.current_page {
                TabPage::SelectFormat => self.current_page = TabPage::SelectPath,
                TabPage::SelectPath => self.current_page = TabPage::Installing,
                TabPage::Installing => self.current_page = TabPage::Done,
                TabPage::Done => self.current_page = TabPage::Done,
            },
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("src/style.css"))
            .expect("unable to load style.css");

        cx.add_font_mem(include_bytes!("../../assets/JetBrainsMono-Bold.ttf"));

        AppData {
            install_vst3: false,
            install_clap: false,
            current_page: TabPage::SelectFormat,
        }
        .build(cx);

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "IM_DISPERSER").class("title");
                Label::new(cx, "一个一个一个 Disperser 插件").class("subtitle");

                SelectFormatPage::new(cx);
            })
            .width(Stretch(1.0));

            VStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "下一步")).class("next-btn");
            })
            .alignment(Alignment::BottomRight)
            .width(Stretch(1.0));
        })
        .class("main-stack");
    })
    .inner_size((800, 333))
    .title("IM_DISPERSER INSTALLER")
    .anchor_target(AnchorTarget::Monitor)
    .parent_anchor(Anchor::Center)
    .enabled_window_buttons(WindowButtons::empty())
    .enabled_window_buttons(WindowButtons::MINIMIZE)
    .enabled_window_buttons(WindowButtons::CLOSE)
    .run()
}
