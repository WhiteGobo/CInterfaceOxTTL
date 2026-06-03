use oxttl::trig::{WriterTriGSerializer, TriGSerializer};
use std::collections::HashMap;
use oxrdf::{GraphNameRef, LiteralRef, NamedNodeRef, TermRef, NamedOrBlankNodeRef, BlankNodeRef, BlankNode};
use crate::genterms::BNodeMap;

pub struct TrigSerializer {
    pub config: Option<TriGSerializer>,
    writer: Option<WriterTriGSerializer<Vec<u8>>>,
    bnodemap: HashMap<String, BlankNode>,
}

impl BNodeMap for &mut TrigSerializer {
    fn get_bnode(self, key: &str) -> Option<BlankNode>
    {
        let m = &mut self.bnodemap;
        if m.contains_key(key){
            match m.get(key){
                Some(bnode) => Some(bnode.clone()),
                None => None,
            }
        } else {
            let bnode = BlankNode::default();
            m.insert(key.to_owned(), bnode);
            match m.get(key){
                Some(bnode) => Some(bnode.clone()),
                None => None
            }
        }
    }
}

impl TrigSerializer {
    pub fn new() -> Self {
        Self {
            config: Some(TriGSerializer::new()),
            writer: None,
            bnodemap: HashMap::new(),
        }
    }

    pub fn finish(&mut self) -> Result<Vec<u8>, ()> {
        use std::mem::replace;
        let writer = match replace(&mut self.writer, None) {
            None => {return Err(());},
            Some(w) => w,
        };
        let mut ret = match writer.finish() {
            Ok(x) => x,
            Err(_) => {return Err(());},
        };
        ret.push(0); //ensure trailing '\0'
        Ok(ret)
    }

    fn in_write_state(&mut self) {
        use std::mem::replace;
        match replace(&mut self.config, None) {
            None => {},
            Some(cfg) => {
                self.writer = Some(cfg.for_writer(Vec::new()))
            },
        }
    }

    pub fn serialize_quad(&mut self, subj: NamedOrBlankNodeRef, pred: NamedNodeRef, obj: TermRef, graph: GraphNameRef) {
        use oxrdf::QuadRef;
        self.in_write_state();
        match &mut self.writer {
            Some(w) => {
                w.serialize_quad(QuadRef::new(subj, pred, obj, graph));
            },
            None => {},
        };
    }
}
