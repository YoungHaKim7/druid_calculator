use data::CalcState;
use druid::{AppLauncher, LocalizedString, WindowDesc};

use ui::build_calc;

mod data;
mod ui;

pub fn main() {
    let window = WindowDesc::new(build_calc())
        .window_size((223., 300.))
        .resizable(false)
        .title(
            LocalizedString::new("calc-demo-window-title").with_placeholder("Simple Calculator"),
        );
    let calc_state = CalcState {
        value: "0".to_string(),
        operand: 0.0,
        operator: 'C',
        in_num: false,
    };
    AppLauncher::with_window(window)
        .log_to_console()
        .launch(calc_state)
        .expect("launch failed");
}
