use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

macro_rules! actions {
    ( enum Action { $($content:tt)* } ) => {
        actions!{ @elements {} {} {} {} {} { parse_string } { fuzzy_vec fuzzy_str } Action $($content)* }
    };

    (@elements
        { $($code:tt)* }
        { $($enum_pile:ident $variant_pile:ident)* }
        { $($current_enum:tt)* }
        { $($as_strs:tt)* }
        { $($is_public:tt)* }
        { $parse_args:ident $($parse:tt)* }
        { $fuzzy_vec:ident $fuzzy_str:ident $($fuzzy_match:tt)* }
        $enum_name:ident
        $variant:ident => enum $inner_enum_name:ident {
            $($body:tt)*
        }
        $($tail:tt)*
    ) => {
        actions!{@elements
            { actions!{@elements { $($code)* } {$($enum_pile $variant_pile)* $enum_name $variant} {} {} {} {$parse_args} {$fuzzy_vec $fuzzy_str} $inner_enum_name $($body)* } }
            { $($enum_pile $variant_pile)* }
            { $($current_enum)* $variant($inner_enum_name),}
            { $($as_strs)* Self::$variant(action) => action.as_strs(), }
            { $($is_public)* Self::$variant(action) => action.is_public(), }
            { $parse_args $($parse)* {
                if let Some(action) = $inner_enum_name::parse_from_args($parse_args) {
                    return Some(Self::$variant(action));
                }
            } }
            { $fuzzy_vec $fuzzy_str $($fuzzy_match)* {
                $fuzzy_vec.extend_from_slice(&$inner_enum_name::fuzzy_scores($fuzzy_str));
            } }
            $enum_name $($tail)*
        }
    };

    (@elements
        { $($code:tt)* }
        { $($enum_pile:ident $variant_pile:ident)* }
        { $($current_enum:tt)* }
        { $($as_strs:tt)* }
        { $($is_public:tt)* }
        { $parse_args:ident $($parse:tt)* }
        { $fuzzy_vec:ident $fuzzy_str:ident $($fuzzy_match:tt)* }
        $enum_name:ident
        pub $variant:ident $({
            $($field:ident : $field_ty:ty),*
        })? $(,$string:literal)+;
        $($tail:tt)*
    ) => {
        actions!{@elements
            { $($code)* }
            { $($enum_pile $variant_pile)* }
            { $($current_enum)* $variant $({
                $($field: $field_ty),*
            })?, }
            { $($as_strs)* Self::$variant $({$($field: _),*})? => &[$($string),*], }
            { $($is_public)* Self::$variant $({$($field: _)*})? => true,}
            { $parse_args $($parse)* if matches!($parse_args.get(0), $(Some(&$string))|+) {
                #[allow(unused_variables, unused_mut)]
                let mut arg_index = 0;
                return Some(Self::$variant $({
                    $($field: match ArgumentParse::parse(
                        match $parse_args.get({arg_index += 1; arg_index}) {
                            Some(arg) => arg,
                            None => return None,
                        }) {
                            Some(arg) => arg,
                            None => return None,
                        })*
                })?);
            } }
            { $fuzzy_vec $fuzzy_str $($fuzzy_match)* {$(
                    if let Some(score) = SkimMatcherV2::default().fuzzy_match(
                        $fuzzy_str,
                        &$string[0..$string.char_indices().nth($fuzzy_str.chars().count()).map(|(i, _)| i).unwrap_or($string.len())])
                    {
                        $fuzzy_vec.push((score, $string));
                    }
                )*}
            }
            $enum_name $($tail)*
        }
    };

    (@elements
        { $($code:tt)* }
        { $($enum_pile:ident $variant_pile:ident)* }
        { $($current_enum:tt)* }
        { $($as_strs:tt)* }
        { $($is_public:tt)* }
        { $parse_args:ident $($parse:tt)* }
        { $fuzzy_vec:ident $fuzzy_str:ident $($fuzzy_match:tt)* }
        $enum_name:ident
        $variant:ident $({
            $($field:ident : $field_ty:ty),*
        })? $(,$string:literal),+;
        $($tail:tt)*
    ) => {
        actions!{@elements
            { $($code)* }
            { $($enum_pile $variant_pile)* }
            { $($current_enum)* $variant $({
                $($field: $field_ty),*
            })?, }
            { $($as_strs)* Self::$variant $({$($field: _),*})? => &[$($string),*], }
            { $($is_public)*}
            { $parse_args $($parse)* if matches!($parse_args.get(0), $(Some(&$string))|+) {
                #[allow(unused_variables, unused_mut)]
                let mut arg_index = 0;
                return Some(Self::$variant $({
                    $($field: match ArgumentParse::parse(
                        match $parse_args.get({arg_index += 1; arg_index}) {
                            Some(arg) => arg,
                            None => return None,
                        }) {
                            Some(arg) => arg,
                            None => return None,
                        })*
                })?);
            } }
            { $fuzzy_vec $fuzzy_str $($fuzzy_match)* }
            $enum_name
            $($tail)*
        }
    };

    (@elements
        { $($code:tt)* }
        { $($enum_pile:ident $variant_pile:ident)* }
        { $($current_enum:tt)* }
        { $($as_strs:tt)* }
        { $($is_public:tt)* }
        { $parse_args:ident $($parse:tt)* }
        { $fuzzy_vec:ident $fuzzy_str:ident $($fuzzy_match:tt)* }
        $enum_name:ident
    ) => {
        #[derive(Clone, Debug)]
        pub enum $enum_name {
            $($current_enum)*
        }

        impl $enum_name {
            pub fn as_strs(&self) -> &'static [&'static str] {
                match self {
                    $($as_strs)*
                }
            }

            pub fn is_public(&self) -> bool {
                match self {
                    $($is_public)*
                    _ => false,
                }
            }

            pub fn parse(string: &str) -> Option<Self> {
                Self::parse_from_args(&string.split_whitespace().collect())
            }

            fn parse_from_args($parse_args: &Vec<&str>) -> Option<Self> {
                $($parse)*

                None
            }

            pub fn fuzzy_ordered(fuzzy_str: &str) -> Vec<&'static str> {
                let mut fuzzy_scores = Self::fuzzy_scores(fuzzy_str);
                fuzzy_scores.sort_unstable_by_key(|(score, _)| *score);

                fuzzy_scores.into_iter().rev().map(|(_, string)| string).collect()
            }


            #[allow(unused_variables)]
            pub fn fuzzy_scores($fuzzy_str: &str) -> Vec<(i64, &'static str)> {
                #[allow(unused_mut)]
                let mut $fuzzy_vec = Vec::new();

                $($fuzzy_match)*

                $fuzzy_vec
            }
        }

        actions!{@from_impl
            { $($enum_pile $variant_pile)* }
            $enum_name
        }

        $($code)*
    };

    (@from_impl
        { $($enum_pile:ident $variant_pile:ident)* }
        Action
    ) => {};

    (@from_impl
        { $($enum_pile:ident $variant_pile:ident)* }
        $enum_name:ident
    ) => {
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

            ExtendStartLeft, "extend_start_left";
            ExtendStartRight, "extend_start_right";
            ExtendStartUp, "extend_start_up";
            ExtendStartDown, "extend_start_down";

            ExtendEndLeft, "extend_end_left";
            ExtendEndRight, "extend_end_right";
            ExtendEndUp, "extend_end_up";
            ExtendEndDown, "extend_end_down";

            MoveSelectionLeft, "move_selection_left";
            MoveSelectionRight, "move_selection_right";
            MoveSelectionUp, "move_selection_up";
            MoveSelectionDown, "move_selection_down";

            InsertLineBeforeCursor, "insert_line_before_cursor";
            pub Write, "write", "w";
        }
        pub Quit, "quit", "q";
        pub Open{path: String}, "open", "o";
        pub Redraw, "redraw";
        Validate, "validate";
        Cancel, "cancel";
        EnterNormalMode, "enter_normal_mode";
        EnterInsertMode, "enter_insert_mode";
        EnterSelectionMode, "enter_selection_mode";
        FocusCommandBar, "focus_command_bar";
        FocusEditor, "focus_editor";
    }
}

trait ArgumentParse
where
    Self: Sized,
{
    fn parse(arg: &str) -> Option<Self>;
}

impl ArgumentParse for String {
    fn parse(arg: &str) -> Option<Self> {
        Some(arg.to_string())
    }
}
impl ArgumentParse for char {
    fn parse(arg: &str) -> Option<Self> {
        let mut chars = arg.chars();
        let Some(char) = chars.next() else {
            return None;
        };
        if chars.next().is_some() {
            return None;
        }
        Some(char)
    }
}
