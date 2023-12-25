macro_rules! define_actions {
    (
        $document:ident,
        $(
            $name:ident
            $((
                $($variant_name:ident : $variant_type:path),+
            ))?
            => $code:block
        )*
    ) => {
        pub enum Action {
            $($name $(($($variant_type),+))?),*
        }

        #[allow(unused_variables)]
        impl Action {
            pub(crate) fn execute(&self, $document: &mut crate::Document) {
                match self {
                    $(Self::$name $(($($variant_name),+))? => $code)*
                }
            }

            pub const fn has_args(&self) -> bool {
                match self {
                    $(Self::$name $(($($variant_name),+))? => define_actions!(has_args $(,($($variant_type)+))?),)*
                }
            }

            pub const fn as_str(&self) -> &'static str {
                paste::paste!{
                    match self {
                        $(Self::$name $(($($variant_name),+))? => stringify!($name:snake),)*
                    }
                }
            }
        }
    };

    (has_args) => { true };
    (has_args, ($($variant_type:path),+)) => { false };
}

define_actions! {
    document,
    MoveLeft => { document.move_left() }
    MoveRight => { document.move_right() }
    MoveUp => { document.move_up() }
    MoveDown => { document.move_down() }
    DeleteBefore => { document.delete_before() }
    Insert(char: char) => { document.insert(*char) }
    Write => { document.write() }
    InsertLineBeforeCursor => { document.insert_line_before_cursor() }
}
