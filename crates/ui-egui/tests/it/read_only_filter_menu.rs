//! View-only mode (`Session::set_read_only`): opening a table's filter menu changes nothing.
//! Editable, the menu installs the table's autofilter on demand; read-only it must not (the
//! fallback that installs it writes the sheet directly, past the command check).

use gridcraft_engine::Session;
use gridcraft_ui_egui::SheetApp;
use serde_json::json;
use std::sync::Arc;

fn session() -> Session {
    let mut s = Session::new();
    s.new_workbook();
    s.execute("range.setValues", json!({"range": "A1", "values": [["Name", "Amount"], ["a", 1], ["b", 2]]})).unwrap();
    s.execute("insert.table", json!({"range": "A1:B3", "header": true})).unwrap();
    s
}

/// Opens the filter menu of column B (the table's header row) and draws a few frames. Returns
/// whether the workbook changed, and whether the menu is still open.
fn open_filter_menu(read_only: bool) -> (bool, bool) {
    let mut s = session();
    let sh = s.active().unwrap().wb.active().unwrap().clone();
    assert!(sh.autofilter.is_none(), "precondition: the table has no sheet autofilter yet");
    s.set_read_only(read_only);
    let before = s.active().unwrap().wb.clone();
    let mut h = crate::common::builder().with_size(egui::vec2(1000.0, 700.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        crate::common::app(s),
    );
    h.run_steps(2);
    h.state_mut().grid.filter_menu = Some((1, 0, egui::pos2(300.0, 120.0)));
    h.run_steps(3);
    let d = h.state().session.active().unwrap();
    (!Arc::ptr_eq(&d.wb, &before), h.state().grid.filter_menu.is_some())
}

#[test]
fn an_editable_table_gets_its_autofilter_when_the_filter_menu_opens() {
    let (changed, open) = open_filter_menu(false);
    assert!(changed, "editable: the menu installs the table's autofilter");
    assert!(open, "editable: the menu stays open");
}

#[test]
fn a_read_only_table_is_left_alone_and_the_menu_closes() {
    let (changed, open) = open_filter_menu(true);
    assert!(!changed, "read-only: opening the filter menu must not change the workbook");
    assert!(!open, "read-only: sorting and filtering are edits, so the menu has nothing to offer");
}
