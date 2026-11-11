//! The IDs of the promises on which the loader's rejections rest. A test checks that each is
//! one the specification defines, in the form that a diagnostic's `ruleId` takes; the list
//! [`ALL`] is made from the same declarations, so no rule can be left out of it.

macro_rules! rules {
    ($($(#[doc = $doc:literal])+ $name:ident = $id:literal;)+) => {
        $($(#[doc = $doc])+ pub const $name: &str = $id;)+

        /// Every rule above.
        pub const ALL: &[&str] = &[$($name),+];
    };
}

rules! {
    /// A duplicate key, an anchor, an alias, a merge key, a tag or a key that is not a string.
    SCH_012 = "SCH-012";
    /// A number with no JSON form: an infinity, a not-a-number, or an integer written in
    /// hexadecimal or octal.
    SCH_017 = "SCH-017";
    /// A number with no fractional part outside the range of I-JSON.
    SCH_006 = "SCH-006";
    /// A number with a fractional part outside the range of IEEE 754 binary64.
    SCH_021 = "SCH-021";
    /// A `pattern` that is not a regular expression of ECMA-262, matched with the `u` flag.
    SCH_016 = "SCH-016";
    /// A schema of the specification that a JSON Schema 2020-12 validator cannot compile, or
    /// that uses a `$ref` that does not resolve. Only the specification itself is checked for
    /// this.
    SCH_001 = "SCH-001";
    /// A `$ref` that is not one of the three forms, or that names no Schema contract.
    SCH_015 = "SCH-015";
    /// Something that is not a template, an instance, a contract or the project's one
    /// manifest: a file that is not UTF-8, or not YAML or JSON as the ending of its name says,
    /// and a document or a manifest that a Schema contract rejects.
    SUB_005 = "SUB-005";
    /// Two Schema contracts of the specification with one name. Only the specification itself
    /// is checked for this; the core checks the documents of a project.
    NAM_027 = "NAM-027";
    /// A directory, or an entry of it that MAN-011 reads, that cannot be read: a link that
    /// leads nowhere, a file that may not be read, or anything that is not a regular file.
    SUB_010 = "SUB-010";
    /// A manifest that does not hold exactly one document.
    MAN_001 = "MAN-001";
    /// More than one manifest.
    MAN_002 = "MAN-002";
    /// A document beyond a limit that the engine states under DL10 of the implementation-defined
    /// list: here, how deeply its values may nest.
    SUB_011 = "SUB-011";
}
