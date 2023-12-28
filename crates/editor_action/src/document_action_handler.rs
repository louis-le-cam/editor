pub trait DocumentActionHandler {
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);

    fn extend_end_left(&mut self);
    fn extend_end_right(&mut self);
    fn extend_end_up(&mut self);
    fn extend_end_down(&mut self);

    fn move_selection_left(&mut self);
    fn move_selection_right(&mut self);
    fn move_selection_up(&mut self);
    fn move_selection_down(&mut self);

    fn insert(&mut self, ch: char);
    fn delete_before(&mut self);
    fn insert_line_before_cursor(&mut self);

    fn write(&mut self);
}
