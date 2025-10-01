
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::rc::Rc;

use slint::{Model, ModelRc, SharedString, VecModel};

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() -> Result<(), slint::PlatformError> {

    let app = AppWindow::new()?;

    let app_handle = app.as_weak();
    app.on_steplist_add(move |new_string: SharedString| {
        let app = app_handle.unwrap();

        let prevsteplist = app.get_steplist();
        let _steplist = prevsteplist.as_any().downcast_ref::<VecModel<SharedString>>().unwrap().push(new_string);
    });

    let app_handle = app.as_weak();
    app.on_steplist_empty(move || {
        let app = app_handle.unwrap();

        let prevsteplist = app.get_steplist();
        let steplist = prevsteplist.as_any().downcast_ref::<VecModel<SharedString>>().unwrap();
        steplist.clear();
        steplist.push("0".into());
    });

    let app_handle = app.as_weak();
    app.on_reinitialize(move || {
        let app = app_handle.unwrap();

        let prevsteplist = app.get_steplist();
        let steplist = prevsteplist.as_any().downcast_ref::<VecModel<SharedString>>().unwrap();
        steplist.clear();
        steplist.push("0".into());
        
        let matrix: Rc<VecModel<ModelRc<i32>>> = Rc::new(VecModel::default());
        for _y in 0..app.get_Y() {
            let n = Rc::new(VecModel::default());
            for _x in 0..app.get_X() {
                n.push(0);
            }
            matrix.push(n.into());
        }
        app.set_matrix(matrix.into());
        
    });

    let app_handle = app.as_weak();
    app.on_print_matrix(move || {
        let app = app_handle.unwrap();

        let matrix = app.get_matrix();
        let matrix = matrix.as_any().downcast_ref::<VecModel<ModelRc<i32>>>().unwrap();
        matrix.iter().for_each(|x| {
            x.as_any().downcast_ref::<VecModel<i32>>().unwrap().iter().for_each(|v| print!("{v}, "));
            println!();
        });
        println!();
    });

    app.run()
}
