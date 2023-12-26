mod command_handler;

pub use command_handler::CommandHandler;

use editor_document::Document;

macro_rules! actions {
    (
        enum Actions {
            $($variant_name:ident => enum $enum_name:ident |$($fn_arg:ident : $fn_arg_type:ty),*| {
                $($name:ident $(($($arg_name:ident : $arg_type:ty),+))? $code:block)*
            })+
        }
    ) => {
        pub enum Action {
            $($variant_name($enum_name),)+
        }

        impl Action {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant_name(action) => action.as_str(),)*
                }
            }
        }

        $(
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

                pub fn as_str(&self) -> &'static str {
                    paste::paste!{
                        match self {
                            $(Self::$name $(($($arg_name),+))? => stringify!($name:snake),)*
                        }
                    }
                }
            }
        )+
    };
}

actions! {
    enum Actions {
        Command => enum Command |handler: &mut impl CommandHandler| {
            Quit { handler.quit() }
        }
        Document => enum DocumentAction |document: &mut Document| {
            MoveLeft { document.move_left(); }
            MoveRight { document.move_right(); }
            MoveUp { document.move_up(); }
            MoveDown { document.move_down(); }
            DeleteBefore { document.delete_before() }
            Insert(char: char) { document.insert(*char) }
            Write { document.write() }
            InsertLineBeforeCursor { document.insert_line_before_cursor() }
        }
    }
}
