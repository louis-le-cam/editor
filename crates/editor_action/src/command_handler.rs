pub trait CommandHandler {
    fn quit(&mut self);
    fn enter_insert_mode(&mut self);
    fn enter_normal_mode(&mut self);
}
