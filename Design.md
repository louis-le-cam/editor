# Actions
Currently actions are divided in two:
  - Document actions
  - Commands

Caveats:
  - write operate on a document but it should be accessible as command
  - Single line document can receive document actions they doesn't support

### Proposal #0
At tree enum that could look like this:
  - Action
    - Other
    - Document
      - SingleLine
      - MultiLine

Single line documents can be given SingleLine and multi line ones can be given Document

Actions that appears in the command bar can be prefixed

All actions can be given to the app via an ActionHandler wich would be implemented by App
which will then have the responsability to forward the action to the focused view 

#### `macro_rules` prototype #0
```rust
actions!{
    handler,

    enum Action {
        Other => enum OtherAction {
            pub Quit, quit;
        }
        Document => enum DocumentAction {
            SingleLine => enum {
                MoveLeft, move_left;
                MoveRight, move_right;
                Insert(char: char);
            }
            MultiLine => enum MultiLineAction {
                MoveUp, move_up;
                MoveDown, move_down;
            }
        }
    }
}
```

#### `macro_rules` prototype #1
```rust
actions!{
    enum Action {
        Document => enum DocumentAction {
            SingleLine => enum SingleLineDocumentAction {
                MoveLeft, "move_left";
                MoveRight, "move_right";
                Insert(char: char), "insert";
            }
            MoveUp, "move_up";
            MoveDown, "move_down";
        }
        pub Write, "write", "w";
        pub Quit, "quit", "q";
    }
}
```

which would extend to

```rust
pub enum Action {
    Document(DocumentAction),
    Write,
    Quit,
}
impl Action {
    pub fn as_strs(&self) -> &'static [&'static str] {
        match self {
            Document(action) => action.as_strs(),
            Write => &["write", "w"],
            Quit => &["quit", "q"],
        }
    }
}

pub enum DocumentAction {
    SingleLine(SingleLineDocumentAction),
    MoveUp,
    MoveDown,
}
impl DocumentAction {
    pub fn as_strs(&self) -> &'static [&'static str] {
        match self {
            SingleLine(action) => action.as_strs(),
            MoveUp => &["move_up"],
            MoveDown => &["move_down"],
        }
    }
}
impl Into<Action> for DocumentAction {
    fn into(self) {
        Action(self)
    }
}

pub enum SingleLineDocumentAction {
    MoveLeft,
    MoveRight,
    Insert(char),
}
impl SingleLineDocumentAction {
    pub fn as_strs(&self) -> &'static [&'static str] {
        match self {
            MoveLeft => &["move_left"],
            MoveRight => &["move_right"],
            MoveInsert => &["insert"],
        }
    }
}
impl Into<Action> for SingleLineDocumentAction {
    fn into(self) {
        Action(DocumentAction(self))
    }
}
impl Into<DocumentAction> for SingleLineDocumentAction {
    fn into(self) {
        DocumentAction(self)
    }
    
}
```
