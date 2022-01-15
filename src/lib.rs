pub trait Plugin {
    type Error;

    fn start(&mut self) -> Result<(), Self::Error>;
    fn process(&mut self, msg: &str) -> Result<(), Self::Error>;
    fn result(&mut self) -> Result<String, Self::Error>;
}

pub fn command<P: Plugin>(plugin: &mut P, msgs: Vec<String>) -> Result<String, P::Error> {
    plugin.start()?;
    for msg in msgs {
        plugin.process(&msg)?;
    }
    plugin.result()
}
