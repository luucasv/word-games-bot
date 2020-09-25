use super::string_formatter::StringLiteral;

pub struct Join;
impl StringLiteral for Join {
    fn pt() -> &'static str {
        "entrar"
    }
}

pub struct Leave;
impl StringLiteral for Leave {
    fn pt() -> &'static str {
        "sair"
    }
}

pub struct Start;
impl StringLiteral for Start {
    fn pt() -> &'static str {
        "inciar"
    }
}