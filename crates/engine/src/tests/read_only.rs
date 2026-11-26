//! View-only mode (`Session::set_read_only`): a host shows a workbook the viewer may look at but
//! not change. Only queries and the commands that look around run; everything else is refused
//! before it runs and changes nothing.

use std::sync::Arc;

use gridcraft_model::{Sheet, Workbook};
use serde_json::{Value as Json, json};

use crate::cmd::{READ_ONLY, command_specs, runs_read_only};
use crate::{EngineError, Session};

/// The sales sample, read-only.
fn viewer() -> Session {
    let mut s = Session::new();
    s.execute("file.new", json!({"sample": "sales"})).unwrap();
    s.set_read_only(true);
    s
}

fn is_query(id: &str) -> bool {
    command_specs().iter().any(|c| c.id == id && !c.journal && !c.undoable)
}

/// The workbook as a reader sees its content: every sheet without where it is scrolled, zoomed and
/// selected, and without which sheet is active.
fn content(wb: &Workbook) -> Workbook {
    let mut wb = wb.clone();
    wb.set_active_sheet(0);
    wb.view = Default::default();
    for sh in wb.sheets.iter_mut() {
        let mut copy: Sheet = (**sh).clone();
        copy.zoom = 100;
        copy.view_active = Default::default();
        copy.view_top_left = Default::default();
        *sh = Arc::new(copy);
    }
    wb
}

#[test]
fn every_editing_command_is_refused_and_changes_nothing() {
    let mut refused = Vec::new();
    for spec in command_specs() {
        if runs_read_only(spec.id, !spec.journal && !spec.undoable) {
            continue;
        }
        let mut s = viewer();
        let (uid, wb) = s.active().map(|d| (d.uid, d.wb.clone())).unwrap();
        let undo = s.active().map(|d| d.undo.len()).unwrap();
        let journal = s.journal.len();
        let r = s.execute(spec.id, json!({}));
        assert!(matches!(&r, Err(EngineError::Disabled(_, why)) if why == READ_ONLY), "{} ran in read-only mode: {r:?}", spec.id);
        let d = s.active().unwrap();
        assert_eq!(d.uid, uid, "{}", spec.id);
        assert!(Arc::ptr_eq(&d.wb, &wb), "{} changed the workbook", spec.id);
        assert_eq!(d.undo.len(), undo, "{}", spec.id);
        assert_eq!(s.journal.len(), journal, "{}", spec.id);
        assert_eq!(s.documents().len(), 1, "{}", spec.id);
        refused.push(spec.id);
    }
    // The commands that change a workbook or hand it out are among them, and most commands are.
    for id in [
        "cell.set",
        "range.setValues",
        "home.bold",
        "edit.paste",
        "edit.cut",
        "edit.undo",
        "file.save",
        "file.saveAs",
        "file.saveBytes",
        "file.exportPdf",
        "file.exportCsv",
        "home.insertSheet",
        "sheet.rename",
        "insert.chart",
    ] {
        assert!(refused.contains(&id), "{id} must be refused");
    }
    assert!(refused.len() * 2 > command_specs().len(), "{} of {} refused", refused.len(), command_specs().len());
}

#[test]
fn what_still_runs_leaves_the_content_alone() {
    let allowed: Vec<&str> = command_specs().iter().filter(|c| runs_read_only(c.id, !c.journal && !c.undoable)).map(|c| c.id).collect();
    assert!(allowed.len() < command_specs().len() / 2);
    // Real parameters for the commands that look around, besides the empty ones every command gets.
    let calls: &[(&str, Json)] = &[
        ("selection.set", json!({"range": "B2:C4"})),
        ("sheet.next", json!({})),
        ("sheet.activate", json!({"index": 0})),
        ("edit.goTo", json!({"reference": "D5"})),
        ("edit.find", json!({"what": "a", "all": true})),
        ("edit.copy", json!({})),
        ("view.zoom", json!({"percent": 150})),
    ];
    let runs = allowed.iter().map(|id| (*id, json!({}))).chain(calls.iter().cloned());
    for (id, params) in runs {
        let mut s = viewer();
        let (uid, before) = s.active().map(|d| (d.uid, content(&d.wb))).unwrap();
        let _ = s.execute(id, params.clone());
        assert!(s.is_read_only(), "{id} left view-only mode");
        // Closing the workbook is allowed; a workbook that is still open has its content.
        if let Some(d) = s.documents().iter().find(|d| d.uid == uid) {
            assert!(content(&d.wb) == before, "{id} {params} changed the workbook's content in read-only mode");
            assert!(d.undo.is_empty(), "{id} made an undo step");
        }
    }
}

#[test]
fn looking_around_still_works() {
    let mut s = viewer();
    s.execute("selection.set", json!({"range": "B2:C4"})).unwrap();
    assert_eq!(s.doc().unwrap().selection.a1(), "B2:C4");
    s.execute("edit.copy", json!({})).unwrap();
    assert!(s.clipboard.is_some(), "copying works");
    s.execute("view.zoom", json!({"percent": 150})).unwrap();
    // The ribbon shows editing commands as unavailable, with the reason.
    let info = s.commands();
    let find = |id: &str| info.iter().find(|c| c.id == id).unwrap();
    assert!(!find("cell.set").enabled);
    assert_eq!(find("cell.set").disabled_reason.as_deref(), Some(READ_ONLY));
    assert!(!find("file.save").enabled && !find("file.saveAs").enabled);
    assert!(find("selection.set").enabled && find("edit.copy").enabled);
    // No cell can be edited, so the UI opens no editor.
    assert_eq!(s.locked_cell_message(0, crate::cell(0, 0)).as_deref(), Some(READ_ONLY));
}

#[test]
fn a_workbook_opened_in_read_only_mode_is_read_only_and_turning_it_off_edits_again() {
    let mut src = Session::new();
    src.execute("file.new", json!({"sample": "sales"})).unwrap();
    let bytes = src.execute("file.saveBytes", json!({"format": "xlsx"})).unwrap();
    let mut s = Session::new();
    s.set_read_only(true);
    s.execute("file.open", json!({"name": "Sales.xlsx", "base64": bytes["base64"]})).unwrap();
    assert_eq!(s.documents().len(), 1, "the host's workbook opens");
    assert!(s.execute("cell.set", json!({"cell": "A1", "input": "x"})).is_err());
    s.set_read_only(false);
    assert!(!is_query("cell.set"));
    s.execute("cell.set", json!({"cell": "A1", "input": "x"})).unwrap();
    assert_eq!(s.locked_cell_message(0, crate::cell(0, 0)), None);
}

#[test]
fn view_only_mode_is_off_by_default() {
    let mut s = Session::new();
    s.new_workbook();
    assert!(!s.is_read_only());
    s.execute("cell.set", json!({"cell": "A1", "input": "1"})).unwrap();
    assert!(s.commands().iter().filter(|c| c.disabled_reason.as_deref() == Some(READ_ONLY)).count() == 0);
}
