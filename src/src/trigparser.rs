use std::io::Read;
use oxttl::trig::{ReaderTriGParser, TriGParser};
use oxiri::IriParseError;

pub struct TrigConfig {
}

impl TrigConfig {
    pub fn decorate_reader<'a, R: Read>(
        &'a self, mut parser: ReaderTriGParser<R>)
        -> ReaderTriGParser<R>
    {
        //parser = self.decorate_with_callback(parser);
        parser
    }

    pub fn decorate_parser(&self, mut parser: TriGParser)
        -> Result<TriGParser, IriParseError>
    {
        Ok(parser)
    }
}
