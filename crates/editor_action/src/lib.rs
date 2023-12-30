macro_rules! actions {
    { enum Action { $($content:tt)* } } => {
        actions!{ @elements {} {} {} {} { parse_string } Action $($content)* }
    };

    {@elements
        { $($code:tt)* }
        { $($enum_pile:ident $variant_pile:ident)* }
        { $($current_enum:tt)* }
        { $($as_strs:tt)* }
        { $parse_string:ident $($parse:tt)* }
        $enum_name:ident
        $variant:ident => enum $inner_enum_name:ident {
            $($body:tt)*
        }
        $($tail:tt)*
    } => {
        actions!{@elements
            { actions!{@elements { $($code)* } {$($enum_pile $variant_pile)* $enum_name $variant} {} {} {$parse_string} $inner_enum_name $($body)* } }
            { $($enum_pile $variant_pile)* }
            { $($current_enum)* $variant($inner_enum_name),}
            { $($as_strs)* Self::$variant(action) => action.as_strs(), }
            { $parse_string $($parse)* {
                if let Some(action) = $inner_enum_name::parse($parse_string) {
                    return Some(Self::$variant(action));
                }
            } }
            $enum_name $($tail)*
        }
    };

    {@elements
        { $($code:tt)* }
        { $($enum_pile:ident $variant_pile:ident)* }
        { $($current_enum:tt)* }
        { $($as_strs:tt)* }
        { $parse_string:ident $($parse:tt)* }
        $enum_name:ident
        pub $variant:ident $({
            $($field:ident : $field_ty:ty)*
        })? $(,$string:literal)+;
        $($tail:tt)*
    } => {
        actions!{@elements
            { $($code)* }
            { $($enum_pile $variant_pile)* }
            { $($current_enum)* $variant $({
                $($field: $field_ty)*
            })?, }
            { $($as_strs)* Self::$variant $({$($field: _),*})? => &[$($string),*], }
            { $parse_string $($parse)* if $($parse_string.trim() == $string)||* {
                actions!(@if_has_fields $(($($field)*))? {} else {
                  return Some(Self::$variant);
                });
            } }
            $enum_name
            $($tail)*
        }
    };

    {@elements
        { $($code:tt)* }
        { $($enum_pile:ident $variant_pile:ident)* }
        { $($current_enum:tt)* }
        { $($as_strs:tt)* }
        { $parse_string:ident $($parse:tt)* }
        $enum_name:ident
        $variant:ident $({
            $($field:ident : $field_ty:ty)*
        })? $(,$string:literal),+;
        $($tail:tt)*
    } => {
        actions!{@elements
            { $($code)* }
            { $($enum_pile $variant_pile)* }
            { $($current_enum)* $variant $({
                $($field: $field_ty)*
            })?, }
            { $($as_strs)* Self::$variant $({$($field: _),*})? => &[$($string),*], }
            { $parse_string $($parse)* if $($parse_string.trim() == $string)||* {
                actions!(@if_has_fields $(($($field)*))? {} else {
                  return Some(Self::$variant);
                });
            } }
            $enum_name
            $($tail)*
        }
    };

    {@elements
        { $($code:tt)* }
        { $($enum_pile:ident $variant_pile:ident)* }
        { $($current_enum:tt)* }
        { $($as_strs:tt)* }
        { $parse_string:ident $($parse:tt)* }
        $enum_name:ident
    } => {
        #[derive(Clone, Copy, Debug)]
        pub enum $enum_name {
            $($current_enum)*
        }

        impl $enum_name {
            pub fn as_strs(&self) -> &'static [&'static str] {
                match self {
                    $($as_strs)*
                }
            }

            pub fn parse($parse_string: &str) -> Option<Self> {
                $($parse)*

                None
            }
        }

        actions!{@from_impl
            { $($enum_pile $variant_pile)* }
            $enum_name
        }

        $($code)*
    };

    {@from_impl
        { $($enum_pile:ident $variant_pile:ident)* }
        Action
    } => {};

    {@from_impl
        { $($enum_pile:ident $variant_pile:ident)* }
        $enum_name:ident
    } => {
        #[allow(unused_variables)]
        impl From<$enum_name> for Action {
            fn from(value: $enum_name) -> Self {
                actions!(@from_impl_inner value $($enum_pile $variant_pile)*)
            }
        }
    };

    (@from_impl_inner
        $value:ident
        $front_enum:ident $front_variant:ident $($enum_pile:ident $variant_pile:ident)*
    ) => {
        $front_enum::$front_variant(actions!(@from_impl_inner $value $($enum_pile $variant_pile)*))
    };

    (@from_impl_inner $value:ident) => {
        $value
    };

    (@if_has_fields ($($field:ident)*) $if_code:block else $else_code:block) => {
        $if_code
    };
    (@if_has_fields $if_code:block else $else_code:block) => {
        $else_code
    };
}

// TODO: make action with args parseable
actions! {
    enum Action {
        Document => enum DocumentAction {
            SingleLine => enum SingleLineDocumentAction {
                MoveLeft, "move_left";
                MoveRight, "move_right";
                Insert{char: char}, "insert";
                DeleteBefore, "delete_before";
            }
            MoveUp, "move_up";
            MoveDown, "move_down";
            ExtendEndLeft, "extend_end_left";
            ExtendEndRight, "extend_end_right";
            ExtendEndUp, "extend_end_up";
            ExtendEndDown, "extend_end_down";
            MoveSelectionLeft, "move_selection_left";
            MoveSelectionRight, "move_selection_right";
            MoveSelectionUp, "move_selection_up";
            MoveSelectionDown, "move_selection_down";
            InsertLineBeforeCursor, "insert_line_before_cursor";
            Write, "write";
        }
        pub Quit, "quit", "q";
        Validate, "validate";
        Cancel, "cancel";
        EnterNormalMode, "enter_normal_mode";
        EnterInsertMode, "enter_insert_mode";
        EnterSelectionMode, "enter_selection_mode";
        FocusCommandBar, "focus_command_bar";
        FocusEditor, "focus_editor";
    }
}
