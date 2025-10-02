
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fraction;

use std::rc::Rc;
use num::pow;
use slint::{Model, ModelRc, SharedString, ToSharedString, VecModel};

use crate::fraction::Fraction;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() -> Result<(), slint::PlatformError> {

    let app = AppWindow::new()?;

    let app_handle = app.as_weak();
    app.on_steplist_add(move |new_string: SharedString| {
        let app = app_handle.unwrap();

        let prevsteplist = app.get_steplist();
        prevsteplist.as_any().downcast_ref::<VecModel<SharedString>>().unwrap().push(new_string.clone());
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
        
        if !app.get_matrix_lock() {
            let matrix: Rc<VecModel<ModelRc<i32>>> = Rc::new(VecModel::default());
            for _y in 0..app.get_Y() {
                let n = Rc::new(VecModel::default());
                for _x in 0..app.get_X() {
                    n.push(0);
                }
                matrix.push(n.into());
            }
            app.set_matrix(matrix.into());
            let solutions = Rc::new(VecModel::default());
            app.set_solutions(solutions.into());
        }
    });

    let app_handle = app.as_weak();
    app.on_reset_solutions(move || {
        let app = app_handle.unwrap();
        let new_matrix = VecModel::default();

        let init_matrix = app.get_matrix();
        let init_matrix = init_matrix.as_any().downcast_ref::<VecModel<ModelRc<i32>>>().unwrap();
        init_matrix.iter().for_each(|s| {
            let row = VecModel::default();
            let s = s.as_any().downcast_ref::<VecModel<i32>>().unwrap();
            s.iter().for_each(|x|
                row.push(x.to_shared_string())
            );
            new_matrix.push(ModelRc::new(Rc::new(row)));
        });

        let matrix_list = app.get_solutions();
        let matrix_list = matrix_list.as_any().downcast_ref::<VecModel<ModelRc<ModelRc<SharedString>>>>().unwrap();
        matrix_list.clear();
        matrix_list.push(Rc::new(new_matrix).into());
    });

    let app_handle = app.as_weak();
    app.on_calculate_steps(move || {
        let app = app_handle.unwrap();

        let steplist = app.get_steplist();
        let steplist = steplist.as_any().downcast_ref::<VecModel<SharedString>>().unwrap();
        let matrices = app.get_solutions();
        let matrices = matrices.as_any().downcast_ref::<VecModel<ModelRc<ModelRc<SharedString>>>>().unwrap();
        let curr_matrix = matrices.row_data(matrices.row_count() - 1).unwrap();
        let curr_matrix = curr_matrix.as_any().downcast_ref::<VecModel<ModelRc<SharedString>>>().unwrap();
        let step = steplist.row_data(steplist.row_count() - 1).unwrap();
        let mut step_split: Vec<&str> = step.trim().split('R').filter(|p| !p.is_empty()).collect();
        if step_split[0] == "0" {
            step_split.remove(0);
        }

        match step_split.len() {
            2 => {
                let chars: Vec<char> = step_split[0].chars().collect();
                let a = sub_to_int(chars[0].to_string().as_str()) as usize;
                let b = sub_to_int(step_split[1]) as usize;
                let temp = VecModel::default();
                curr_matrix.iter().for_each(|x| temp.push(x));
                temp.swap( a - 1, b - 1);
                matrices.push(Rc::new(temp).into());
            },
            3 => {
                match step_split[0].parse::<Fraction>() {
                    Ok(num) => {

                        let target_row = sub_to_int(step_split[2]) as usize;

                        let temp = VecModel::default();
                        curr_matrix.iter().for_each(|x| temp.push(x));

                        let data1 = temp.row_data(target_row - 1).unwrap();
                        let data1 = data1.as_any().downcast_ref::<VecModel<SharedString>>().unwrap();
                        let data: VecModel<SharedString> = data1.iter().map(|x| x.parse::<Fraction>().unwrap()).map(|x| (x * num.clone()).to_shared_string()).collect();
                        let data = ModelRc::new(Rc::new(data));

                        temp.set_row_data(target_row - 1, data);
                        
                        matrices.push(Rc::new(temp).into());
                    },
                    Err(_err) => {

                        let mut row_data = Vec::new();
                        let mut factor = Fraction::new(1, 1);

                        step_split.iter().for_each(|x| {
                            let keys: Vec<&str> = x.split(' ').collect();
                            row_data.push(sub_to_int(keys[0]));
                            keys.iter().for_each(|t| {
                                match t.parse::<Fraction>() {
                                    Ok(num) => factor = num,
                                    Err(_err) => (),
                                }
                            });
                        });
                        row_data.pop();

                        let target_row = row_data[0];
                        let factor_row = row_data[1];

                        let temp = VecModel::default();
                        curr_matrix.iter().for_each(|x| temp.push(x));

                        let data1 = temp.row_data(factor_row as usize - 1).unwrap();
                        let data1 = data1.as_any().downcast_ref::<VecModel<SharedString>>().unwrap();
                        let data1 = data1.iter().map(|x| x.parse::<Fraction>().unwrap()).map(|x| x * factor.clone());

                        let data2 = temp.row_data(target_row as usize - 1).unwrap();
                        let data2 = data2.as_any().downcast_ref::<VecModel<SharedString>>().unwrap();
                        let data2 = data2.iter().map(|x| x.parse::<Fraction>().unwrap());

                        let data: VecModel<SharedString> = data1.zip(data2).map(|(x, y)| (x + y).to_shared_string()).collect();
                        let data = ModelRc::new(Rc::new(data));
                        temp.set_row_data(target_row as usize - 1, data);
                        
                        matrices.push(Rc::new(temp).into());

                        println!("multiply row {factor_row} by {factor} and add to row {target_row}");

                    },
                }
            },
            _ => (),
        }
    });

    app.run()
}

fn sub_to_int(string: &str) -> i64 {
    let mut result: i64 = 0;
    let i: Vec<i64> = string.as_bytes().iter().map(|&x| x as i64 - 128).collect();
    let mut digits = Vec::new();
    for x in 0..i.len() {
        if (x + 1) % 3 == 0 {
            digits.push(i[x])
        }
    }
    
    for x in 0..digits.len() {
        result += digits[x] * pow(10, digits.len() - x - 1);
    }
    
    result
}
