use crate::Error::ParseError;
use crate::Parse::ParseState::{OpP, OpS};
use crate::Error::ParseError::parse_error;
#[macro_export]
macro_rules! parse_error {
    ( ) => {{
        return Error(ParseError);
    }};
}
struct Parser {
    state: ParseState,
    buf: [u8; 512],
    arg_len: usize,
    msg_buf: Option<Vec<u8>>,
    //解析过程中收到新消息,那么 新消息的总长度是msg_total_len,已收到部分应该是msg_len
    msg_total_len: usize,
    msg_len: usize,
    debug: bool,
}

pub enum ParseResult<'a> {
    Error(ParseError),
    Sub(SubMsg<'a>),
    Pub(PubMsg<'a>),
}

pub struct SubMsg<'a> {

}

pub struct PubMsg<'a> {}

const BUF_LEN: u32 = 512;

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
            arg_len: 0,
            msg_buf: None,
            msg_total_len: 0,
            msg_len: 0,
            debug: false,
        }
    }

    pub fn parse(&mut self, buf: &[u8]) -> (ParseResult, usize) {
        let index = 0;
        while i < buf.len() {
            let c = buf[i] as char;
            match self.state {
                ParseState::OpStart => {
                    match c {
                        'P' => self.state = OpP,
                        'S' => self.state = OpS,
                        _ => parse_error!()
                    }
                }
                ParseState::OpP => {}
                ParseState::OpPu => {}
                ParseState::OpPub => {}
                ParseState::OpPubSpace => {}
                ParseState::OpPubArg => {}
                ParseState::OpS => {}
                ParseState::OpSu => {}
                ParseState::OpSub => {}
                ParseState::OPSubSpace => {}
                ParseState::OpSubArg => {}
                ParseState::OpMsg => {}
                ParseState::OpMsgFull => {}
            }
        }
        return (ParseResult::new(), 1);
    }
}