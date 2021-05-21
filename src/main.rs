

use tuix::*;

pub mod inspector;
pub use inspector::*;

use femtovg::{Canvas, renderer::OpenGl, Path, Paint};

#[derive(Default)]
struct Overlay {
    selected: Entity,
}

impl Widget for Overlay {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        if self.selected != Entity::null() {
            let bounds = state.data.get_bounds(self.selected);

            let mut path = Path::new();

            path.move_to(bounds.x, bounds.y);
            path.line_to(bounds.x + bounds.w, bounds.y);
            path.line_to(bounds.x + bounds.w, bounds.y + bounds.h);
            path.line_to(bounds.x, bounds.y + bounds.h);
            path.line_to(bounds.x, bounds.y);

            let paint = Paint::color(femtovg::Color::white());

            canvas.stroke_path(&mut path, paint);
        }
    }
}


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

    first: Entity,
    second: Entity,
    third: Entity,
}

impl Widget for AppWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        //Element::new().build(&mut context).class("tree");
        TreePanel::new().build(state, entity, |builder| builder.class("tree"));

        // CANVAS
        self.canvas = Element::new().build(state, entity, |builder| 
            builder
                .class("canvas")
                .set_child_space(Stretch(1.0))
        );

        // Overlay

        state.style.layout_type.insert(self.canvas, LayoutType::Horizontal);

        self.selected = Element::new().build(state, self.canvas, |builder|
            builder
            .set_width(Units::Pixels(300.0))
            .set_height(Units::Pixels(300.0))
            .set_background_color(Color::rgb(50, 50, 160))
            .set_clip_widget(self.canvas)
            .class("item"));

        state.style.layout_type.insert(self.selected, LayoutType::Vertical);

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

        self.first = Element::new().build(state, self.selected, |builder| 
            builder
                .set_background_color(Color::rgb(200, 80, 0))
                .set_width(Units::Stretch(1.0))
                .set_height(Units::Stretch(1.0))
                .set_max_width(Units::Pixels(50.0))
                .set_text_justify(Justify::Center)
                .set_color(Color::black())
                .class("item")
            );
        self.second = Element::new().build(state, self.selected, |builder|
            builder
                .set_background_color(Color::rgb(200, 200, 0))
                .set_width(Units::Stretch(1.0))
                .set_height(Units::Stretch(1.0))
                .set_text_justify(Justify::Center)
                .set_color(Color::black())
                //.set_max_width(Units::Pixels(70.0))
                .class("item")
        );

        self.third = Element::new().build(state, self.selected, |builder|
            builder
                .set_background_color(Color::rgb(200, 80, 200))
                .set_width(Units::Stretch(1.0))
                .set_height(Units::Stretch(1.0))
                .set_min_width(Units::Pixels(90.0))
                .set_max_width(Units::Pixels(100.0))
                .set_text_justify(Justify::Center)
                .set_color(Color::black())
                .class("item")
        );

        Inspector::new(self.selected).build(state, entity, |builder| builder.class("inspector"));

        state.insert_event(Event::new(AppEvent::Init(self.canvas)).target(entity).propagate(Propagation::Fall));

        state.focused = entity;

        entity.set_flex_direction(state, FlexDirection::Row).set_layout_type(state, LayoutType::Horizontal)
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {

                WindowEvent::GeometryChanged(_) => {
                    if event.target != self.first || event.target != self.second || event.target != self.third {
                        let first_width = format!("{}",state.data.get_width(self.first) as u32);
                        self.first.set_text(state, &first_width);

                        let second_width = format!("{}",state.data.get_width(self.second) as u32);
                        self.second.set_text(state, &second_width);

                        let third_width = format!("{}",state.data.get_width(self.third) as u32);
                        self.third.set_text(state, &third_width);
                    }
                    
                }

                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        if self.selected != state.hovered {
                            if state.hovered.is_descendant_of(&state.hierarchy, self.canvas) {
                                
                                state.hovered.set_checked(state, true);
                                self.selected.set_checked(state, false);
                                println!("Selected: {} Hovered: {}", self.selected, state.hovered);
                                self.selected = state.hovered;
                                state.focused = entity;
                                state.insert_event(Event::new(AppEvent::SelectWidget(state.hovered)).target(entity).propagate(Propagation::Fall));
                            }
                        }
                    }
                }

                WindowEvent::KeyDown(code, key) => {
                    if event.target == entity {
                        if *key == Some(Key::Enter) {
                            println!("Enter Key Pressed");
                            Element::new().build(state, self.selected, |builder| 
                                builder
                                    .set_width(Units::Pixels(30.0))
                                    .set_height(Units::Pixels(30.0))
                                    .set_background_color(Color::red())
                                    .class("item")
                            );
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

        window.set_flex_direction(state, FlexDirection::Row).set_layout_type(state, LayoutType::Horizontal);

    });

    app.run();
}