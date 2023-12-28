pub trait DocumentActionHandler {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);

    fn insert(&mut self, ch: char);
    fn delete_before(&mut self);
    fn insert_line_before_cursor(&mut self);

    fn write(&mut self);
}
