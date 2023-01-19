#[derive(Debug)]
pub struct Address(pub String);

#[derive(Debug)]
pub struct Label(pub String);
#[derive(Debug)]
pub enum AssASTTypes {
    /// a > b == true jump to c
    JumpIfHigherThan {
        a: Address,
        b: Address,
        c: Label,
    },
    /// a/b = c
    Divide {
        a: Address,
        b: Address,
        c: Address,
    },
    /// a != b go to C
    NotEqual {
        a: Address,
        b: Address,
        c: Label,
    },
    /// a == b go to C
    Equal {
        a: Address,
        b: Address,
        c: Label,
    },
    LabelDefenition(Label),
    /// a * b = c
    Multiply {
        a: Address,
        b: Address,
        c: Address,
    },
    /// a - b = c
    Subtract {
        a: Address,
        b: Address,
        c: Address,
    },
    /// jump to a
    Jump {
        a: Label,
    },
    /// copy a to b
    Copy {
        a: Address,
        b: Address,
    },
    /// a + b = c
    Add {
        a: Address,
        b: Address,
        c: Address,
    },
    /// define a thing
    Define {
        name: Label,
        a: u64,
    },
    Exit {
        code: Address,
    },
    /// write to SECS
    Yeet {
        a: Address,
        b: Address,
        c: Address,
    },
}
