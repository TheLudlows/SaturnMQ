use crate::Error::{ParseError, ERROR_PARSE, MyResult, NO_ONE};
use crate::Parse::ParseState::*;
use std::str::from_utf8_unchecked;
use crate::Parse::ParseResult::Pub;

/**
PUB <subject> <size>\r\n
<message>\r\n
*/

/**
SUB <subject> <cid>
*/


pub fn parse_error() {
    panic!("parse err");
}

struct Parser {
    state: ParseState,
    buf: [u8; 512],
    head_len: usize,
    //新消息的总长度是msg_total_len,已收到部分应该是msg_len
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
    //pub msg_ref: &'a str,
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
        szb.parse::<usize>().map_err(|_| ParseError::new(4))
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
                    ' ' | '\t' => {}
                    _ => {
                        self.state = OpPubArg;
                        self.head_len = 0;
                        continue;
                    }
                }
                OpPubArg => match c {
                    '\r' => {}
                    '\n' => {
                        self.state = OpMsg;
                        let size = self.get_message_size()?;
                        if size > 1024 * 1024 {
                            return Err(ParseError::new(3));
                        }
                        self.msg_total_len = size;
                    }
                    // add head
                    _ => {
                        self.buf[self.head_len] = c as u8;
                        self.head_len += 1;
                        self.msg_len = 0;
                    }
                }
                OpMsg => {
                    if self.msg_len < self.msg_total_len {
                        self.buf[self.head_len + self.msg_len] = c as u8;
                        self.msg_len += 1;
                    } else {
                        self.state = OpMsgFull;
                    }
                }
                OpMsgFull => match c {
                    '\r' => {}
                    '\n' => {
                        self.state = OpStart;
                        let pub_msg = Pub(self.process_msg());
                        return Ok((pub_msg, index + 1));
                    }
                    _ => {
                        println!("{:?},{}", std::str::from_utf8(buf), self.head_len);
                        parse_error()
                    }
                }
                ParseState::OpS => {}
                ParseState::OpSu => {}
                ParseState::OpSub => {}
                ParseState::OPSubSpace => {}
                ParseState::OpSubArg => {}
                _ => {}
            }
            index += 1;
        }
        return Err(ParseError::new(NO_ONE));
    }

    fn process_msg(&self) -> PubMsg {
        let head = &self.buf[0..self.head_len];
        let head_str = unsafe { from_utf8_unchecked(head) };
        let mut args = head_str.split(' ').into_iter();
        let subject = args.next().unwrap();
        let size = self.msg_total_len;
        PubMsg {
            subject,
            buf: &self.buf[self.head_len..(self.msg_total_len + self.head_len)],
            size,
        }
    }
}

#[test]
fn test() {
    let data = "PUB sub1 6\r\nabcdef\r\nPUB sub2 5\r\n12345\r\n".as_bytes();
    println!("{}", data.len());
    let mut parse = Parser::new();
    let r = parse.parse(&data);
    print_result(&r);
    if let Ok(res) = r {
        let index = res.1;
        let r1 = parse.parse(&data[index..]);
        print_result(&r1)
    }
}

fn print_result(r: &MyResult<(ParseResult, usize)>) {
    match r {
        Ok(res) => match &res.0 {
            Pub(msg) => {
                println!("{:?}", msg.buf);
                println!("{}", msg.size);
                println!("{}", msg.subject);
                println!("{}", res.1)
            }
            _ => {}
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}