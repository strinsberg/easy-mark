pub trait Writer {
    fn to_text(&self) -> String;
    fn to_latex(&self) -> String;
}

