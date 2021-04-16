use tuix::Widget;

use tuix::*;


use super::AppEvent;

#[derive(Debug,Clone,PartialEq)]
pub enum InspectorEvent {
    ChangeLayout(LayoutType),
    SetLeft(String),
    SetWidth(String),
    SetRight(String),
    SetTop(String),
    SetHeight(String),
    SetBottom(String),

    SetChildLeft(String),
    SetChildRight(String),
    SetChildTop(String),
    SetChildBottom(String),
    SetChildBetween(String),

    RowAlignLeft,
    RowAlignCenter,
    RowAlignRight,
    RowSpaceAround,
    RowSpaceBetween,
    RowStretch,

    ColumnAlignTop,
    ColumnAlignCenter,
    ColumnAlignBottom,
    ColumnStretch,

    // SelfAlignStart,
    // SelfAlignCenter,
    // SelfAlignEnd,
    // SelfAlignStretch,
}

pub struct Inspector {
    selected: Entity,
}

impl Inspector {
    pub fn new(selected: Entity) -> Self {
        Self {
            selected,
        }
    }
}

impl Widget for Inspector {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        ElementLayout::new(self.selected).build(state, entity, |ctx| ctx);
        
        entity
    }
}

pub fn str_to_units(val: &str) -> Option<Units> {
    if val == "Auto" {
        Some(Units::Auto)
    } else if val.chars().last() == Some('s') {
        if let Ok(v) = val[0..val.len()-1].parse::<f32>() {
            Some(Units::Stretch(v))
        } else {
            None
        }
    } else {
        if let Ok(v) = val.parse::<f32>() {
            Some(Units::Pixels(v))
        } else {
            None
        }
    }
}


#[derive(Default)]
struct ElementLayout {

    selected: Entity,

    // Positioning Type
    self_directed: Entity,
    parent_directed: Entity,
    // Horizontal Axis
    left: Entity,
    width: Entity,
    right: Entity,
    // Vertical Axis
    top: Entity,
    height: Entity,
    bottom: Entity,
    // Align
    child_left: Entity,
    child_right: Entity,
    child_top: Entity,
    child_bottom: Entity,
    child_between: Entity,

    // Main Axis Align Buttons
    row_align_left: Entity,
    row_align_center: Entity,
    row_align_right: Entity,
    row_space_around: Entity,
    row_space_between: Entity,
    row_space_evenly: Entity,
    row_stretch: Entity,


    // Cross Axis Align Buttons
    column_align_top: Entity,
    column_align_bottom: Entity,
    column_align_center: Entity,
    column_stretch: Entity,


}

impl ElementLayout {
    pub fn new(selected: Entity) -> Self {
        Self {
            selected,
            ..Default::default()
        }
    }
}

impl Widget for ElementLayout {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        Label::new("ELEMENT LAYOUT")
            .build(state, entity, |cx| cx.class("title"));
        Label::new("Positioning Type (TODO)")
            .build(state, entity, |cx| cx.class("h1"));
        let list = List::new()
            .build(state, entity, |cx| cx.class("list"));
        CheckButton::new(false)
            .build(state, list, |cx| cx.class("list_button").set_text("Self-Directed"));
        CheckButton::new(true)
            .build(state, list, |cx| cx.class("list_button").set_text("Parent-Directed"));
        
        Label::new("Horizontal Axis")
            .build(state, entity, |cx| cx.class("h1"));
        let row = Row::new()
            .build(state, entity, |cx| cx.class("row"));
        
        let column = Column::new()
            .build(state, row, |cx| cx);
        Label::new("Left")
            .build(state, column, |cx| cx.class("input_label"));
        self.left = Textbox::new("1s")
            .on_change(|val| Event::new(InspectorEvent::SetLeft(val.to_owned())))
            .build(state, column, |cx| cx);

        let column = Column::new()
            .build(state, row, |cx| cx.set_width(Stretch(1.0)));
        Label::new("Width")
            .build(state, column, |cx| cx.class("input_label"));
        self.width = Textbox::new("1s")
            .on_change(|val| Event::new(InspectorEvent::SetWidth(val.to_owned())))
            .build(state, column, |cx| cx);

        let column = Column::new()
            .build(state, row, |cx| cx.set_width(Stretch(1.0)));
        Label::new("Right")
            .build(state, column, |cx| cx.class("input_label"));
        self.right = Textbox::new("1s")
            .on_change(|val| Event::new(InspectorEvent::SetRight(val.to_owned())))
            .build(state, column, |cx| cx);

        Label::new("Vertical Axis").build(state, entity, |cx| cx.class("h1"));
        let row = Row::new().build(state, entity, |cx| cx.set_width(Stretch(1.0)).class("row"));
        
        let column = Column::new()
            .build(state, row, |cx| cx.set_flex_grow(1.0));
        Label::new("Top")
            .build(state, column, |cx| cx.class("input_label"));
        self.top = Textbox::new("1s")
            .on_change(|val| Event::new(InspectorEvent::SetTop(val.to_owned())))
            .build(state, column, |cx| cx);

        let column = Column::new()
            .build(state, row, |cx| cx.set_flex_grow(1.0));
        Label::new("Height")
            .build(state, column, |cx| cx.class("input_label"));
        self.height = Textbox::new("1s")
            .on_change(|val| Event::new(InspectorEvent::SetHeight(val.to_owned())))
            .build(state, column, |cx| cx);

        let column = Column::new()
            .build(state, row, |cx| cx.set_flex_grow(1.0));
        Label::new("Bottom")
            .build(state, column, |cx| cx.class("input_label"));
        self.bottom = Textbox::new("1s")
            .on_change(|val| Event::new(InspectorEvent::SetBottom(val.to_owned())))
            .build(state, column, |cx| cx);



        // // let list = List::new().build(state ,entity, |cx| cx.class("list"));
        // // Button::with_label("Start")
        // //     .on_press(Event::new(InspectorEvent::SelfAlignStart))
        // //     .build(state, list, |cx| cx.class("quick_button"));
        // // Button::with_label("Center")
        // //     .on_press(Event::new(InspectorEvent::SelfAlignCenter))
        // //     .build(state, list, |cx| cx.class("quick_button"));
        // // Button::with_label("End")
        // //     .on_press(Event::new(InspectorEvent::SelfAlignEnd))
        // //     .build(state, list, |cx| cx.class("quick_button"));
        // // Button::with_label("Stretch")
        // //     .on_press(Event::new(InspectorEvent::SelfAlignStretch))
        // //     .build(state, list, |cx| cx.class("quick_button"));

        Label::new("CHILDREN LAYOUT").build(state, entity, |cx| cx.class("title"));
        //let row = HBox::new().build(&mut context).set_flex_grow(1.0);

        // TODO - Replace with dropdown
        Label::new("Layout Type")
            .build(state, entity, |cx| cx.class("h1"));
        let list = List::new()
            .build(state, entity, |cx| cx.class("list"));
        CheckButton::new(false)
            .on_checked(Event::new(InspectorEvent::ChangeLayout(LayoutType::Horizontal)))
            .build(state, list, |cx| cx.class("list_button").set_text("Horizontal"));
        CheckButton::new(true)
            .on_checked(Event::new(InspectorEvent::ChangeLayout(LayoutType::Vertical)))
            .build(state, list, |cx| cx.class("list_button").set_text("Vertical"));

        Label::new("Horizontal Axis Align").build(state, entity, |cx| cx.class("h1"));
        let row = Row::new().build(state, entity, |cx| cx.class("row"));
        
        let column = Column::new().build(state, row, |cx| cx.set_flex_grow(1.0));
        Label::new("Left").build(state, column, |cx| cx.class("input_label"));
        self.child_left = Textbox::new("0")
            .on_change(|val| Event::new(InspectorEvent::SetChildLeft(val.to_owned())))
            .build(state, column, |cx| cx);

        let column = Column::new().build(state, row, |cx| cx.set_flex_grow(1.0));
        Label::new("Between").build(state, column, |cx| cx.class("input_label"));
        self.child_between = Textbox::new("0")
            .on_change(|val| Event::new(InspectorEvent::SetChildBetween(val.to_owned())))
            .build(state, column, |cx| cx);


        let column = Column::new().build(state, row, |cx| cx.set_flex_grow(1.0));
        Label::new("Right").build(state, column, |cx| cx.class("input_label"));
        self.child_right = Textbox::new("0")
            .on_change(|val| Event::new(InspectorEvent::SetChildRight(val.to_owned())))
            .build(state, column, |cx| cx);

        let list = List::new().build(state, entity, |cx| cx.class("list"));
        self.row_align_left = Button::with_label("Left")
            .on_press(Event::new(InspectorEvent::RowAlignLeft))
            .build(state, list, |cx| cx.class("quick_button"));
        self.row_align_center = Button::with_label("Center")
            .on_press(Event::new(InspectorEvent::RowAlignCenter))
            .build(state, list, |cx| cx.class("quick_button"));
        self.row_align_right = Button::with_label("End")
            .on_press(Event::new(InspectorEvent::RowAlignRight))
            .build(state, list, |cx| cx.class("quick_button"));
        let list = List::new().build(state, entity, |cx| cx.class("list"));
        self.row_space_around = Button::with_label("Around")
            .on_press(Event::new(InspectorEvent::RowSpaceAround))
            .build(state, list, |cx| cx.class("quick_button"));
        self.row_space_between = Button::with_label("Between")
            .on_press(Event::new(InspectorEvent::RowSpaceBetween))
            .build(state, list, |cx| cx.class("quick_button"));
        //self.main_space_evenly = Button::with_label("Evenly").build(&mut list).class("quick_button").entity();
        self.row_stretch = Button::with_label("Stretch")
            .on_press(Event::new(InspectorEvent::RowStretch))
            .build(state, list, |cx| cx.class("quick_button"));


        Label::new("Verical Axis Align").build(state, entity, |cx| cx.class("h1"));
        let row = Row::new().build(state, entity, |cx| cx.class("row"));
        
        let column = Column::new().build(state, row, |cx| cx.set_flex_grow(1.0));
        Label::new("Top").build(state, column, |cx| cx.class("label"));
        self.child_top = Textbox::new("0")
            .on_change(|val| Event::new(InspectorEvent::SetChildTop(val.to_owned())))
            .build(state, column, |cx| cx);

        let column = Column::new().build(state, row, |cx| cx.set_flex_grow(1.0));
        Label::new("After").build(state, column, |cx| cx.class("label"));
        self.child_bottom = Textbox::new("0")
            .on_change(|val| Event::new(InspectorEvent::SetChildBottom(val.to_owned())))
            .build(state, column, |cx| cx);

        let list = List::new().build(state ,entity, |cx| cx.class("list"));
        self.column_align_top = Button::with_label("Top")
            .on_press(Event::new(InspectorEvent::ColumnAlignTop))
            .build(state, list, |cx| cx.class("quick_button"));
        self.column_align_center = Button::with_label("Center")
            .on_press(Event::new(InspectorEvent::ColumnAlignCenter))
            .build(state, list, |cx| cx.class("quick_button"));
        self.column_align_bottom = Button::with_label("Bottom")
            .on_press(Event::new(InspectorEvent::ColumnAlignBottom))
            .build(state, list, |cx| cx.class("quick_button"));
        self.column_stretch = Button::with_label("Stretch")
            .on_press(Event::new(InspectorEvent::ColumnStretch))
            .build(state, list, |cx| cx.class("quick_button"));

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {

        if let Some(inspector_event) = event.message.downcast::<InspectorEvent>() {
            match inspector_event {
                InspectorEvent::ChangeLayout(layout_type) => {
                    state.style.layout_type.insert(self.selected, layout_type.clone());
                    event.consume();
                }

                InspectorEvent::SetLeft(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.left.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::SetWidth(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.width.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::SetRight(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.right.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::SetTop(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.top.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::SetHeight(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.height.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::SetBottom(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.bottom.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::SetChildLeft(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.child_left.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::SetChildRight(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.child_right.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::SetChildTop(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.child_top.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::SetChildBottom(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.child_bottom.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::SetChildBetween(value) => {
                    if let Some(units) = str_to_units(value) {
                        state.style.child_between.insert(self.selected, units);
                        event.consume();
                    }
                }

                InspectorEvent::RowAlignLeft => {
                    state.style.child_left.insert(self.selected, Units::Pixels(0.0));
                    state.style.child_between.insert(self.selected, Units::Pixels(0.0));
                    state.style.child_right.insert(self.selected, Units::Stretch(1.0));

                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_left));
                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_between));
                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_right));
                }

                InspectorEvent::RowAlignCenter => {
                    state.style.child_left.insert(self.selected, Units::Stretch(1.0));
                    state.style.child_between.insert(self.selected, Units::Pixels(0.0));
                    state.style.child_right.insert(self.selected, Units::Stretch(1.0));

                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_left));
                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_between));
                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_right));
                }

                InspectorEvent::RowAlignRight => {
                    state.style.child_left.insert(self.selected, Units::Stretch(1.0));
                    state.style.child_between.insert(self.selected, Units::Pixels(0.0));
                    state.style.child_right.insert(self.selected, Units::Pixels(0.0));

                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_left));
                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_between));
                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_right));
                }

                InspectorEvent::RowSpaceAround => {
                    state.style.child_left.insert(self.selected, Units::Stretch(1.0));
                    state.style.child_between.insert(self.selected, Units::Stretch(1.0));
                    state.style.child_right.insert(self.selected, Units::Stretch(1.0));

                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_left));
                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_between));
                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_right));
                }

                InspectorEvent::RowSpaceBetween => {
                    state.style.child_left.insert(self.selected, Units::Pixels(0.0));
                    state.style.child_between.insert(self.selected, Units::Stretch(1.0));
                    state.style.child_right.insert(self.selected, Units::Pixels(0.0));

                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_left));
                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_between));
                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_right));
                }

                InspectorEvent::RowStretch => {
                    state.style.child_left.insert(self.selected, Units::Pixels(0.0));
                    state.style.child_between.insert(self.selected, Units::Pixels(0.0));
                    state.style.child_right.insert(self.selected, Units::Pixels(0.0));

                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_left));
                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_between));
                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_right));
                }

                InspectorEvent::ColumnAlignTop => {
                    state.style.child_top.insert(self.selected, Units::Pixels(0.0));
                    state.style.child_bottom.insert(self.selected, Units::Stretch(1.0));

                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_top));
                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_bottom));
                }

                InspectorEvent::ColumnAlignCenter => {
                    state.style.child_top.insert(self.selected, Units::Stretch(1.0));
                    state.style.child_bottom.insert(self.selected, Units::Stretch(1.0));

                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_top));
                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_bottom));
                }

                InspectorEvent::ColumnAlignBottom => {
                    state.style.child_top.insert(self.selected, Units::Stretch(1.0));
                    state.style.child_bottom.insert(self.selected, Units::Pixels(0.0));

                    state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.child_top));
                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_bottom));
                }

                InspectorEvent::ColumnStretch => {
                    state.style.child_top.insert(self.selected, Units::Pixels(0.0));
                    state.style.child_bottom.insert(self.selected, Units::Pixels(0.0));

                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_top));
                    state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.child_bottom));
                }

                // InspectorEvent::SelfAlignStart => {
                //     state.style.left.insert(self.selected, Units::Pixels(0.0));
                //     state.style.right.insert(self.selected, Units::Stretch(1.0));

                //     state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.left));
                //     state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.right));
                // }

                // InspectorEvent::SelfAlignCenter => {
                //     state.style.left.insert(self.selected, Units::Stretch(1.0));
                //     state.style.right.insert(self.selected, Units::Stretch(1.0));

                //     state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.left));
                //     state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.right));
                // }
                
                // InspectorEvent::SelfAlignEnd => {
                //     state.style.left.insert(self.selected, Units::Stretch(1.0));
                //     state.style.right.insert(self.selected, Units::Pixels(0.0));

                //     state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.left));
                //     state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.right));
                // }

                // InspectorEvent::SelfAlignStretch => {
                //     state.style.left.insert(self.selected, Units::Pixels(0.0));
                //     state.style.right.insert(self.selected, Units::Pixels(0.0));

                //     state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.left));
                //     state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.right));
                // }
            }
        }

        if let Some(app_event) = event.message.downcast::<AppEvent>() {
            match app_event {
                AppEvent::SelectWidget(selected) => {
                    println!("Selected Widget: {}", selected);
                    self.selected = *selected;

                    let left = state.style.left.get(self.selected).cloned().unwrap_or_default();
                    let width = state.style.width.get(self.selected).cloned().unwrap_or_default();
                    let right = state.style.right.get(self.selected).cloned().unwrap_or_default();
                    let top = state.style.top.get(self.selected).cloned().unwrap_or_default();
                    let height = state.style.height.get(self.selected).cloned().unwrap_or_default();
                    let bottom = state.style.bottom.get(self.selected).cloned().unwrap_or_default();

                    state.insert_event(Event::new(TextboxEvent::SetValue(left.to_string())).target(self.left));
                    state.insert_event(Event::new(TextboxEvent::SetValue(width.to_string())).target(self.width));
                    state.insert_event(Event::new(TextboxEvent::SetValue(right.to_string())).target(self.right));

                    state.insert_event(Event::new(TextboxEvent::SetValue(top.to_string())).target(self.top));
                    state.insert_event(Event::new(TextboxEvent::SetValue(height.to_string())).target(self.height));
                    state.insert_event(Event::new(TextboxEvent::SetValue(bottom.to_string())).target(self.bottom));

                }

                _=> {}
            }
        }
        /*
        if let Some(button_event) = event.message.downcast::<ButtonEvent>() {
            match button_event {
                ButtonEvent::Pressed => {
                    if event.target == self.cross_start {
                        if self.selected != Entity::null() {
                            if let Some(cross_axis_align) = state.style.cross_axis_align.get_mut(self.selected) {
                                cross_axis_align.space_before_first = Units::Pixels(0.0);
                                cross_axis_align.space_after_last = Units::Stretch(1.0);
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.cross_space_before_first));
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.cross_space_after_last));
                            }
                        }
                    }

                    if event.target == self.cross_center {
                        if self.selected != Entity::null() {
                            if let Some(cross_axis_align) = state.style.cross_axis_align.get_mut(self.selected) {
                                cross_axis_align.space_before_first = Units::Stretch(1.0);
                                cross_axis_align.space_after_last = Units::Stretch(1.0);
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.cross_space_before_first));
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.cross_space_after_last));
                            }
                        }
                    }

                    if event.target == self.cross_end {
                        if self.selected != Entity::null() {
                            if let Some(cross_axis_align) = state.style.cross_axis_align.get_mut(self.selected) {
                                cross_axis_align.space_before_first = Units::Stretch(1.0);
                                cross_axis_align.space_after_last = Units::Pixels(0.0);
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.cross_space_before_first));
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.cross_space_after_last));
                            }
                        }
                    }

                    if event.target == self.cross_stretch {
                        if self.selected != Entity::null() {
                            if let Some(cross_axis_align) = state.style.cross_axis_align.get_mut(self.selected) {
                                cross_axis_align.space_before_first = Units::Pixels(0.0);
                                cross_axis_align.space_after_last = Units::Pixels(0.0);
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.cross_space_before_first));
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.cross_space_after_last));
                            }
                        }
                    }

                    if event.target == self.main_start {
                        if self.selected != Entity::null() {
                            if let Some(main_axis_align) = state.style.main_axis_align.get_mut(self.selected) {
                                main_axis_align.space_before_first = Units::Pixels(0.0);
                                main_axis_align.space_between = Units::Pixels(0.0);
                                main_axis_align.space_after_last = Units::Stretch(1.0);
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.main_space_before_first));
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.main_space_between));
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.main_space_after_last));
                            }
                        }
                    }

                    if event.target == self.main_end {
                        if self.selected != Entity::null() {
                            if let Some(main_axis_align) = state.style.main_axis_align.get_mut(self.selected) {
                                main_axis_align.space_before_first = Units::Stretch(1.0);
                                main_axis_align.space_between = Units::Pixels(0.0);
                                main_axis_align.space_after_last = Units::Pixels(0.0);
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.main_space_before_first));
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.main_space_between));
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.main_space_after_last));
                            }
                        }
                    }

                    if event.target == self.main_center {
                        if self.selected != Entity::null() {
                            if let Some(main_axis_align) = state.style.main_axis_align.get_mut(self.selected) {
                                main_axis_align.space_before_first = Units::Stretch(1.0);
                                main_axis_align.space_between = Units::Pixels(0.0);
                                main_axis_align.space_after_last = Units::Stretch(1.0);
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.main_space_before_first));
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.main_space_between));
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.main_space_after_last));
                            }
                        }
                    }

                    if event.target == self.main_space_between_button {
                        if self.selected != Entity::null() {
                            if let Some(main_axis_align) = state.style.main_axis_align.get_mut(self.selected) {
                                main_axis_align.space_before_first = Units::Pixels(0.0);
                                main_axis_align.space_between = Units::Stretch(1.0);
                                main_axis_align.space_after_last = Units::Pixels(0.0);
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.main_space_before_first));
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.main_space_between));
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.main_space_after_last));
                            }
                        }
                    }

                    if event.target == self.main_space_around {
                        if self.selected != Entity::null() {
                            if let Some(main_axis_align) = state.style.main_axis_align.get_mut(self.selected) {
                                main_axis_align.space_before_first = Units::Stretch(1.0);
                                main_axis_align.space_between = Units::Stretch(1.0);
                                main_axis_align.space_after_last = Units::Stretch(1.0);
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.main_space_before_first));
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.main_space_between));
                                state.insert_event(Event::new(TextboxEvent::SetValue("1s".to_string())).target(self.main_space_after_last));
                            }
                        }
                    }

                    if event.target == self.main_space_stretch {
                        if self.selected != Entity::null() {
                            if let Some(main_axis_align) = state.style.main_axis_align.get_mut(self.selected) {
                                main_axis_align.space_before_first = Units::Pixels(0.0);
                                main_axis_align.space_between = Units::Pixels(0.0);
                                main_axis_align.space_after_last = Units::Pixels(0.0);
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.main_space_before_first));
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.main_space_between));
                                state.insert_event(Event::new(TextboxEvent::SetValue("0".to_string())).target(self.main_space_after_last));
                            }
                        }
                    }
                }

                _=> {}
            }
        }
        
        
        if let Some(textbox_event) = event.message.downcast::<TextboxEvent>() {
            match textbox_event {
                TextboxEvent::ValueChanged(val) => {

                    // Translate text to units
                    //let new_val = Units::Pixels(0.0);

                    let new_val = if val == "Auto" {
                        Some(Units::Auto)
                    } else if val.chars().last() == Some('s') {
                        if let Ok(v) = val[0..val.len()-1].parse::<f32>() {
                            Some(Units::Stretch(v))
                        } else {
                            None
                        }
                    } else {
                        if let Ok(v) = val.parse::<f32>() {
                            Some(Units::Pixels(v))
                        } else {
                            None
                        }
                    };

                    if let Some(new_val) = new_val {
                        if event.target == self.left {
                            if self.selected != Entity::null() {
                                state.style.left.insert(self.selected, new_val);
                            }
                        }    
                        
                        if event.target == self.width {
                            if self.selected != Entity::null() {
                                state.style.width.insert(self.selected, new_val);
                            }
                        }  

                        if event.target == self.right {
                            if self.selected != Entity::null() {
                                state.style.right.insert(self.selected, new_val);
                            }
                        }  

                        if event.target == self.top {
                            if self.selected != Entity::null() {
                                state.style.top.insert(self.selected, new_val);
                            }
                        }     
                        
                        if event.target == self.height {
                            if self.selected != Entity::null() {
                                state.style.height.insert(self.selected, new_val);
                            }
                        }   

                        if event.target == self.bottom {
                            if self.selected != Entity::null() {
                                state.style.bottom.insert(self.selected, new_val);
                            }
                        }  

                        if event.target == self.main_space_before_first {
                            if self.selected != Entity::null() {
                                state.style.main_before_first.insert(self.selected, new_val);
                            }
                        }    
                        
                        if event.target == self.main_space_between {
                            if self.selected != Entity::null() {
                                state.style.main_between.insert(self.selected, new_val);
                            }
                        }  

                        if event.target == self.main_space_after_last {
                            if self.selected != Entity::null() {
                                state.style.main_after_last.insert(self.selected, new_val);
                            }
                        } 

                        if event.target == self.cross_space_before_first {
                            if self.selected != Entity::null() {
                                state.style.cross_before_first.insert(self.selected, new_val);
                            }
                        }  

                        if event.target == self.cross_space_after_last {
                            if self.selected != Entity::null() {
                                state.style.cross_after_last.insert(self.selected, new_val);
                            }
                        } 

                        
                    } else {
                        println!("Failed To Parse: {}", val);
                    }

                }

                _=> {}
            }
        }
        */
    }
}

pub fn generate_item(state: &mut State, parent: Entity, parent_widget: Entity, level: u32) {

    let hierarchy = state.hierarchy.clone();
    let class_name = "level".to_string() + &level.to_string();
    for entity in parent.child_iter(&hierarchy) {
        
        let child = Panel::new("CHILD").build(state, parent_widget, |builder| builder.class(&class_name));
        let child_entity = child.entity();
        println!("Child: {}", child_entity);
        generate_item(state,entity, child, level + 1);
    }
}


pub struct TreePanel {
    root: Entity,
    root_panel: Entity,
}

impl TreePanel {
    pub fn new() -> Self {
        Self {
            root: Entity::null(),
            root_panel: Entity::null(),
        }
    }
}

impl Widget for TreePanel {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        
        let main = Element::new().build(state, entity, |builder| builder.set_flex_grow(1.0));
        self.root_panel = Panel::new("ROOT").build(state, main, |b| b);
        
        Element::new().build(state, entity, |b| b.set_flex_basis(Units::Pixels(30.0)));

        entity
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast::<AppEvent>() {
            match app_event {

                AppEvent::Init(root) => {
                    if *root != Entity::null() {
                        generate_item(state, *root, self.root_panel, 1);
                        event.consume();
                    }

                }

                AppEvent::SelectWidget(selected) => {
                    
                }

                AppEvent::AddWidget(widget) => {
                    
                }

                AppEvent::RemoveWidget(widget) => {

                }
                
            }
        }
    }
}
