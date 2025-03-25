use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct PanelManager {
    selected: Rc<RefCell<PanelTreeNode>>,
}

impl PanelManager {
    pub fn select_panel(&mut self, direction: Direction) {
        let Some(next) = self.selected.borrow().links.get(&direction).cloned() else {
            return;
        };

        if next.borrow().kind == PanelKind::Words {
            next.borrow_mut()
                .links
                .insert(Direction::Up, self.selected.clone());
        }

        self.selected = next;
    }

    pub fn selected(&self) -> PanelKind {
        self.selected.borrow().kind
    }

    pub fn hints(&self) -> HashMap<PanelKind, char> {
        self.selected
            .borrow()
            .links
            .iter()
            .map(|(dir, node)| (node.borrow().kind, dir.to_char()))
            .collect()
    }
}

impl Default for PanelManager {
    fn default() -> Self {
        let letters = Rc::new(RefCell::new(PanelTreeNode::new(PanelKind::Letters)));
        let regex = Rc::new(RefCell::new(PanelTreeNode::new(PanelKind::Regex)));
        let words = Rc::new(RefCell::new(PanelTreeNode::new(PanelKind::Words)));

        letters
            .borrow_mut()
            .links
            .insert(Direction::Right, regex.clone());
        letters
            .borrow_mut()
            .links
            .insert(Direction::Down, words.clone());
        regex
            .borrow_mut()
            .links
            .insert(Direction::Left, letters.clone());
        regex
            .borrow_mut()
            .links
            .insert(Direction::Down, words.clone());

        Self { selected: letters }
    }
}

struct PanelTreeNode {
    kind: PanelKind,
    links: HashMap<Direction, Rc<RefCell<Self>>>,
}

impl PanelTreeNode {
    pub fn new(kind: PanelKind) -> Self {
        Self {
            kind,
            links: HashMap::default(),
        }
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

impl Direction {
    fn to_char(self) -> char {
        match self {
            Direction::Up => '↑',
            Direction::Down => '↓',
            Direction::Left => '←',
            Direction::Right => '→',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_select_panel() {
        let mut panel_mngr = PanelManager::default();

        assert_eq!(panel_mngr.selected(), PanelKind::Letters);
        assert_eq!(
            panel_mngr.hints(),
            HashMap::from([
                (PanelKind::Words, Direction::Down.to_char()),
                (PanelKind::Regex, Direction::Right.to_char())
            ]),
            "wrong hints for {:?}",
            panel_mngr.selected()
        );

        panel_mngr.select_panel(Direction::Right);

        assert_eq!(panel_mngr.selected(), PanelKind::Regex);
        assert_eq!(
            panel_mngr.hints(),
            HashMap::from([
                (PanelKind::Words, Direction::Down.to_char()),
                (PanelKind::Letters, Direction::Left.to_char())
            ]),
            "wrong hints for {:?}: {:?}",
            panel_mngr.selected(),
            panel_mngr
                .selected
                .borrow()
                .links
                .iter()
                .map(|(dir, nd)| (*dir, nd.borrow().kind))
                .collect::<HashMap<Direction, PanelKind>>(),
        );

        panel_mngr.select_panel(Direction::Down);

        assert_eq!(panel_mngr.selected(), PanelKind::Words);
        assert_eq!(
            panel_mngr.hints(),
            HashMap::from([(PanelKind::Regex, Direction::Up.to_char())]),
            "wrong hints for {:?}",
            panel_mngr.selected()
        );

        panel_mngr.select_panel(Direction::Left);

        assert_eq!(panel_mngr.selected(), PanelKind::Words);
        assert_eq!(
            panel_mngr.hints(),
            HashMap::from([(PanelKind::Regex, Direction::Up.to_char())]),
            "wrong hints for {:?}",
            panel_mngr.selected()
        );

        panel_mngr.select_panel(Direction::Up);

        assert_eq!(panel_mngr.selected(), PanelKind::Regex);
        assert_eq!(
            panel_mngr.hints(),
            HashMap::from([
                (PanelKind::Words, Direction::Down.to_char()),
                (PanelKind::Letters, Direction::Left.to_char())
            ]),
            "wrong hints for {:?}",
            panel_mngr.selected()
        );

        panel_mngr.select_panel(Direction::Left);
        panel_mngr.select_panel(Direction::Down);

        assert_eq!(panel_mngr.selected(), PanelKind::Words);
        assert_eq!(
            panel_mngr.hints(),
            HashMap::from([(PanelKind::Letters, Direction::Up.to_char())]),
            "wrong hints for {:?}",
            panel_mngr.selected()
        );
    }
}
