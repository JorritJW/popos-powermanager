// SPDX-License-Identifier: GPL-3.0-only

use cosmic::app::{Command, Core};
use cosmic::iced::wayland::popup::{destroy_popup, get_popup};
use cosmic::iced::window::Id;
use cosmic::iced::Limits;
use cosmic::iced_style::application;
use cosmic::widget::{text, Column};
use cosmic::{Application, Element, Theme};
use sysinfo::System;





/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
#[derive(Default)]
pub struct PowerManager {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// The popup id.
    popup: Option<Id>,
    /// Example row toggler.
    cpu_usages: Vec<f32>,
    system: System,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    Tick,
}

impl PowerManager {
    fn calculate_avg_cpu_usage(&self) -> f32 {
        let total_cpus = self.cpu_usages.len();
        if total_cpus == 0 {
            return 0.0;
        }

        let total_usage: f32 = self.cpu_usages.iter().sum();

        total_usage / total_cpus as f32
    }
}

impl Application for PowerManager {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.example.PowerManager";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = PowerManager {
            core,
            ..Default::default()
        };

        (app, Command::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    // fn view(&self) -> Element<Self::Message> {
    //     self.core
    //         .applet
    //         .icon_button("display-symbolic")
    //         .on_press(Message::TogglePopup)
    //         .into()

    // }
    //     fn view_window(&self, _id: Id) -> Element<Self::Message> {

    fn view(&self) -> Element<Self::Message> {
        let aggregate_cpu_usage = self.calculate_avg_cpu_usage();

        let aggregate_cpu_usage_display: Element<Message> =
            text(format!("Aggregate CPU usage: {:.2}%", aggregate_cpu_usage))
                .size(20)
                .into();

        let cpu_usage_display: Vec<Element<Message>> = self
            .system
            .cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| {
                text(format!("CPU {}: {:.2}%", i + 1, cpu.cpu_usage()))
                    .size(20)
                    .into()
            })
            .collect();
        // let content_list = widget::list_column()
        //     .padding(5)
        //     .spacing(0)
        //     .add(settings::item(
        //         fl!("example-row"),
        //         widget::toggler(None, self.example_row, |value| ll{
        //             Message::ToggleExampleRow(value)
        //         }),
        //     ));
        //let content = Column::with_children(cpu_usage_display).spacing(10).padding(20);
        let content = Column::new()
            .push(aggregate_cpu_usage_display)
            // .push(Column::with_children(cpu_usage_display).spacing(10))
            .spacing(10)
            .padding(20);

        self.core.applet.popup_container(content).into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings =
                        self.core
                            .applet
                            .get_popup_settings(Id::MAIN, new_id, None, None, None);
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::Tick => {
                self.system.refresh_cpu_all();
                self.cpu_usages = self
                    .system
                    .cpus()
                    .iter()
                    .map(|cpu| cpu.cpu_usage())
                    .collect();
            }
        }
        Command::none()
    }

    fn style(&self) -> Option<<Theme as application::StyleSheet>::Style> {
        Some(cosmic::applet::style())
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        cosmic::iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }
}
