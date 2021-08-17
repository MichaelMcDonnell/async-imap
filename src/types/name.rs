use std::borrow::Cow;

use imap_proto::{MailboxDatum, Response};

use crate::types::ResponseData;

rental! {
    pub mod rents {
        use super::*;

        /// A name that matches a `LIST` or `LSUB` command.
        #[rental(debug, covariant)]
        pub struct Name {
            response: Box<ResponseData>,
            inner: InnerName<'response>,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct InnerName<'a> {
    attributes: Vec<NameAttribute<'a>>,
    delimiter: Option<&'a str>,
    name: &'a str,
}

pub use rents::Name;

/// From [RFC 6154 section 2](https://datatracker.ietf.org/doc/html/rfc6154#section-2)
/// IMAP LIST Extension for Special-Use Mailboxes:
///
/// > An IMAP server that supports this extension MAY include any or all of
/// > the following attributes in responses to the non-extended IMAP LIST
/// > command.  The new attributes are included along with existing
/// > attributes, such as "\Marked" and "\Noselect".  A given mailbox may
/// > have none, one, or more than one of these attributes.  In some cases,
/// > a special use is advice to a client about what to put in that
/// > mailbox.  In other cases, it's advice to a client about what to
/// > expect to find there.  There is no capability string related to the
/// > support of special-use attributes on the non-extended LIST command.
/// >
/// > ...
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum SpecialUseMailbox {
    /// From [RFC 6154 section 2](https://datatracker.ietf.org/doc/html/rfc6154#section-2)
    /// IMAP LIST Extension for Special-Use Mailboxes:
    ///
    /// > This mailbox presents all messages in the user's message store.
    /// > Implementations MAY omit some messages, such as, perhaps, those
    /// > in \Trash and \Junk.  When this special use is supported, it is
    /// > almost certain to represent a virtual mailbox.
    All,

    /// From [RFC 6154 section 2](https://datatracker.ietf.org/doc/html/rfc6154#section-2)
    /// IMAP LIST Extension for Special-Use Mailboxes:
    ///
    /// > This mailbox is used to archive messages.  The meaning of an
    /// > "archival" mailbox is server-dependent; typically, it will be
    /// > used to get messages out of the inbox, or otherwise keep them
    /// > out of the user's way, while still making them accessible.
    Archive,

    /// From [RFC 6154 section 2](https://datatracker.ietf.org/doc/html/rfc6154#section-2)
    /// IMAP LIST Extension for Special-Use Mailboxes:
    ///
    /// > This mailbox is used to hold draft messages -- typically,
    /// > messages that are being composed but have not yet been sent.  In
    /// > some server implementations, this might be a virtual mailbox,
    /// > containing messages from other mailboxes that are marked with
    /// > the "\Draft" message flag.  Alternatively, this might just be
    /// > advice that a client put drafts here.
    Drafts,

    /// From [RFC 6154 section 2](https://datatracker.ietf.org/doc/html/rfc6154#section-2)
    /// IMAP LIST Extension for Special-Use Mailboxes:
    ///
    /// > This mailbox presents all messages marked in some way as
    /// > "important".  When this special use is supported, it is likely
    /// > to represent a virtual mailbox collecting messages (from other
    /// > mailboxes) that are marked with the "\Flagged" message flag.
    Flagged,

    /// From [RFC 6154 section 2](https://datatracker.ietf.org/doc/html/rfc6154#section-2)
    /// IMAP LIST Extension for Special-Use Mailboxes:
    ///
    /// > This mailbox is where messages deemed to be junk mail are held.
    /// > Some server implementations might put messages here
    /// > automatically.  Alternatively, this might just be advice to a
    /// > client-side spam filter.
    Junk,

    /// From [RFC 6154 section 2](https://datatracker.ietf.org/doc/html/rfc6154#section-2)
    /// IMAP LIST Extension for Special-Use Mailboxes:
    ///
    /// > This mailbox is used to hold copies of messages that have been
    /// > sent.  Some server implementations might put messages here
    /// > automatically.  Alternatively, this might just be advice that a
    /// > client save sent messages here.
    Sent,

    /// From [RFC 6154 section 2](https://datatracker.ietf.org/doc/html/rfc6154#section-2)
    /// IMAP LIST Extension for Special-Use Mailboxes:
    ///
    /// > This mailbox is used to hold messages that have been deleted or
    /// > marked for deletion.  In some server implementations, this might
    /// > be a virtual mailbox, containing messages from other mailboxes
    /// > that are marked with the "\Deleted" message flag.
    /// > Alternatively, this might just be advice that a client that
    /// > chooses not to use the IMAP "\Deleted" model should use this as
    /// > its trash location.  In server implementations that strictly
    /// > expect the IMAP "\Deleted" model, this special use is likely not
    /// > to be supported.
    Trash,
}
/// An attribute set for an IMAP name.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum NameAttribute<'a> {
    /// It is not possible for any child levels of hierarchy to exist
    /// under this name; no child levels exist now and none can be
    /// created in the future.
    NoInferiors,

    /// It is not possible to use this name as a selectable mailbox.
    NoSelect,

    /// The mailbox has been marked "interesting" by the server; the
    /// mailbox probably contains messages that have been added since
    /// the last time the mailbox was selected.
    Marked,

    /// The mailbox does not contain any additional messages since the
    /// last time the mailbox was selected.
    Unmarked,

    /// Special-use mailboxes are defined in
    /// [RFC 6154](https://datatracker.ietf.org/doc/html/rfc6154).
    SpecialUseMailbox(SpecialUseMailbox),

    /// A non-standard user- or server-defined name attribute.
    Custom(Cow<'a, str>),
}

impl NameAttribute<'static> {
    /// Parses the special-use mailbox defined in
    /// [RFC 6154 section 2](https://datatracker.ietf.org/doc/html/rfc6154#section-2)
    /// from the string.
    fn special_use_mailbox(s: &str) -> Option<Self> {
        match s {
            "\\All" => Some(NameAttribute::SpecialUseMailbox(SpecialUseMailbox::All)),
            "\\Archive" => Some(NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Archive)),
            "\\Drafts" => Some(NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Drafts)),
            "\\Flagged" => Some(NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Flagged)),
            "\\Junk" => Some(NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Junk)),
            "\\Sent" => Some(NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Sent)),
            "\\Trash" => Some(NameAttribute::SpecialUseMailbox(SpecialUseMailbox::Trash)),
            _ => None,
        }
    }

    /// Parses the name attributes defined in
    /// [RFC 3501 section 7.2.2](https://datatracker.ietf.org/doc/html/rfc3501#section-7.2.2)
    /// from the string.
    fn system(s: &str) -> Option<Self> {
        match s {
            "\\Noinferiors" => Some(NameAttribute::NoInferiors),
            "\\Noselect" => Some(NameAttribute::NoSelect),
            "\\Marked" => Some(NameAttribute::Marked),
            "\\Unmarked" => Some(NameAttribute::Unmarked),
            _ => None,
        }
    }
}

impl<'a> From<String> for NameAttribute<'a> {
    fn from(s: String) -> Self {
        if let Some(f) = NameAttribute::system(&s) {
            f
        } else if let Some(f) = NameAttribute::special_use_mailbox(&s) {
            f
        } else {
            NameAttribute::Custom(Cow::Owned(s))
        }
    }
}

impl<'a> From<&'a str> for NameAttribute<'a> {
    fn from(s: &'a str) -> Self {
        if let Some(f) = NameAttribute::system(s) {
            f
        } else if let Some(f) = NameAttribute::special_use_mailbox(s) {
            f
        } else {
            NameAttribute::Custom(Cow::Borrowed(s))
        }
    }
}

impl Name {
    pub(crate) fn from_mailbox_data(resp: ResponseData) -> Self {
        Name::new(Box::new(resp), |response| match response.parsed() {
            Response::MailboxData(MailboxDatum::List {
                flags,
                delimiter,
                name,
            }) => InnerName {
                attributes: flags
                    .iter()
                    .map(|s| NameAttribute::from(s.as_ref()))
                    .collect(),
                delimiter: delimiter.as_deref(),
                name,
            },
            _ => panic!("cannot construct from non mailbox data"),
        })
    }

    /// Attributes of this name.
    pub fn attributes(&self) -> &[NameAttribute<'_>] {
        &self.suffix().attributes[..]
    }

    /// The hierarchy delimiter is a character used to delimit levels of hierarchy in a mailbox
    /// name.  A client can use it to create child mailboxes, and to search higher or lower levels
    /// of naming hierarchy.  All children of a top-level hierarchy node use the same
    /// separator character.  `None` means that no hierarchy exists; the name is a "flat" name.
    pub fn delimiter(&self) -> Option<&str> {
        self.suffix().delimiter
    }

    /// The name represents an unambiguous left-to-right hierarchy, and are valid for use as a
    /// reference in `LIST` and `LSUB` commands. Unless [`NameAttribute::NoSelect`] is indicated,
    /// the name is also valid as an argument for commands, such as `SELECT`, that accept mailbox
    /// names.
    pub fn name(&self) -> &str {
        self.suffix().name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that the special-use mailbox attributes that the server returns can
    // be parsed into the correct enum values.
    #[test]
    fn parse_special_use_mailboxes() {
        use self::SpecialUseMailbox::*;
        use NameAttribute::*;

        let special_use_mailboxes = [
            ("\\All", All),
            ("\\Archive", Archive),
            ("\\Drafts", Drafts),
            ("\\Flagged", Flagged),
            ("\\Junk", Junk),
            ("\\Sent", Sent),
            ("\\Trash", Trash),
        ];

        for (string, enum_value) in special_use_mailboxes {
            assert_eq!(NameAttribute::from(string), SpecialUseMailbox(enum_value));
        }
    }
}
