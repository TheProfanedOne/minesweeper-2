use web_sys::MouseEvent;

mod random;
mod minesweeper;

use {
    std::cell::RefCell,
    minesweeper::*,
    wasm_bindgen::{
        prelude::*,
        JsCast,
    },
    substring::Substring,
};

thread_local! {
    static MS: RefCell<Minesweeper> = RefCell::new(Minesweeper::new({
        let size = prompt("Choose a board size (\"Small\", \"Medium\", or \"Large\"):", "Medium");
        if size == "Small" || size == "small" || size == "s" {
            (9, 9, 10)
        } else if size == "Medium" || size == "medium" || size == "m" {
            (16, 16, 40)
        } else if size == "Large" || size == "large" || size == "l" {
            (30, 16, 99)
        } else {
            alert(&format!("\"{}\" is not a valid board size. Defaulting to Medium.", size));
            (16, 16, 40)
        }
    }));

    static FO: RefCell<usize> = RefCell::new(0);
    static FR: RefCell<usize> = RefCell::new(MS.with(|ms| ms.borrow().mine_count()));
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn prompt(text: &str, default_text: &str) -> String;
}

fn get_data(state: String) -> Vec<Vec<String>> {
    let mut cells: Vec<Vec<String>> = vec![vec![]];

    state.split("\n").for_each(|row| {
        let mut cells_row: Vec<String> = vec![];

        row.split("\u{200B}").for_each(|cell| cells_row.push(cell.into()));

        cells.push(cells_row);
    });

    cells
}

fn open_fields(x: usize, y: usize) -> bool {
    if let Some(status) = MS.with(|ms| ms.borrow_mut().open((x, y))) {
        if status == OpenResult::Mine {
            if !MS.with(|ms| ms.borrow().lose_state()) {
                MS.with(|ms| ms.borrow_mut().lost());
            }
        }
        true
    } else {
        false
    }
}

fn toggle_flag(x: usize, y: usize) -> bool {
    if let Some(flagged) = MS.with(|ms| ms.borrow_mut().toggle_flag((x, y))) {
        flagged
    } else {
        false
    }
}

fn render() -> Result<(), JsValue> {
    let window = web_sys::window().expect("No global `window` exists.");
    let document = window.document().expect("There should be a document on the window.");

    let mut lost: bool = MS.with(|ms| ms.borrow().lose_state());
    let mut false_alarm_var = false;
    if lost {
        if FO.with(|fo| *fo.borrow()) == 1 {
            MS.with(|ms| ms.borrow_mut().false_lost());
            lost = false;
            MS.with(|ms| ms.borrow_mut().board_reset());
            false_alarm_var = true;
        } else {
            let (width, height) = MS.with(|ms| ms.borrow().width_and_height());
            MS.with(|ms| ms.borrow_mut().show_loss(width, height));
        }
    }

    let won = MS.with(|ms| ms.borrow().win_check());

    let root = document.get_element_by_id("root").expect("No element with the id `root` exists.");
    root.set_inner_html("");

    let data = get_data(MS.with(|ms| ms.borrow().to_string()));

    root.class_list().add_1("board")?;
    root.set_attribute("style", &format!(
        "display: inline-grid; grid-template: repeat({}, auto) / repeat({}, auto)",
        data.len(), data[0].len()
    ))?;

    for (y, row) in data.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let field = document.create_element("a")?;

            let mut style = "field";
            if lost && cell == "💣" {
                style = "mine";
            }
            else if won && cell == "🚩" {
                style = "flag";
            }
            field.class_list().add_1(style)?;

            if style == "field" && cell != "  " {
                let num: u8 = cell.substring(0, 1).parse().unwrap();
                match num {
                    1 => field.class_list().add_1("one")?,
                    2 => field.class_list().add_1("two")?,
                    3 => field.class_list().add_1("three")?,
                    4 => field.class_list().add_1("four")?,
                    5 => field.class_list().add_1("five")?,
                    6 => field.class_list().add_1("six")?,
                    7 => field.class_list().add_1("seven")?,
                    _ => field.class_list().add_1("eight")?,
                }
            }
            
            field.set_attribute("href", "#")?;
            field.set_text_content(Some(cell));

            {
                let closure = Closure::<dyn FnMut(_) -> Result<(), JsValue>>::new(move |evt: MouseEvent| -> Result<(), JsValue> {
                    evt.prevent_default();
                    if !lost && !won {
                        if open_fields(x, y) {
                            if !false_alarm_var {
                                FO.with(|fo| *fo.borrow_mut() += 1);
                            } else {
                                false_alarm_var = false;
                            }
                        }
                        render()
                    } else {
                        Ok(())
                    }
                });
                field.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
                closure.forget();
            }
            {
                let closure = Closure::<dyn FnMut(_) -> Result<(), JsValue>>::new(move |evt: MouseEvent| -> Result<(), JsValue> {
                    evt.prevent_default();
                    if !lost && !won {
                        if FR.with(|fr| *fr.borrow()) > 0 {
                            if toggle_flag(x, y) {
                                FR.with(|fr| *fr.borrow_mut() -= 1);
                            } else if !MS.with(|ms| ms.borrow().is_open((x, y))) {
                                FR.with(|fr| *fr.borrow_mut() += 1);
                            }
                        } else if MS.with(|ms| ms.borrow().is_flagged((x, y))) {
                            toggle_flag(x, y);
                            FR.with(|fr| *fr.borrow_mut() += 1);
                        }
                        render()
                    } else {
                        Ok(())
                    }
                });
                field.add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())?;
                closure.forget();
            }

            root.append_child(&field)?;
        }
    }

    Ok(())
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    render()
}
