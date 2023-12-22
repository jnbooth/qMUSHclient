use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::str;

use enumeration::Enum;
use mlua::{self, Lua, Value};

use crate::client::state::Phase;
use crate::script::ScriptArg;

pub fn is_valid(target: &str) -> bool {
    let s: &[u8] = target.as_ref();
    !s.is_empty()
        && s[0].is_ascii_alphabetic()
        && s.iter()
            .all(|&c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-' || c == b'.')
}

pub fn validate(target: &str, error: Error) -> Result<(), ParseError> {
    if is_valid(target) {
        Ok(())
    } else {
        Err(ParseError::new(target, error))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseError {
    target: String,
    error: Error,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}: \"{}\"", self.error, self.target)
    }
}
impl StdError for ParseError {}

impl ParseError {
    pub fn new(target: &str, error: Error) -> Self {
        Self {
            target: target.to_owned(),
            error,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Error {
    ///  eg. < ... \n
    UnterminatedElement,
    ///  eg. <!-- ... \n
    UnterminatedComment,
    ///  eg. & ... \n
    UnterminatedEntity,
    ///  eg. < ' ... \n
    UnterminatedQuote,
    ///  eg. <>
    EmptyElement,
    ///  eg. <!>
    ElementTooShort,
    ///  eg. &*;
    InvalidEntityName,
    ///  eg. <!ELEMENT ... > in open mode
    DefinitionWhenNotSecure,
    ///  eg. < 2345 >  or </ 2345 >
    InvalidElementName,
    ///  ie. not <!ELEMENT ...> or <!ENTITY ...>
    InvalidDefinition,
    ///  cannot redefine inbuilt element
    CannotRedefineElement,
    ///  no < in element definition, eg.
    NoTagInDefinition,
    // <!ELEMENT foo 'bold' >  (should be '<bold>' )
    ///  eg. <!ELEMENT foo '<<bold>' >
    UnexpectedDefinitionSymbol,
    ///  eg. <!ELEMENT foo '<send "west>' >
    NoClosingDefinitionQuote,
    ///  eg. <!ELEMENT foo '<bold' >
    NoClosingDefinitionTag,
    ///  defining unknown tag, eg. <!ELEMENT foo '<bar>' >
    NoInbuiltDefinitionTag,
    ///  eg. <!ELEMENT foo '<>' >
    NoDefinitionTag,
    ///  variable name in FLAG does not meet MUSHclient rules
    BadVariableName,
    ///  ATTLIST for undefined element name
    UnknownElementInAttlist,
    ///  cannot redefine inbuilt entity
    CannotRedefineEntity,
    ///  eg. <!ENTITY foo &quot >
    NoClosingSemicolon,
    ///  eg. <!ENTITY foo 'bar' xxxx >
    UnexpectedEntityArguments,
    ///  eg. <blah>
    UnknownElement,
    ///  eg. <send> in open mode
    ElementWhenNotSecure,
    ///  eg. <!ELEMENT foo '<send &bar>'>
    NoClosingSemicolonInArgument,
    ///  closing tag we don't recognise
    ClosingUnknownTag,
    ///  argument to COLOR or FONT not recognised color
    UnknownColor,
    ///  eg. &#xxx;
    InvalidEntityNumber,
    ///  eg. &#5000;
    DisallowedEntityNumber,
    ///  eg. &foo;
    UnknownEntity,
    ///  eg. <color 123=blue>  (123 is invalid)
    InvalidArgumentName,
    ///  eg. <font color=>
    NoArgument,
    ///  using Pueblo element in MXP mode
    PuebloOnly,
    ///  using MXP element in Pueblo mode
    MxpOnly,
    ///  Pueblo does not support <!ELEMENT> or <!ENTITY>
    DefinitionAttemptInPueblo,
    ///  invalid argument to <support> tag
    InvalidSupportArgument,
    ///  invalid argument to <option> tag
    InvalidOptionArgument,
    ///  eg. <!ELEMENT foo '</bold>' >
    DefinitionCannotCloseElement,
    ///  eg. <!ELEMENT foo '<!ELEMENT>' >
    DefinitionCannotDefineElement,
    ///  cannot change option with <recommend_option>
    CannotChangeOption,
    ///  option not in acceptable range
    OptionOutOfRange,
    /// cannot convert bytes into UTF-8
    MalformedBytes,
    ///  eg. </send bar >
    ArgumentsToClosingTag,
    ///  when closing an open tag secure tag blocks it
    OpenTagBlockedBySecureTag,
    ///  eg. </bold> when no opening tag
    OpenTagNotThere,
    ///  cannot close tag - it was opened in secure mode
    TagOpenedInSecureMode,
}

impl Error {
    pub const fn unterminated(phase: Phase) -> Option<Self> {
        match phase {
            Phase::MxpElement => Some(Self::UnterminatedElement),
            Phase::MxpComment => Some(Self::UnterminatedComment),
            Phase::MxpEntity => Some(Self::UnterminatedEntity),
            Phase::MxpQuote => Some(Self::UnterminatedQuote),
            _ => None,
        }
    }
}

impl ScriptArg for Error {
    fn to_arg(self, _: &Lua) -> mlua::Result<Value> {
        Ok(Value::Integer(self as i64 + 1_000))
    }
}
