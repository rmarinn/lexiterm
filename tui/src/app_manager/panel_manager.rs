use super::AppManager;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl AppManager {
    /// Selects the [`Panel`] in the [`Direction`] of the currently selected [`Panel`]
    /// and return `true` if a new panel was selected.
    pub fn select_panel(&mut self, direction: Direction) -> bool {
        let Some(new_selected) = self.get_panel_in_dir(direction) else {
            return false;
        };

        if new_selected.kind() == PanelKind::Words {
            new_selected
                .0
                .borrow_mut()
                .links
                .insert(Direction::Up, self.selected_panel());
        }

        self.state.selected_panel = new_selected;

        true
    }

    /// Returns the [`PanelKind`] of the currently selected [`Panel`].
    pub fn selected_panel(&self) -> PanelRef {
        self.state.selected_panel.clone()
    }

    /// Returns the lined [`PanelRefs`] in each [`Direction`] of the currently selected
    /// [`PanelRef`].
    ///
    /// [`PanelRefs`]: PanelRef
    pub fn get_linked_panels(&self) -> HashMap<Direction, PanelRef> {
        self.state.selected_panel.0.borrow().links.clone()
    }

    /// Gets the [`PanelRef`] linked in the specified [`Direction`].
    pub fn get_panel_in_dir(&self, direction: Direction) -> Option<PanelRef> {
        self.state
            .selected_panel
            .0
            .borrow()
            .links
            .get(&direction)
            .cloned()
    }
}

struct Panel {
    kind: PanelKind,
    links: HashMap<Direction, PanelRef>,
}

#[derive(Clone)]
pub struct PanelRef(Rc<RefCell<Panel>>);

impl PanelRef {
    pub fn new(kind: PanelKind) -> Self {
        Self(Rc::new(RefCell::new(Panel {
            kind,
            links: HashMap::default(),
        })))
    }

    /// Link another [`PanelRef`] in a specified [`Direction`] .
    pub fn link(&self, direction: Direction, panel: PanelRef) {
        self.0.borrow_mut().links.insert(direction, panel);
    }

    pub fn kind(&self) -> PanelKind {
        self.0.borrow().kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanelKind {
    Letters,
    Regex,
    Words,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_select_panel() {
        let mut mngr = AppManager::default();

        assert_eq!(mngr.selected_panel().kind(), PanelKind::Letters);

        mngr.select_panel(Direction::Right);

        assert_eq!(mngr.selected_panel().kind(), PanelKind::Regex);

        mngr.select_panel(Direction::Down);

        assert_eq!(mngr.selected_panel().kind(), PanelKind::Words);

        mngr.select_panel(Direction::Left);

        assert_eq!(mngr.selected_panel().kind(), PanelKind::Words);

        mngr.select_panel(Direction::Up);

        assert_eq!(mngr.selected_panel().kind(), PanelKind::Regex);

        mngr.select_panel(Direction::Left);
        mngr.select_panel(Direction::Down);

        assert_eq!(mngr.selected_panel().kind(), PanelKind::Words);
    }
}
