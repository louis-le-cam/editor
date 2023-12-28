mod command_handler;
mod document_action_handler;

pub use command_handler::CommandHandler;
pub use document_action_handler::DocumentActionHandler;

macro_rules! actions {
    (
        enum Action {
            $($variant_name:ident => enum $enum_name:ident |$($fn_arg:ident : $fn_arg_type:ty),*| {
                $($name:ident $(($($arg_name:ident : $arg_type:ty),+))? $(, $string:literal)+ $code:block)*
            })+
        }
    ) => {
        #[derive(Clone, Debug)]
        pub enum Action {
            $($variant_name($enum_name),)+
        }

        impl Action {
            pub fn as_strs(&self) -> &'static [&'static str] {
                match self {
                    $(Self::$variant_name(action) => action.as_strs(),)*
                }
            }
        }

        $(
            #[derive(Clone, Debug)]
            pub enum $enum_name {
                $($name $(($($arg_type)+))?,)*
            }

            #[allow(unused_variables)]
            impl $enum_name {
                pub fn execute(&self, $($fn_arg: $fn_arg_type),*) {
                    match self {
                        $(Self::$name $(($($arg_name),+))? => $code)*
                    }
                }

                pub fn as_strs(&self) -> &'static [&'static str] {
                    paste::paste!{
                        match self {
                            $(Self::$name $(($($arg_name),+))? => &[$($string),+],)*
                        }
                    }
                }

                // TODO: Support actions with args
                pub fn from_str(str: &str) -> Option<Self> {
                    paste::paste!{
                        match str {
                            $($($string)|+ => actions!(from_str, $name $(,($($arg_name),+))?),)*
                            _ => None
                        }
                    }
                }
            }

            impl Into<Action> for $enum_name {
                fn into(self) -> Action {
                    Action::$variant_name(self)
                }
            }
        )+
    };

    (from_str, $name:ident, ($($arg_name:ident),+)) => {
        None
    };
    (from_str, $name:ident) => {
        Some(Self::$name)
    };
}

actions! {
    enum Action {
        Command => enum Command |handler: &mut impl CommandHandler| {
            Quit, "q", "quit" { handler.quit() }
            EnterNormalMode, "enter_normal_mode" { handler.enter_normal_mode() }
            EnterInsertMode, "enter_insert_mode" { handler.enter_insert_mode() }
            EnterSelectionMode, "enter_selection_mode" { handler.enter_selection_mode() }
            FocusEditor, "focus_editor" { handler.focus_editor() }
            FocusCommandBar, "focus_command_bar" { handler.focus_command_bar() }
            Validate, "validate" { handler.validate() }
            Cancel, "cancel" { handler.cancel() }
        }
        Document => enum DocumentAction |document: &mut impl DocumentActionHandler| {
            MoveLeft, "move_left" { document.move_left(); }
            MoveRight, "move_right" { document.move_right(); }
            MoveUp, "move_up" { document.move_up(); }
            MoveDown, "move_down" { document.move_down(); }
            ExtendEndLeft, "extend_end_left" { document.extend_end_left(); }
            ExtendEndRight, "extend_end_right" { document.extend_end_right(); }
            ExtendEndUp, "extend_end_up" { document.extend_end_up(); }
            ExtendEndDown, "extend_end_down" { document.extend_end_down(); }
            MoveSelectionLeft, "move_selection_left" { document.move_selection_left(); }
            MoveSelectionRight, "move_selection_right" { document.move_selection_right(); }
            MoveSelectionUp, "move_selection_up" { document.move_selection_up(); }
            MoveSelectionDown, "move_selection_down" { document.move_selection_down(); }
            DeleteBefore, "delete_before" { document.delete_before() }
            Insert(char: char), "insert"  { document.insert(*char) }
            Write, "write" { document.write() }
            InsertLineBeforeCursor, "insert_line_before_cursor" { document.insert_line_before_cursor() }
        }
    }
}
