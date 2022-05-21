//! see https://url.spec.whatwg.org/#c0-control-percent-encode-set

use percent_encoding::AsciiSet;
pub use percent_encoding::CONTROLS;

pub(crate) const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
pub(crate) const QUERY: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');
pub(crate) const SPECIAL_QUERY: &AsciiSet = &QUERY.add(b'\'');
pub(crate) const PATH: &AsciiSet = &QUERY.add(b'?').add(b'`').add(b'{').add(b'}');
pub(crate) const USERINFO: &AsciiSet = &PATH
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'[')
    .add(b'^')
    .add(b'|');
pub(crate) const COMPONENT: &AsciiSet = &USERINFO.add(b'$').add(b'&').add(b'+').add(b',');
pub(crate) const FORM: &AsciiSet = &COMPONENT.add(b'!').add(b'\'').add(b')').add(b'~');
