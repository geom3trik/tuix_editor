

use tuix::*;

pub mod inspector;
pub use inspector::*;


#[derive(Debug, PartialEq, Clone)]
pub enum AppEvent {
    Init(Entity),
    SelectWidget(Entity),
    AddWidget(Entity),
    RemoveWidget(Entity),
}

#[derive(Default)]
struct AppWidget {
    selected: Entity,
    canvas: Entity,
}

impl Widget for AppWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        //Element::new().build(&mut context).class("tree");
        TreePanel::new().build(state, entity, |builder| builder.class("tree"));

        self.canvas = Element::new().build(state, entity, |builder| builder.class("canvas"));

        state.style.layout_type.insert(self.canvas, LayoutType::Vertical);

        self.selected = Element::new().build(state, self.canvas, |builder|
            builder
            .set_width(Length::Pixels(100.0))
            .set_height(Length::Pixels(100.0))
            .set_background_color(Color::rgb(50, 50, 160))
            .set_clip_widget(self.canvas)
            .class("item"));

        state.style.layout_type.insert(self.selected, LayoutType::Vertical);

        state.style.main_axis.insert(self.selected, Axis {
            space_before: Units::Stretch(1.0),
            size: Units::Stretch(1.0),
            space_after: Units::Stretch(1.0),
        });

        state.style.cross_axis.insert(self.selected, Axis {
            space_before: Units::Stretch(1.0),
            size: Units::Stretch(1.0),
            space_after: Units::Stretch(1.0),
        });

        state.style.main_axis_align.insert(self.selected, AxisAlign {
            space_before_first: Units::Stretch(1.0),
            space_between: Units::Stretch(1.0),
            space_after_last: Units::Stretch(1.0),
        });

        state.style.cross_axis_align.insert(self.selected, AxisAlign {
            space_before_first: Units::Stretch(1.0),
            space_between: Units::Stretch(1.0),
            space_after_last: Units::Stretch(1.0),
        });

        let first = Element::new().build(state, self.selected, |builder| 
            builder
                .set_background_color(Color::rgb(200, 80, 0))
                .class("item")
            );
        let second = Element::new().build(state, self.selected, |builder|
            builder
                .set_background_color(Color::rgb(200, 200, 0))
                .class("item")
        );
        let third = Element::new().build(state, self.selected, |builder|
            builder
                .set_background_color(Color::rgb(200, 80, 200))
                .class("item")
        );

        Inspector::new(self.selected).build(state, entity, |builder| builder.class("inspector"));

        state.insert_event(Event::new(AppEvent::Init(self.canvas)).target(entity).propagate(Propagation::Fall));

        entity.set_flex_direction(state, FlexDirection::Row)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if self.selected != state.hovered {
                            if state.hovered.is_descendant_of(&state.hierarchy, self.canvas) {
                                
                                state.hovered.set_checked(state, true);
                                self.selected.set_checked(state, false);
                                println!("Selected: {} Hovered: {}", self.selected, state.hovered);
                                self.selected = state.hovered;
                                state.insert_event(Event::new(AppEvent::SelectWidget(state.hovered)).target(entity).propagate(Propagation::Fall));
                            }
                        }
                    }
                }

                _=> {}
            }
        }
    }
}


fn main() {
    let app = Application::new(|state, window|{
        state.add_stylesheet("src/style.css");

        

        AppWidget::default().build(state, window.entity(), |builder| builder.set_flex_grow(1.0));

        window.set_flex_direction(state, FlexDirection::Row);

    });

    app.run();
}