use crate::Error::{ParseError, ERROR_PARSE, MyResult};
use crate::Parse::ParseState::*;

/**
PUB <subject> <size>\r\n
<message>\r\n
*/


pub fn parse_error() {
    panic!("parse err");
}

struct Parser {
    state: ParseState,
    buf: [u8; 512],
    head_len: usize,
    msg_buf: Option<Vec<u8>>,
    //解析过程中收到新消息,那么 新消息的总长度是msg_total_len,已收到部分应该是msg_len
    msg_total_len: usize,
    msg_len: usize,
    debug: bool,
}

pub enum ParseResult<'a> {
    Error,
    Sub(SubMsg<'a>),
    Pub(PubMsg<'a>),
}


pub struct SubMsg<'a> {
    pub subject: &'a str,
    pub sid: &'a str,
    pub queue: Option<&'a str>,
}

pub struct PubMsg<'a> {
    pub subject: &'a str,
    pub msg_ref: &'a str,
    pub buf: &'a [u8],
    pub size: usize,
}

const BUF_LEN: usize = 512;

pub enum ParseState {
    OpStart,
    OpS,
    OpSu,
    OpSub,
    OPSubSpace,
    OpSubArg,
    OpP,
    OpPu,
    OpPub,
    //pub argument
    OpPubSpace,
    OpPubArg,
    OpMsg,
    //pub message
    OpMsgFull,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: ParseState::OpStart,
            buf: [0; BUF_LEN],
            head_len: 0,
            msg_buf: None,
            msg_total_len: 0,
            msg_len: 0,
            debug: false,
        }
    }
    fn get_message_size(&self) -> MyResult<usize> {
        let arg_buf = &self.buf[0..self.head_len];
        let pos = arg_buf
            .iter()
            .rev()
            .position(|b| *b == ' ' as u8 || *b == '\t' as u8);
        if pos.is_none() {
            parse_error();
        }
        let pos = pos.unwrap();
        let size_buf = &arg_buf[arg_buf.len() - pos..];
        let szb = unsafe {
            std::str::from_utf8_unchecked(size_buf)
        };
        szb.parse::<usize>().map_err(|_| ParseError::new(ERROR_PARSE))
    }

    pub fn parse(&mut self, buf: &[u8]) -> MyResult<(ParseResult, usize)> {
        let mut index = 0;
        while index < buf.len() {
            let c = buf[index] as char;
            match &self.state {
                OpStart => match c {
                    'P' => self.state = OpP,
                    'S' => self.state = OpS,
                    _ => parse_error()
                }
                OpP => match c {
                    'U' => self.state = OpPu,
                    _ => parse_error(),
                }
                OpPu => match c {
                    'B' => self.state = OpPub,
                    _ => parse_error(),
                }
                OpPub => match c {
                    ' ' => self.state = OpPubSpace,
                    _ => parse_error(),
                }
                OpPubSpace => match c {
                    ' ' | '\t' => {
                        self.state = OpPubArg;
                        self.head_len = 0;
                        continue;
                    }
                    _ => parse_error(),
                }
                OpPubArg => match c {
                    '\t' => {},
                    '\n' => {
                        self.state = OpMsg;
                        let size = self.get_message_size()?;
                    },
                    // subject size
                    _ => {
                        self.head_len +=1;
                    }
                }
                ParseState::OpS => {}
                ParseState::OpSu => {}
                ParseState::OpSub => {}
                ParseState::OPSubSpace => {}
                ParseState::OpSubArg => {}
                ParseState::OpMsg => {}
                ParseState::OpMsgFull => {}
                _ => {}
            }
            index += 1;
        }
        return Ok((ParseResult::Error, 1));
    }
}
fn get_message_size(buf : &[u8]) -> MyResult<usize> {
    let pos = buf
        .iter()
        .rev()
        .position(|b| *b == ' ' as u8 || *b == '\t' as u8);
    if pos.is_none() {
        parse_error();
    }
    let pos = pos.unwrap();
    let size_buf = &buf[buf.len() - pos..];
    let szb = unsafe {
        std::str::from_utf8_unchecked(size_buf)
    };
    szb.parse::<usize>().map_err(|_| ParseError::new(ERROR_PARSE))
}

#[test]
fn test() {
    let buf = "abc 123".as_bytes();
    let size = get_message_size(&buf);
    println!("{}",size.unwrap())
}