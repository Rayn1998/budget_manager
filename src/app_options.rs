use crate::Budgets;

pub struct AppOptions<'app> {
    pub budgets: &'app mut Budgets,
    pub current_budget: &'app mut Option<usize>,
    pub something_changed: bool,
    pub been_saved: bool,
}

impl<'app> AppOptions<'app> {
    pub fn changes_true(&mut self) {
        self.something_changed = true;
    }

    pub fn changes_false(&mut self) {
        self.something_changed = false;
    }

    pub fn been_saved_true(&mut self) {
        self.been_saved = true;
    }

    pub fn been_saved_false(&mut self) {
        self.been_saved = false;
    }
}