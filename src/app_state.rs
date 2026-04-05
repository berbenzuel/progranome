use std::cell::Cell;
use std::rc::Rc;
use slint::VecModel;
use crate::MetronomeUnit;



pub struct AppState {
    model: Rc<VecModel<MetronomeUnit>>,
    selected_index: Cell<Option<usize>>,
}

impl AppState {
    pub fn new(model: &Rc<VecModel<MetronomeUnit>>) -> Self {
        Self {
            model: model.clone(),
            selected_index: Cell::from(None)
        }
    }

    pub fn push(&mut self, unit: MetronomeUnit) {
        self.model.push(unit)
    }



}