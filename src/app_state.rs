use std::cell::Cell;
use std::fmt::Error;
use std::io::ErrorKind::{InvalidData, NotFound};
use std::rc::Rc;

use slint::{Model, VecModel};
use slint::platform::SetPlatformError;
use crate::MetronomeUnit;



pub struct AppState {
    model: Rc<VecModel<MetronomeUnit>>,
    selected_index: Cell<Option<usize>>,
}

impl AppState {
    pub fn new(model: Rc<VecModel<MetronomeUnit>>) -> Self {
        Self {
            model,
            selected_index: Cell::from(None),
        }
    }

    pub fn push(&self, unit: MetronomeUnit) {
        self.model.push(unit)
    }

    pub fn model(&self) -> Rc<VecModel<MetronomeUnit>> { self.model.clone() }
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index.get()
    }
    pub fn set_index(&self, index: usize) {
        self.selected_index.set(Some(index));
    }

    pub fn selected_unit(&self) -> Option<MetronomeUnit> {
        if let Some(index) = self.selected_index.get()
            && let Some(unit) = self.model.row_data(index) {
            return Some(unit)
        }
        None
    }

    // pub fn next(&self) -> Option<usize> {
    //     let mut index = self.selected_index.get();
    //     match index {
    //         Some( i) => {
    //             if let  Some( mut active_unit) = self.model.row_data(i) {
    //                 active_unit.set_active(false);
    //             }
    //             index = Some(i + 1);
    //         }
    //         None => {
    //             if self.model.row_count() == 0 {
    //                 return None;
    //             }
    //             index = Some(0)
    //         }
    //     }
    //     if let  Some( mut active_unit) = self.model.row_data(index.unwrap()) {
    //         active_unit.set_active(true);
    //     }
    //     self.selected_index.set(index);
    //     index
    // }

    //todo! make this cleaner, at least errors
    //todo! custom errors -> handling ui ?
    //
    pub fn select_unit(&self, index: usize) -> Result<(),  Box<dyn std::error::Error>> {

        match self.model.row_data(index) {
            Some(mut unit) => {
                unit.set_active(true);
                self.model.set_row_data(index, unit.clone());
            }
            None => return Err(Box::from(Error::default()))
        }

        //sets active unit to inactive
        let old_index = self.selected_index.get();
        if let Some(i) = old_index
        && let  Some( mut active_unit) = self.model.row_data(i)
        && index != i {
            active_unit.set_active(false);
            self.model.set_row_data(i, active_unit)
        }

        self.set_index(index);

        Ok(())
    }



}