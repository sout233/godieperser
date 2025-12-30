// use crate::pages::select_format_page::TabPage;

use vizia::prelude::*;

#[derive(Lens)]
pub(crate) struct SelectFormatPage {
    install_vst3: bool,
    install_clap: bool,
    // current_page: TabPage,
}

pub(crate) enum SelectFormartEvent {
    ToggleInstallVst3,
    ToggleInstallClap,
}

impl SelectFormatPage {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        SelectFormatPage {
            install_vst3: false,
            install_clap: false,
            // current_page: TabPage::SelectFormat,
        }
        .build(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "选择要安装的格式：").class("p");

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, Self::install_vst3)
                        .on_toggle(|ex| {
                            ex.emit(SelectFormartEvent::ToggleInstallVst3);
                        })
                        .class("checkbox");
                    Label::new(cx, "VST3");
                })
                .class("checkbox-stack");

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, Self::install_clap)
                        .on_toggle(|ex| {
                            ex.emit(SelectFormartEvent::ToggleInstallClap);
                        })
                        .class("checkbox");
                    Label::new(cx, "CLAP");
                })
                .class("checkbox-stack");
            })
            .class("opt-panel");
        })
    }
}

impl View for SelectFormatPage {
    fn element(&self) -> Option<&'static str> {
        Some("select-format-page")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| match app_event {
            SelectFormartEvent::ToggleInstallVst3 => self.install_vst3 = !self.install_vst3,
            SelectFormartEvent::ToggleInstallClap => self.install_clap = !self.install_clap,
        });
    }
}
