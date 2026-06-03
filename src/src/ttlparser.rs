use std::io::Read;
use oxttl::turtle::{ReaderTurtleParser, TurtleParser};
use oxiri::IriParseError;

pub struct TTLConfig{
}

impl TTLConfig {
    pub fn decorate_reader<'a, R: Read>(
        &'a self, mut parser: ReaderTurtleParser<R>)
        -> ReaderTurtleParser<R>
    {
        //parser = self.decorate_with_callback(parser);
        parser
    }

    pub fn decorate_parser(&self, mut parser: TurtleParser)
        -> Result<TurtleParser, IriParseError>
    {
        Ok(parser)
    }
}
