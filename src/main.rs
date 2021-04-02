

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
    fn on_build(&mut self, mut context: Context) -> Self::Ret {

        //Element::new().build(&mut context).class("tree");
        TreePanel::new().build(&mut context).class("tree");

        let mut canvas = Element::new().build(&mut context).class("canvas");
        let canvas_entity = canvas.entity();
        self.canvas = canvas_entity;

        canvas.state().style.layout_type.insert(canvas_entity, LayoutType::Vertical);
        let canvas_entity = canvas.entity();

        let mut selected = Element::new().build(&mut canvas)
            .set_width(Length::Pixels(100.0))
            .set_height(Length::Pixels(100.0))
            .set_background_color(Color::rgb(50, 50, 160))
            .set_clip_widget(canvas_entity)
            .class("item");
        self.selected = selected.entity();

        selected.state().style.layout_type.insert(self.selected, LayoutType::Vertical);

        selected.state().style.main_axis.insert(self.selected, Axis {
            space_before: Units::Stretch(1.0),
            size: Units::Stretch(1.0),
            space_after: Units::Stretch(1.0),
        });

        selected.state().style.cross_axis.insert(self.selected, Axis {
            space_before: Units::Stretch(1.0),
            size: Units::Stretch(1.0),
            space_after: Units::Stretch(1.0),
        });

        selected.state().style.main_axis_align.insert(self.selected, AxisAlign {
            space_before_first: Units::Stretch(1.0),
            space_between: Units::Stretch(1.0),
            space_after_last: Units::Stretch(1.0),
        });

        selected.state().style.cross_axis_align.insert(self.selected, AxisAlign {
            space_before_first: Units::Stretch(1.0),
            space_between: Units::Stretch(1.0),
            space_after_last: Units::Stretch(1.0),
        });

        let first = Element::new().build(&mut selected).set_background_color(Color::rgb(200, 80, 0)).class("item");
        let first_entity = first.entity();
        let second = Element::new().build(&mut selected).set_background_color(Color::rgb(200, 200, 0)).class("item");
        let second_entity = second.entity();
        let third = Element::new().build(&mut selected).set_background_color(Color::rgb(200, 80, 200)).class("item");
        let third_entity = third.entity();

        Inspector::new(self.selected).build(&mut context).class("inspector");

        let context_entity = context.entity();
        context.state().insert_event(Event::new(AppEvent::Init(canvas_entity)).target(context_entity).propagate(Propagation::Fall));

        context.set_flex_direction(FlexDirection::Row).entity()
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
    let app = Application::new(|mut context, window|{
        context.state().add_stylesheet("src/style.css");

        

        AppWidget::default().build(&mut context).set_flex_grow(1.0);

        context.set_flex_direction(FlexDirection::Row);

    });

    app.run();
}