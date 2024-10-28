use gpui::*;
use std::{ops::DerefMut, time::Duration};

struct SubWindow {}

fn button(text: &str, on_click: impl Fn(&mut WindowContext) + 'static) -> impl IntoElement {
    div()
        .id(SharedString::from(text.to_string()))
        .flex_none()
        .px_4()
        .py_1()
        .bg(rgb(0x09090b))
        .active(|this| this.opacity(0.85))
        .rounded_md()
        .cursor_pointer()
        .text_color(rgb(0xffffff))
        .child(text.to_string())
        .on_click(move |_, cx| on_click(cx))
}

impl Render for SubWindow {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .bg(rgb(0x000000))
            .opacity(0.80)
            .size_full()
            .gap_2()
            .child(
                div()
                    .p_8()
                    .gap_2()
                    .flex()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .text_color(rgb(0xffffff))
                    .text_xl()
                    .child("01:22:34"),
            )
    }
}

struct TimerModel {
    seconds: i32,
    running: bool,
}

struct TimerTick {
    seconds: i32,
}

impl EventEmitter<TimerTick> for TimerModel {}

impl TimerModel {
    fn new() -> Self {
        Self {
            seconds: 0,
            running: false,
        }
    }
}

struct TimerWindow {
    timer: Model<TimerModel>,
    _subscription: gpui::Subscription,
}

impl TimerWindow {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        let timer = cx.new_model(|_cx| TimerModel::new());
        let timer_handle = timer.clone();

        let subscription = cx.subscribe(&timer, |_this, _model, _event: &TimerTick, cx| {
            cx.notify();
        });

        let _ = cx
            .deref_mut()
            .spawn(|mut cx| async move {
                let timer = timer_handle;
                loop {
                    Timer::after(Duration::from_secs(1)).await;
                    let _ = timer.update(&mut cx, |model, cx| {
                        if model.running {
                            model.seconds += 1;
                            cx.emit(TimerTick {
                                seconds: model.seconds,
                            });
                        }
                    });
                }
            })
            .detach();

        Self {
            timer,
            _subscription: subscription,
        }
    }
}

impl Render for TimerWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let timer_data = self.timer.read(cx);

        div()
            .bg(rgb(0x27272a))
            .opacity(0.80)
            .size_full()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .font_family("CommitMono Nerd Font Mono")
            .gap_4()
            .child(
                div()
                    .flex()
                    .justify_center()
                    .items_center()
                    .text_color(rgb(0xffffff))
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .child(format!(
                        "{:02}:{:02}",
                        timer_data.seconds / 60,
                        timer_data.seconds % 60
                    )),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .justify_center()
                    .items_center()
                    .child(button(if timer_data.running { "Pause" } else { "Play" }, {
                        let timer = self.timer.clone();
                        move |cx| {
                            timer.update(cx, |model, cx| {
                                model.running = !model.running;
                                cx.emit(TimerTick {
                                    seconds: model.seconds,
                                });
                            });
                        }
                    }))
                    .child(button("Reset", {
                        let timer = self.timer.clone();
                        move |cx| {
                            timer.update(cx, |model, cx| {
                                model.seconds = 0;
                                model.running = false;
                                cx.emit(TimerTick {
                                    seconds: model.seconds,
                                });
                            });
                        }
                    })),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(250.0), px(200.0)), cx));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                kind: WindowKind::PopUp,
                display_id: None,
                focus: true,
                show: true,
                is_movable: true,
                window_background: WindowBackgroundAppearance::Transparent,
                titlebar: Some(TitlebarOptions {
                    title: Some("Timer".into()),
                    appears_transparent: true,
                    traffic_light_position: Default::default(),
                }),
                ..Default::default()
            },
            |cx| cx.new_view(|cx| TimerWindow::new(cx)),
        )
        .unwrap();
    });
}
