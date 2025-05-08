use mail_parser::{Message, MessageParser, MessagePart, MimeHeaders};

use rustler::{Atom, Binary, Env, Error, NifResult, NifStruct, OwnedBinary, Term};
use rustler::{Decoder, Encoder};

mod atoms {
    rustler::atoms! {
        ok
    }
}

#[derive(Clone, Debug, NifStruct)]
#[module = "ExMailParser.Attachment"]
struct Attachment {
    name: String,
    content_type: Option<String>,
    content_bytes: ContentBytes,
}

#[derive(Clone, Debug, NifStruct)]
#[module = "ExMailParser.Header"]
struct Header {
    subject: String,
    from: String,
    to: String,
    cc: Option<String>,
    bcc: Option<String>,
    message_id: Option<String>,
    priority: Option<String>,
    x_priority: Option<String>,
    references: Option<String>,
    in_reply_to: Option<String>,
    newsgroup: Option<String>,
    reply_to: Option<String>,
    keywords: Option<String>,
    comments: Option<String>,
    mime_version: Option<String>,
    list_id: Option<String>,
    list_subscribe: Option<String>,
    list_unsubscribe: Option<String>,
    list_post: Option<String>,
    list_archive: Option<String>,
    list_help: Option<String>,
    list_owner: Option<String>,
    content_type: Option<String>,
    date: String,
}

// #[derive(Clone, Debug, NifStruct)]
// #[module = "MailParser.MessagePart"]
// pub struct MessagePart<'x> {
//     pub headers: Vec<Header<'x>>,
//     pub is_encoding_problem: bool,
//     pub body: PartType<'x>,
//     pub encoding: Encoding,
//     pub offset_header: usize,
//     pub offset_body: usize,
//     pub offset_end: usize,
// }

impl From<&MessagePart<'_>> for Attachment {
    fn from(part: &MessagePart) -> Self {
        let name = part.attachment_name().unwrap_or("untitled").to_string();
        let content_bytes = ContentBytes::new(part.contents());

        let content_type = part.content_type().map(|content_type| {
            let roottype = content_type.ctype();

            match content_type.subtype() {
                Some(subtype) => format!("{roottype}/{subtype}"),
                None => roottype.to_string(),
            }
        });

        Attachment {
            name,
            content_bytes,
            content_type,
        }
    }
}

#[derive(Clone, Debug)]
struct ContentBytes(Vec<u8>);

impl ContentBytes {
    fn new(content_bytes: &[u8]) -> Self {
        ContentBytes(content_bytes.to_vec())
    }
}

impl Encoder for ContentBytes {
    fn encode<'a>(&self, env: Env<'a>) -> Term<'a> {
        let mut owned_binary = OwnedBinary::new(self.0.len()).expect("allocation failed");
        owned_binary.as_mut_slice().copy_from_slice(&self.0);
        Binary::from_owned(owned_binary, env).encode(env)
    }
}
impl Decoder<'_> for ContentBytes {
    fn decode(term: Term) -> NifResult<ContentBytes> {
        Ok(Self(term.to_binary().to_vec()))
    }
}

fn get_attachments(message: &Message) -> Vec<Attachment> {
    message
        .attachments()
        .flat_map(|attachment| match attachment.message() {
            Some(nested_message) => get_attachments(nested_message),
            None => Vec::from([attachment.into()]),
        })
        .collect()
}
fn get_header(message: &Message) -> Header {
    // Here what thunderbird is asking for
    // "From" "To" "Cc" "Bcc" "Subject" "Date" "Message-ID" "Priority" "X-Priority"
    // "References" "Newsgroups" "In-Reply-To" "Content-Type" "Reply-To"
    let subject = message.subject().unwrap_or("untitled").to_string();
    let from = message
        .from()
        .and_then(|vec| vec.first())
        .and_then(|address| address.address())
        .map(|addr| addr.to_string())
        .unwrap_or_else(|| String::from(""));

    let to = message
        .to()
        .and_then(|vec| vec.first())
        .and_then(|address| address.address())
        .map(|addr| addr.to_string())
        .unwrap_or_else(|| String::from(""));

    let cc = message
        .cc()
        .and_then(|vec| vec.first())
        .and_then(|address| address.address())
        .map(|addr| addr.to_string());

    let bcc = message
        .bcc()
        .and_then(|vec| vec.first())
        .and_then(|address| address.address())
        .map(|addr| addr.to_string());

    let message_id = message.message_id().map(|item| item.to_string());

    let reply_to = message
        .reply_to()
        .and_then(|vec| vec.first())
        .and_then(|address| address.address())
        .map(|item| item.to_string());

    let in_reply_to = message.in_reply_to().as_text().map(|s| s.to_string());
    let priority = message
        .header_raw("Priority")
        .map(|item| item.trim().to_string());
    let x_priority = message
        .header_raw("X-Priority")
        .map(|item| item.trim().to_string());
    let references = message.references().as_text().map(|s| s.to_string());
    let newsgroup = message
        .header_raw("Newsgroup")
        .map(|item| item.trim().to_string());
    let date = message.date().unwrap().to_rfc3339();
    let list_unsubscribe = message.list_unsubscribe().as_text().map(|s| s.to_string());
    let keywords = message.keywords().as_text().map(|s| s.to_string());
    let comments = message.comments().as_text().map(|s| s.to_string());
    let mime_version = message.mime_version().as_text().map(|s| s.to_string());
    let list_id = message.list_id().as_text().map(|s| s.to_string());
    let list_subscribe = message.list_subscribe().as_text().map(|s| s.to_string());
    let list_post = message.list_post().as_text().map(|s| s.to_string());
    let list_archive = message.list_archive().as_text().map(|s| s.to_string());
    let list_help = message.list_help().as_text().map(|s| s.to_string());
    let list_owner = message.list_owner().as_text().map(|s| s.to_string());
    let content_type = message
        .header_raw("Content-Type")
        .map(|s| s.trim().to_string());

    Header {
        subject: subject,
        from: from,
        to: to,
        cc: cc,
        bcc: bcc,
        message_id: message_id,
        priority: priority,
        x_priority: x_priority,
        references: references,
        newsgroup: newsgroup,
        reply_to: reply_to,
        in_reply_to: in_reply_to,
        content_type: content_type,
        date: date,
        keywords: keywords,
        comments: comments,
        mime_version: mime_version,
        list_id: list_id,
        list_subscribe: list_subscribe,
        list_unsubscribe: list_unsubscribe,
        list_post: list_post,
        list_archive: list_archive,
        list_help: list_help,
        list_owner: list_owner,
    }
}

#[rustler::nif]
fn extract_nested_attachments(raw_message: &str) -> NifResult<(Atom, Vec<Attachment>)> {
    match MessageParser::default().parse(raw_message.as_bytes()) {
        Some(message) => Ok((atoms::ok(), get_attachments(&message))),
        None => Err(Error::Atom("error")),
    }
}

#[rustler::nif]
fn extract_header(raw_message: &str) -> NifResult<(Atom, Header)> {
    match MessageParser::default().parse(raw_message.as_bytes()) {
        Some(message) => Ok((atoms::ok(), get_header(&message))),
        None => Err(Error::Atom("error")),
    }
}

#[rustler::nif]
fn extract_body_html(raw_message: &str) -> NifResult<(Atom, String)> {
    match MessageParser::default().parse(raw_message.as_bytes()) {
        Some(message) => Ok((atoms::ok(), message.body_html(0).unwrap().to_string())),
        None => Err(Error::Atom("error")),
    }
}

#[rustler::nif]
fn extract_body_text(raw_message: &str) -> NifResult<(Atom, String)> {
    match MessageParser::default().parse(raw_message.as_bytes()) {
        Some(message) => Ok((atoms::ok(), message.body_text(0).unwrap().to_string())),
        None => Err(Error::Atom("error")),
    }
}

#[rustler::nif]
fn extract_body_preview(raw_message: &str, preview_len: usize) -> NifResult<(Atom, String)> {
    match MessageParser::default().parse(raw_message.as_bytes()) {
        Some(message) => Ok((
            atoms::ok(),
            message.body_preview(preview_len).unwrap().to_string(),
        )),
        None => Err(Error::Atom("error")),
    }
}

// #[rustler::nif]
// fn extract_message_part(raw_message: &str) -> NifResult<(Atom, MessagePart)> {
//     match MessageParser::default().parse(raw_message.as_bytes()) {
//         Some(message) => Ok((atoms::ok(), get_header(&message))),
//         None => Err(Error::Atom("error")),
//     }
// }

rustler::init!("Elixir.ExMailParser");
