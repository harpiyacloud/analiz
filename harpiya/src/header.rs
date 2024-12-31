use harpiya_domain::ecode::ECode;
use harpiya_domain::public::ReplyHeader;

pub fn reply(id: String) -> Option<ReplyHeader> {
    Some(ReplyHeader {
        trace_id: id,
        code: ECode::Success.into(),
        message: Default::default(),
    })
}

pub fn err_reply(id: String, code: ECode, msg: String) -> Option<ReplyHeader> {
    Some(ReplyHeader {
        trace_id: id,
        code: code.into(),
        message: msg,
    })
}