use iced::widget::button::StyleSheet;
use iced::{
    widget::{Button, Column, Container, Row, Space, Text},
    Element, Length, Sandbox, Settings, Theme,
};

// 카운터 상태
#[derive(Default)]
struct Counter {
    value: i32,
}

// 사용자 이벤트(메시지)
#[derive(Clone, Debug)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

struct CustomButtonStyle;

impl StyleSheet for CustomButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            shadow_offset: iced::Vector::new(1.0, 1.0),
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.2, 0.5, 0.8))),
            border_radius: 8.0,
            border_width: 1.0,
            border_color: iced::Color::from_rgb(0.1, 0.4, 0.7),
            text_color: iced::Color::WHITE,
            ..Default::default()
        }
    }
}

// Sandbox 트레이트 구현
impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    // ⚠ 불변 참조(&self) 사용
    fn view(&self) -> Element<Message> {
        let increment_button = Button::new(Text::new("증가 +").size(20).horizontal_alignment(iced::alignment::Horizontal::Center))
            .padding(12)
            .width(Length::Fixed(120.0))
            .on_press(Message::IncrementPressed)
            .style(iced::theme::Button::Custom(Box::new(CustomButtonStyle)));

        let decrement_button = Button::new(Text::new("감소 -").size(20).horizontal_alignment(iced::alignment::Horizontal::Center))
            .padding(12)
            .width(Length::Fixed(120.0))
            .on_press(Message::DecrementPressed)
            .style(iced::theme::Button::Custom(Box::new(CustomButtonStyle)));

        let value_text = Text::new(format!("현재 값: {}", self.value)).size(32).style(iced::Color::from_rgb(0.2, 0.2, 0.7));

        Container::new(
            Column::new()
                .push(value_text)
                .push(Row::new().push(increment_button).push(Space::with_width(Length::Fixed(20.0))).push(decrement_button).align_items(iced::Alignment::Center))
                .spacing(20)
                .align_items(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

fn main() {
    // Iced 애플리케이션 실행
    Counter::run(Settings::default());
}
