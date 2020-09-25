pub enum Lang {
    Pt,
}

pub trait StringFormatter<P> {
    fn pt(args: P) -> String;
    fn format(lang: &Lang, args: P) -> String {
        match lang {
            Lang::Pt => Self::pt(args)
        }
    }
}

pub trait StringLiteral {
    fn pt() -> &'static str;
    fn get(lang: &Lang) -> &'static str {
        match lang {
            Lang::Pt => Self::pt()
        }
    }
}