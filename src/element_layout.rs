

#[derive(Default)]
struct ElementLayout {
    selected: Entity,
}

impl ElementLayout {
    pub fn new(selected: Element) -> Self {
        Self {
            selected,
        }
    }
}

impl Widget for ElementLayout {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}