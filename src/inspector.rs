use tuix::Widget;

use tuix::*;


use super::AppEvent;
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
    fn on_build(&mut self, mut context: Context) -> Self::Ret {

        ElementLayout::new(self.selected).build(&mut context);
        
        context.entity()
    }
}


#[derive(Default)]
struct ElementLayout {

    selected: Entity,

    // Positioning Type
    self_directed: Entity,
    parent_directed: Entity,
    // Main Axis
    main_space_before: Entity,
    main_size: Entity,
    main_space_after: Entity,
    // Cross Axis
    cross_space_before: Entity,
    cross_size: Entity,
    cross_space_after: Entity,
    // Main Axis Align
    main_space_before_first: Entity,
    main_space_between: Entity,
    main_space_after_last: Entity,

    // Main Axis Align Buttons
    main_start: Entity,
    main_center: Entity,
    main_end: Entity,
    main_space_around: Entity,
    main_space_between_button: Entity,
    main_space_evenly: Entity,
    main_space_stretch: Entity,

    // Cross Axis Align
    cross_space_before_first: Entity,
    cross_space_after_last: Entity,

    // Cross Axis Align Buttons
    cross_start: Entity,
    cross_center: Entity,
    cross_end: Entity,
    cross_stretch: Entity,


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
    fn on_build(&mut self, mut context: Context) -> Self::Ret {
        
        Label::new("ELEMENT LAYOUT").build(&mut context).class("title");
        Label::new("Positioning Type (TODO)").build(&mut context).class("h1");
        let mut list = List::new().build(&mut context).class("list");
        CheckButton::new(false).build(&mut list).class("list_button").set_text("Self-Directed");
        CheckButton::new(true).build(&mut list).class("list_button").set_text("Parent-Directed");
        
        Label::new("Main Axis").build(&mut context).class("h1");
        let mut row = HBox::new().build(&mut context).class("row");
        
        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("Before").build(&mut column).class("input_label");
        self.main_space_before = Textbox::new("1s").build(&mut column).entity();

        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("Size").build(&mut column).class("input_label");
        self.main_size = Textbox::new("1s").build(&mut column).entity();


        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("After").build(&mut column).class("input_label");
        self.main_space_after = Textbox::new("1s").build(&mut column).entity();

        Label::new("Cross Axis").build(&mut context).class("h1");
        let mut row = HBox::new().build(&mut context).class("row");
        
        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("Before").build(&mut column).class("input_label");
        self.cross_space_before = Textbox::new("1s").build(&mut column).entity();

        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("Size").build(&mut column).class("input_label");
        self.cross_size = Textbox::new("1s").build(&mut column).entity();


        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("After").build(&mut column).class("input_label");
        self.cross_space_after = Textbox::new("1s").build(&mut column).entity();

        Label::new("CHILDREN LAYOUT").build(&mut context).class("title");
        //let row = HBox::new().build(&mut context).set_flex_grow(1.0);


        Label::new("Main Axis Align").build(&mut context).class("h1");
        let mut row = HBox::new().build(&mut context).class("row");
        
        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("Before First").build(&mut column).class("input_label");
        self.main_space_before_first = Textbox::new("1s").build(&mut column).entity();

        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("Between").build(&mut column).class("input_label");
        self.main_space_between = Textbox::new("1s").build(&mut column).entity();


        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("After Last").build(&mut column).class("input_label");
        self.main_space_after_last = Textbox::new("1s").build(&mut column).entity();

        let mut list = List::new().build(&mut context).class("list");
        self.main_start = Button::with_label("Start").build(&mut list).class("quick_button").entity();
        self.main_center = Button::with_label("Center").build(&mut list).class("quick_button").entity();
        self.main_end = Button::with_label("End").build(&mut list).class("quick_button").entity();
        let mut list = List::new().build(&mut context).class("list");
        self.main_space_around = Button::with_label("Around").build(&mut list).class("quick_button").entity();
        self.main_space_between_button = Button::with_label("Between").build(&mut list).class("quick_button").entity();
        //self.main_space_evenly = Button::with_label("Evenly").build(&mut list).class("quick_button").entity();
        self.main_space_stretch = Button::with_label("Stretch").build(&mut list).class("quick_button").entity();


        Label::new("Cross Axis Align").build(&mut context).class("h1");
        let mut row = HBox::new().build(&mut context).class("row");
        
        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("Before").build(&mut column).class("label");
        self.cross_space_before_first = Textbox::new("1s").build(&mut column).entity();

        let mut column = VBox::new().build(&mut row).set_flex_grow(1.0);
        Label::new("After").build(&mut column).class("label");
        self.cross_space_after_last = Textbox::new("1s").build(&mut column).entity();

        let mut list = List::new().build(&mut context).class("list");
        self.cross_start = Button::with_label("Start").build(&mut list).class("quick_button").entity();
        self.cross_center = Button::with_label("Center").build(&mut list).class("quick_button").entity();
        self.cross_end = Button::with_label("End").build(&mut list).class("quick_button").entity();
        self.cross_stretch = Button::with_label("Stretch").build(&mut list).class("quick_button").entity();

        context.entity()
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {

        if let Some(app_event) = event.message.downcast::<AppEvent>() {
            match app_event {
                AppEvent::SelectWidget(selected) => {
                    println!("Selected Widget: {}", selected);
                    self.selected = *selected;

                    let main_axis = state.style.main_axis.get(self.selected).cloned().unwrap_or_default();
                    let cross_axis= state.style.cross_axis.get(self.selected).cloned().unwrap_or_default();

                    state.insert_event(Event::new(TextboxEvent::SetValue(main_axis.space_before.to_string())).target(self.main_space_before));
                    state.insert_event(Event::new(TextboxEvent::SetValue(main_axis.size.to_string())).target(self.main_size));
                    state.insert_event(Event::new(TextboxEvent::SetValue(main_axis.space_after.to_string())).target(self.main_space_after));

                    state.insert_event(Event::new(TextboxEvent::SetValue(cross_axis.space_before.to_string())).target(self.cross_space_before));
                    state.insert_event(Event::new(TextboxEvent::SetValue(cross_axis.size.to_string())).target(self.cross_size));
                    state.insert_event(Event::new(TextboxEvent::SetValue(cross_axis.space_after.to_string())).target(self.cross_space_after));

                }

                _=> {}
            }
        }
        
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

                    let new_val = if val == "Inherit" {
                        Some(Units::Inherit)
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
                        if event.target == self.main_space_before {
                            if self.selected != Entity::null() {
                                if let Some(main_axis) = state.style.main_axis.get_mut(self.selected) {
                                    main_axis.space_before = new_val;
                                    
                                    return;
                                } else {
                                    state.style.main_axis.insert(self.selected, Axis {
                                        space_before: new_val,
                                        size: Default::default(),
                                        space_after: Units::Inherit,
                                    });
                                    return;
                                }
                            }
                        }    
                        
                        if event.target == self.main_size {
                            if self.selected != Entity::null() {
                                if let Some(main_axis) = state.style.main_axis.get_mut(self.selected) {
                                    main_axis.size = new_val;
                                    return;
                                } else {
                                    state.style.main_axis.insert(self.selected, Axis {
                                        space_before: Units::Inherit,
                                        size: new_val,
                                        space_after: Units::Inherit,
                                    });
                                    return;
                                }
                            }
                        }  

                        if event.target == self.main_space_after {
                            if self.selected != Entity::null() {
                                if let Some(main_axis) = state.style.main_axis.get_mut(self.selected) {
                                    main_axis.space_after = new_val;
                                    return;
                                } else {
                                    state.style.main_axis.insert(self.selected, Axis {
                                        space_before: Units::Inherit,
                                        size: Default::default(),
                                        space_after: new_val,
                                    });
                                    return;
                                }
                            }
                        }  

                        if event.target == self.cross_space_before {
                            if self.selected != Entity::null() {
                                if let Some(cross_axis) = state.style.cross_axis.get_mut(self.selected) {
                                    cross_axis.space_before = new_val;
                                    return;
                                } else {
                                    state.style.cross_axis.insert(self.selected, Axis {
                                        space_before: new_val,
                                        size: Default::default(),
                                        space_after: Units::Inherit,
                                    });
                                    return;
                                }
                            }
                        }    
                        
                        if event.target == self.cross_size {
                            if self.selected != Entity::null() {
                                if let Some(cross_axis) = state.style.cross_axis.get_mut(self.selected) {
                                    cross_axis.size = new_val;
                                    return;
                                } else {
                                    state.style.cross_axis.insert(self.selected, Axis {
                                        space_before: Units::Inherit,
                                        size: new_val,
                                        space_after: Units::Inherit,
                                    });
                                    return;
                                }
                            }
                        }  

                        if event.target == self.cross_space_after {
                            if self.selected != Entity::null() {
                                if let Some(cross_axis) = state.style.cross_axis.get_mut(self.selected) {
                                    cross_axis.space_after = new_val;
                                    return;
                                } else {
                                    state.style.cross_axis.insert(self.selected, Axis {
                                        space_before: Units::Inherit,
                                        size: Default::default(),
                                        space_after: new_val,
                                    });
                                    return;
                                }
                            }
                        } 

                        if event.target == self.main_space_before_first {
                            if self.selected != Entity::null() {
                                if let Some(main_axis_align) = state.style.main_axis_align.get_mut(self.selected) {
                                    main_axis_align.space_before_first = new_val;
                                    println!("Do This");
                                    return;
                                }
                            }
                        }    
                        
                        if event.target == self.main_space_between {
                            if self.selected != Entity::null() {
                                if let Some(main_axis_align) = state.style.main_axis_align.get_mut(self.selected) {
                                    main_axis_align.space_between = new_val;
                                    return;
                                }
                            }
                        }  

                        if event.target == self.main_space_after_last {
                            if self.selected != Entity::null() {
                                if let Some(main_axis_align) = state.style.main_axis_align.get_mut(self.selected) {
                                    main_axis_align.space_after_last = new_val;
                                    return;
                                }
                            }
                        } 

                        if event.target == self.cross_space_before_first {
                            if self.selected != Entity::null() {
                                if let Some(cross_axis_align) = state.style.cross_axis_align.get_mut(self.selected) {
                                    cross_axis_align.space_before_first = new_val;
                                    return;
                                }
                            }
                        }  

                        if event.target == self.cross_space_after_last {
                            if self.selected != Entity::null() {
                                if let Some(cross_axis_align) = state.style.cross_axis_align.get_mut(self.selected) {
                                    cross_axis_align.space_after_last = new_val;
                                    return;
                                }
                            }
                        } 

                        
                    } else {
                        println!("Failed To Parse: {}", val);
                    }

                }

                _=> {}
            }
        }
    }
}

pub fn generate_item(parent: Entity, mut context: Context<Entity>, level: u32) {
    println!("context: {}", context.entity());
    let hierarchy = context.state().hierarchy.clone();
    let class_name = "level".to_string() + &level.to_string();
    for entity in parent.child_iter(&hierarchy) {
        
        let mut child = Panel::new("CHILD").build(&mut context).class(&class_name);
        let child_entity = child.entity();
        println!("Child: {}", child_entity);
        generate_item(entity, child, level + 1);
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
    fn on_build(&mut self, mut context: Context) -> Self::Ret {
        
        let mut main = Element::new().build(&mut context).set_flex_grow(1.0);
        let mut root = Panel::new("ROOT").build(&mut main);
        self.root_panel = root.data;
        println!("root_panel: {}", self.root_panel);
        //let parent = root.entity();
        //let state = root.state();
        
        

        Element::new().build(&mut context).set_flex_basis(Length::Pixels(30.0));

        
        
        context.entity()
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(app_event) = event.message.downcast::<AppEvent>() {
            match app_event {

                AppEvent::Init(root) => {
                    if *root != Entity::null() {
                        let root_panel = Context {
                            data: self.root_panel,
                            state,
                            entity: self.root_panel,
                        };
                        generate_item(*root, root_panel, 1);
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
