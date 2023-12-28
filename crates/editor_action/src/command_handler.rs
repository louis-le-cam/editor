pub trait CommandHandler {
    fn quit(&mut self);
    fn enter_normal_mode(&mut self);
    fn enter_insert_mode(&mut self);
    fn enter_selection_mode(&mut self);
    fn focus_editor(&mut self);
    fn focus_command_bar(&mut self);
    fn validate(&mut self);
    fn cancel(&mut self);
}
