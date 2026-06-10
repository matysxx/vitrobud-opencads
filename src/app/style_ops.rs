//! Shared CRUD layer for every style manager (text / dimension / table /
//! multileader / multiline).
//!
//! The five managers all expose the same list operations — New, Copy, Delete,
//! Rename, Set-Current — over a named collection of styles. Only the *property
//! editor* and the *storage backend* differ, so those are the only parts kept
//! per-manager:
//!
//! * **Table-backed** (text, dim): live in `Table<T>`, keyed by upper-cased
//!   name. Renaming must re-key the entry and rewrite name-based entity
//!   references (TEXT/MTEXT `style`, DIMENSION `style_name`).
//! * **Object-backed** (table, multileader, multiline): live in
//!   `document.objects`, keyed by handle. Renaming only mutates the `name`
//!   field; entities reference these by handle, so nothing else moves.
//!
//! Centralising the flow here is what fixes the bug class that kept recurring
//! when each manager was hand-copied: a dead New, a missing ribbon refresh, a
//! style added without a handle (dropped on DWG save, issue #67).

use super::OpenCADStudio;
use acadrust::objects::{MLineStyle, MultiLeaderStyle, ObjectType, TableStyle};
use acadrust::tables::{DimStyle, TextStyle};
use acadrust::types::Handle;

/// Which style manager an operation targets. Carried by the shared rename
/// messages so one handler can dispatch to the right storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleKind {
    Text,
    Dim,
    Table,
    MLeader,
    MLine,
}

impl StyleKind {
    /// True when this style feeds the ribbon's quick-set dropdown.
    fn in_ribbon(self) -> bool {
        !matches!(self, StyleKind::MLine)
    }
}

impl OpenCADStudio {
    // ── Queries ────────────────────────────────────────────────────────────

    /// All style names for `kind`, in display order (object-backed styles are
    /// sorted by name so the `HashMap` backing them renders stably).
    pub(super) fn style_names(&self, kind: StyleKind) -> Vec<String> {
        let doc = &self.tabs[self.active_tab].scene.document;
        let mut from_objects = |pick: fn(&ObjectType) -> Option<&str>| -> Vec<String> {
            let mut v: Vec<String> = doc
                .objects
                .values()
                .filter_map(pick)
                .map(str::to_string)
                .collect();
            v.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            v
        };
        match kind {
            StyleKind::Text => doc.text_styles.iter().map(|s| s.name.clone()).collect(),
            StyleKind::Dim => doc.dim_styles.iter().map(|s| s.name.clone()).collect(),
            StyleKind::Table => from_objects(|o| match o {
                ObjectType::TableStyle(s) => Some(s.name.as_str()),
                _ => None,
            }),
            StyleKind::MLeader => from_objects(|o| match o {
                ObjectType::MultiLeaderStyle(s) => Some(s.name.as_str()),
                _ => None,
            }),
            StyleKind::MLine => from_objects(|o| match o {
                ObjectType::MLineStyle(s) => Some(s.name.as_str()),
                _ => None,
            }),
        }
    }

    pub(super) fn style_selected(&self, kind: StyleKind) -> String {
        match kind {
            StyleKind::Text => self.textstyle_selected.clone(),
            StyleKind::Dim => self.dimstyle_selected.clone(),
            StyleKind::Table => self.tablestyle_selected.clone(),
            StyleKind::MLeader => self.mleaderstyle_selected.clone(),
            StyleKind::MLine => self.mlstyle_selected.clone(),
        }
    }

    fn set_style_selected(&mut self, kind: StyleKind, name: String) {
        match kind {
            StyleKind::Text => self.textstyle_selected = name,
            StyleKind::Dim => self.dimstyle_selected = name,
            StyleKind::Table => self.tablestyle_selected = name,
            StyleKind::MLeader => self.mleaderstyle_selected = name,
            StyleKind::MLine => self.mlstyle_selected = name,
        }
    }

    fn style_exists(&self, kind: StyleKind, name: &str) -> bool {
        self.style_names(kind)
            .iter()
            .any(|n| n.eq_ignore_ascii_case(name))
    }

    /// First free `Style{n}` name for a fresh style.
    fn unique_new_name(&self, kind: StyleKind) -> String {
        (1u32..)
            .map(|n| format!("Style{n}"))
            .find(|c| !self.style_exists(kind, c))
            .unwrap()
    }

    /// First free `{base} ({n})` name for a copy / disambiguated entry.
    fn unique_suffixed_name(&self, kind: StyleKind, base: &str) -> String {
        (1u32..)
            .map(|n| format!("{base} ({n})"))
            .find(|c| !self.style_exists(kind, c))
            .unwrap()
    }

    // ── Per-manager glue (the only kind-specific list code) ────────────────

    /// Reload the property-editor buffers for the kinds that have them.
    fn load_style_bufs(&mut self, kind: StyleKind) {
        let i = self.active_tab;
        match kind {
            StyleKind::Text => self.load_textstyle_bufs(i),
            StyleKind::Dim => self.load_dimstyle_bufs(i),
            StyleKind::MLeader => self.load_mleaderstyle_bufs(i),
            StyleKind::Table | StyleKind::MLine => {}
        }
    }

    /// Refresh anything that mirrors the style list / current style after a
    /// mutation (ribbon dropdowns, geometry that depends on the style).
    fn after_style_change(&mut self, kind: StyleKind) {
        if kind.in_ribbon() {
            self.sync_ribbon_styles();
        }
    }

    fn insert_default_style(&mut self, kind: StyleKind, name: &str, handle: Handle) {
        let doc = &mut self.tabs[self.active_tab].scene.document;
        match kind {
            StyleKind::Text => {
                let mut s = TextStyle::new(name);
                s.handle = handle;
                let _ = doc.text_styles.add(s);
            }
            StyleKind::Dim => {
                let mut s = DimStyle::new(name);
                s.handle = handle;
                let _ = doc.dim_styles.add(s);
            }
            StyleKind::Table => {
                let mut s = TableStyle::standard();
                s.name = name.to_string();
                s.handle = handle;
                doc.objects.insert(handle, ObjectType::TableStyle(s));
            }
            StyleKind::MLeader => {
                let mut s = MultiLeaderStyle::new(name);
                s.handle = handle;
                doc.objects.insert(handle, ObjectType::MultiLeaderStyle(s));
            }
            StyleKind::MLine => {
                let mut s = MLineStyle::standard();
                s.name = name.to_string();
                s.handle = handle;
                doc.objects.insert(handle, ObjectType::MLineStyle(s));
            }
        }
    }

    /// Clone the style named `src` under `name` with a fresh `handle`.
    /// Returns false if `src` no longer exists.
    fn clone_style_as(&mut self, kind: StyleKind, src: &str, name: &str, handle: Handle) -> bool {
        let doc = &mut self.tabs[self.active_tab].scene.document;
        match kind {
            StyleKind::Text => {
                if let Some(mut s) = doc.text_styles.get(src).cloned() {
                    s.name = name.to_string();
                    s.handle = handle;
                    let _ = doc.text_styles.add(s);
                    return true;
                }
            }
            StyleKind::Dim => {
                if let Some(mut s) = doc.dim_styles.get(src).cloned() {
                    s.name = name.to_string();
                    s.handle = handle;
                    let _ = doc.dim_styles.add(s);
                    return true;
                }
            }
            StyleKind::Table => {
                if let Some(mut s) = find_object_style(doc, src, |o| match o {
                    ObjectType::TableStyle(s) => Some((s.name.as_str(), s.clone())),
                    _ => None,
                }) {
                    s.name = name.to_string();
                    s.handle = handle;
                    doc.objects.insert(handle, ObjectType::TableStyle(s));
                    return true;
                }
            }
            StyleKind::MLeader => {
                if let Some(mut s) = find_object_style(doc, src, |o| match o {
                    ObjectType::MultiLeaderStyle(s) => Some((s.name.as_str(), s.clone())),
                    _ => None,
                }) {
                    s.name = name.to_string();
                    s.handle = handle;
                    doc.objects.insert(handle, ObjectType::MultiLeaderStyle(s));
                    return true;
                }
            }
            StyleKind::MLine => {
                if let Some(mut s) = find_object_style(doc, src, |o| match o {
                    ObjectType::MLineStyle(s) => Some((s.name.as_str(), s.clone())),
                    _ => None,
                }) {
                    s.name = name.to_string();
                    s.handle = handle;
                    doc.objects.insert(handle, ObjectType::MLineStyle(s));
                    return true;
                }
            }
        }
        false
    }

    fn remove_style_storage(&mut self, kind: StyleKind, name: &str) -> bool {
        let doc = &mut self.tabs[self.active_tab].scene.document;
        match kind {
            StyleKind::Text => doc.text_styles.remove(name).is_some(),
            StyleKind::Dim => doc.dim_styles.remove(name).is_some(),
            StyleKind::Table | StyleKind::MLeader | StyleKind::MLine => {
                let kind2 = kind;
                if let Some(h) = object_handle(doc, name, kind2) {
                    doc.objects.remove(&h).is_some()
                } else {
                    false
                }
            }
        }
    }

    /// Rename `old`→`new` in the backing store, re-keying table entries and
    /// rewriting name-based references + current-style pointers.
    fn rename_style_storage(&mut self, kind: StyleKind, old: &str, new: &str) {
        let i = self.active_tab;
        match kind {
            StyleKind::Text => {
                let doc = &mut self.tabs[i].scene.document;
                if let Some(mut s) = doc.text_styles.get(old).cloned() {
                    s.name = new.to_string();
                    if !s.handle.is_valid() {
                        s.handle = doc.allocate_handle();
                    }
                    let _ = doc.text_styles.add(s);
                }
                doc.text_styles.remove(old);
                if doc.header.current_text_style_name.eq_ignore_ascii_case(old) {
                    doc.header.current_text_style_name = new.to_string();
                }
                for e in doc.entities_mut() {
                    match e {
                        acadrust::entities::EntityType::Text(t)
                            if t.style.eq_ignore_ascii_case(old) =>
                        {
                            t.style = new.to_string();
                        }
                        acadrust::entities::EntityType::MText(t)
                            if t.style.eq_ignore_ascii_case(old) =>
                        {
                            t.style = new.to_string();
                        }
                        _ => {}
                    }
                }
            }
            StyleKind::Dim => {
                let doc = &mut self.tabs[i].scene.document;
                if let Some(mut s) = doc.dim_styles.get(old).cloned() {
                    s.name = new.to_string();
                    if !s.handle.is_valid() {
                        s.handle = doc.allocate_handle();
                    }
                    let _ = doc.dim_styles.add(s);
                }
                doc.dim_styles.remove(old);
                if doc.header.current_dimstyle_name.eq_ignore_ascii_case(old) {
                    doc.header.current_dimstyle_name = new.to_string();
                }
                for e in doc.entities_mut() {
                    if let acadrust::entities::EntityType::Dimension(d) = e {
                        if d.base().style_name.eq_ignore_ascii_case(old) {
                            d.base_mut().style_name = new.to_string();
                        }
                    }
                }
            }
            StyleKind::Table => {
                let doc = &mut self.tabs[i].scene.document;
                if let Some(h) = object_handle(doc, old, kind) {
                    if let Some(ObjectType::TableStyle(s)) = doc.objects.get_mut(&h) {
                        s.name = new.to_string();
                    }
                }
                if self.ribbon.active_table_style.eq_ignore_ascii_case(old) {
                    self.ribbon.active_table_style = new.to_string();
                }
            }
            StyleKind::MLeader => {
                let doc = &mut self.tabs[i].scene.document;
                if let Some(h) = object_handle(doc, old, kind) {
                    if let Some(ObjectType::MultiLeaderStyle(s)) = doc.objects.get_mut(&h) {
                        s.name = new.to_string();
                    }
                }
                if self.tabs[i].active_mleader_style.eq_ignore_ascii_case(old) {
                    self.tabs[i].active_mleader_style = new.to_string();
                }
                if self.ribbon.active_mleader_style.eq_ignore_ascii_case(old) {
                    self.ribbon.active_mleader_style = new.to_string();
                }
            }
            StyleKind::MLine => {
                let doc = &mut self.tabs[i].scene.document;
                if let Some(h) = object_handle(doc, old, kind) {
                    if let Some(ObjectType::MLineStyle(s)) = doc.objects.get_mut(&h) {
                        s.name = new.to_string();
                    }
                }
                if doc.header.multiline_style.eq_ignore_ascii_case(old) {
                    doc.header.multiline_style = new.to_string();
                }
            }
        }
    }

    // ── Public operations (called by the message handlers) ─────────────────

    pub(super) fn style_new(&mut self, kind: StyleKind) {
        let i = self.active_tab;
        let name = self.unique_new_name(kind);
        self.push_undo_snapshot(i, "STYLE NEW");
        let h = self.tabs[i].scene.document.allocate_handle();
        self.insert_default_style(kind, &name, h);
        self.set_style_selected(kind, name.clone());
        self.load_style_bufs(kind);
        self.tabs[i].dirty = true;
        self.after_style_change(kind);
        self.command_line
            .push_output(&format!("Style '{name}' created."));
    }

    pub(super) fn style_copy(&mut self, kind: StyleKind) {
        let i = self.active_tab;
        let src = self.style_selected(kind);
        let name = self.unique_suffixed_name(kind, &src);
        self.push_undo_snapshot(i, "STYLE COPY");
        let h = self.tabs[i].scene.document.allocate_handle();
        if !self.clone_style_as(kind, &src, &name, h) {
            return;
        }
        self.set_style_selected(kind, name.clone());
        self.load_style_bufs(kind);
        self.tabs[i].dirty = true;
        self.after_style_change(kind);
        self.command_line
            .push_output(&format!("Style '{name}' created."));
    }

    pub(super) fn style_delete(&mut self, kind: StyleKind) {
        let i = self.active_tab;
        let name = self.style_selected(kind);
        if name.eq_ignore_ascii_case("Standard") {
            self.command_line
                .push_error("Cannot delete the Standard style.");
            return;
        }
        self.push_undo_snapshot(i, "STYLE DEL");
        if !self.remove_style_storage(kind, &name) {
            return;
        }
        let first = self
            .style_names(kind)
            .into_iter()
            .next()
            .unwrap_or_else(|| "Standard".to_string());
        self.set_style_selected(kind, first);
        self.load_style_bufs(kind);
        self.tabs[i].dirty = true;
        self.after_style_change(kind);
        self.command_line
            .push_output(&format!("Style '{name}' deleted."));
    }

    /// Begin inline rename of the double-clicked style.
    pub(super) fn style_rename_start(&mut self, kind: StyleKind, name: String) {
        self.set_style_selected(kind, name.clone());
        self.load_style_bufs(kind);
        self.style_rename_buf = name.clone();
        self.style_rename = Some(name);
    }

    /// Commit the inline rename. No-op (with feedback) on empty / unchanged /
    /// colliding names, and the Standard style cannot be renamed.
    pub(super) fn style_rename_commit(&mut self, kind: StyleKind) {
        let i = self.active_tab;
        let Some(old) = self.style_rename.take() else {
            return;
        };
        let new = self.style_rename_buf.trim().to_string();
        self.style_rename_buf.clear();
        if new.is_empty() || new.eq_ignore_ascii_case(&old) {
            return;
        }
        if old.eq_ignore_ascii_case("Standard") {
            self.command_line
                .push_error("Cannot rename the Standard style.");
            return;
        }
        if self.style_exists(kind, &new) {
            self.command_line
                .push_error(&format!("Style '{new}' already exists."));
            return;
        }
        self.push_undo_snapshot(i, "STYLE RENAME");
        self.rename_style_storage(kind, &old, &new);
        if self.style_selected(kind).eq_ignore_ascii_case(&old) {
            self.set_style_selected(kind, new.clone());
        }
        self.load_style_bufs(kind);
        self.tabs[i].dirty = true;
        self.after_style_change(kind);
        self.command_line
            .push_output(&format!("Renamed '{old}' → '{new}'."));
    }

    pub(super) fn style_rename_cancel(&mut self) {
        self.style_rename = None;
        self.style_rename_buf.clear();
    }
}

// ── Object-store helpers ───────────────────────────────────────────────────

/// Find the object-backed style named `name` and return a clone. `pick` maps a
/// matching variant to `(its name, a clone of the inner style)`.
fn find_object_style<T>(
    doc: &acadrust::CadDocument,
    name: &str,
    pick: impl Fn(&ObjectType) -> Option<(&str, T)>,
) -> Option<T> {
    doc.objects.values().find_map(|o| {
        let (n, val) = pick(o)?;
        n.eq_ignore_ascii_case(name).then_some(val)
    })
}

fn object_handle(doc: &acadrust::CadDocument, name: &str, kind: StyleKind) -> Option<Handle> {
    doc.objects.iter().find_map(|(&h, o)| {
        let matches = match (kind, o) {
            (StyleKind::Table, ObjectType::TableStyle(s)) => s.name.eq_ignore_ascii_case(name),
            (StyleKind::MLeader, ObjectType::MultiLeaderStyle(s)) => {
                s.name.eq_ignore_ascii_case(name)
            }
            (StyleKind::MLine, ObjectType::MLineStyle(s)) => s.name.eq_ignore_ascii_case(name),
            _ => false,
        };
        matches.then_some(h)
    })
}
