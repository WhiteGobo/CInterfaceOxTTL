use std::ptr;
use std::os::raw::{c_char, c_uchar, c_void};
use std::ffi::CStr;

use crate::ttlparser::TTLConfig;
use crate::ttlserializer::TTLSerializer;
use crate::trigparser::TrigConfig;
use crate::trigserializer::TrigSerializer;
use crate::callhook::call_hook;

use oxttl::{TurtleParser, TriGParser};
use oxrdf::{Triple, Quad, GraphName};

use crate::genterms::{
    generate_IdentifiedNode, generate_IRI, generate_Term, generate_Graph,
};

type TripleHandler = extern "C" fn(
    *const c_char, u8,
    *const c_char,
    *const c_char, *const c_char, u8,
    *const c_char, u8,
    *mut c_void) -> i8;

unsafe extern "C" {
    pub unsafe fn copy2cstring(input: *mut c_uchar) -> *mut c_uchar;
}

#[unsafe(no_mangle)]
pub extern "C" fn free_TTLConfig(config: *mut TTLConfig){
    if !config.is_null(){
        unsafe { let _ = Box::from_raw(config); }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn TTLConfig_set_baseiri(config: *mut TTLConfig, _iri: *const c_char) -> *mut TTLConfig
{
    if config.is_null() {
        return ptr::null_mut();
    }
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn parse_ttl(
    input: *const c_char,
    hook: TripleHandler,
    hook_data: *mut c_void,
    extra_config: *mut TTLConfig,
    ) -> i64
{
    let x: &CStr = unsafe {CStr::from_ptr(input)};
    let mut parser = TurtleParser::new();
    if !extra_config.is_null() {unsafe{
        parser = match (*extra_config).decorate_parser(parser){
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e);
                return -1;
            }
        };
    }}
    let mut reader = parser.for_reader(x.to_bytes());

    if !extra_config.is_null() {unsafe{
        reader = (*extra_config).decorate_reader(reader);
    }}

    for triple in reader {
        let triple: Triple = match triple {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error during Turtle parsing: {}", e);
                return -3;
            },
        };
        let graph = GraphName::DefaultGraph;
        let quad = Quad::new(triple.subject, triple.predicate, triple.object, graph);

        match call_hook(quad, hook, hook_data){
            Ok(()) => {},
            Err(e) => {
                eprintln!("{}", e);
                return -1;
            },
        }
    }
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn TTL_SER_start() -> *mut TTLSerializer {
    let x = TTLSerializer::new();
    let mybox = Box::new(x);
    let config = Box::into_raw(mybox);
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn TTL_SER_set_base_iri(config: *mut TTLSerializer, _baseiri: *const c_char) -> *mut TTLSerializer
{
    if config.is_null() {
        return ptr::null_mut();
    }
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn TTL_SER_set_prefix(config: *mut TTLSerializer, _name: *const c_char, _iri: *const c_char) -> *mut TTLSerializer
{
    if config.is_null() {
        return ptr::null_mut();
    }
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn TTL_SER_finish(config: *mut TTLSerializer) -> *mut c_uchar
{
    if !config.is_null(){
        unsafe {
            let mut cfg = Box::from_raw(config);
            match unsafe{cfg.finish()} {
                Ok(mut x) => copy2cstring(x.as_mut_ptr()), //allocated with C's malloc
                Err(e) => {
                    eprintln!("turtle parser finish failed: {:?}", e);
                    ptr::null_mut()
                },
            }
        }
    } else {
        ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn TTL_SER_add(
        subject: *const c_char, subject_type: u8,
        predicate: *const c_char,
        object: *const c_char, object_suffix: *const c_char,
        object_type: u8,
        _graph_id: *const c_char, _graph_type: u8,
        serializer: *mut TTLSerializer,
        ) -> i64
{
    if serializer.is_null() {
        return -1;
    }
    unsafe {
        let subj = match generate_IdentifiedNode(
            subject, subject_type, &mut (*serializer))
        {
            Ok(x) => x,
            Err(_) => {eprintln!("Failed to translate subject"); return -2;},
        };
        let pred = match generate_IRI(predicate) {
            Ok(x) => x,
            Err(_) => {eprintln!("Failed to translate predicate");return -3;},
        };
        let obj = match generate_Term(
            object, object_suffix, object_type, &mut (*serializer))
        {
            Ok(x) => x,
            Err(_) => {eprintln!("Failed to translate object"); return -4;},
        };
        (*serializer).serialize_triple(subj.as_ref(), pred, obj.as_ref());
    }
    return 0;
}


#[unsafe(no_mangle)]
pub extern "C" fn free_TrigConfig(config: *mut TrigConfig){
    if !config.is_null(){
        unsafe { let _ = Box::from_raw(config); }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn TrigConfig_set_baseiri(
    config: *mut TrigConfig, _baseiri: *const c_char,
    ) -> *mut TrigConfig
{
    if config.is_null() {
        return ptr::null_mut();
    }
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn parse_trig(
    input: *const c_char,
    hook: TripleHandler,
    hook_data: *mut c_void,
    extra_config: *mut TrigConfig,
    ) -> i64
{
    let x: &CStr = unsafe {CStr::from_ptr(input)};
    let mut parser = TriGParser::new();
    if !extra_config.is_null() {unsafe{
        parser = match (*extra_config).decorate_parser(parser){
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e);
                return -1;
            }
        };
    }}
    let mut reader = parser.for_reader(x.to_bytes());

    if !extra_config.is_null() {unsafe{
        reader = (*extra_config).decorate_reader(reader);
    }}

    for quad_q in reader {
        let quad: Quad = match quad_q {
            Ok(x) => x,
            Err(_e) => {
                //print_parse_error(e);
                return -3;
            },
        };
        match call_hook(quad, hook, hook_data){
            Ok(()) => {},
            Err(_e) => {
                //eprintln!("{}", e);
                return -1;
            },
        }
    }
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn Trig_SER_start() -> *mut TrigSerializer
{
    let x = TrigSerializer::new();
    let mybox = Box::new(x);
    let config = Box::into_raw(mybox);
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn Trig_SER_set_base_iri(config: *mut TrigSerializer, _baseiri: *const c_char) -> *mut TrigSerializer
{
    if config.is_null() {
        return ptr::null_mut();
    }
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn Trig_SER_set_prefix(config: *mut TrigSerializer, _name: *const c_char, _iri: *const c_char) -> *mut TrigSerializer
{
    if config.is_null() {
        return ptr::null_mut();
    }
    config
}

#[unsafe(no_mangle)]
pub extern "C" fn Trig_SER_finish(config: *mut TrigSerializer
    ) -> *mut c_uchar
{
    if !config.is_null(){
        unsafe {
            let mut cfg = Box::from_raw(config);
            match unsafe{cfg.finish()} {
                Ok(mut x) => copy2cstring(x.as_mut_ptr()), //allocated with C's malloc
                Err(e) => {
                    eprintln!("trig parser finish failed: {:?}", e);
                    ptr::null_mut()
                },
            }
        }
    } else {
        ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Trig_SER_add(
        subject: *const c_char, subject_type: u8,
        predicate: *const c_char,
        object: *const c_char, object_suffix: *const c_char,
        object_type: u8,
        graph_id: *const c_char, graph_type: u8,
        serializer: *mut TrigSerializer,
        ) -> i64
{
    if serializer.is_null() {
        return -1;
    }
    unsafe {
        let subj = match generate_IdentifiedNode(
            subject, subject_type, &mut (*serializer))
        {
            Ok(x) => x,
            Err(_) => {return -2;},
        };
        let pred = match generate_IRI(predicate) {
            Ok(x) => x,
            Err(_) => {return -3;},
        };
        let obj = match generate_Term(
            object, object_suffix, object_type, &mut (*serializer))
        {
            Ok(x) => x,
            Err(_) => {return -4;},
        };
        let graph = match generate_Graph(
            graph_id, graph_type, &mut (*serializer))
        {
            Ok(x) => x,
            Err(_) => {return -5;},
        };
        (*serializer).serialize_quad(subj.as_ref(), pred, obj.as_ref(), graph.as_ref());
    }
    return 0;
}
