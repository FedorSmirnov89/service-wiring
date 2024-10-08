//! Defines the data used for the generation of the macro code.

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use syn::{Ident, Path};

#[derive(Debug)]
pub(super) struct WiringData {
    pub(super) services: Vec<ServiceData>,
    pub(super) events: HashSet<Ident>,
    pub(super) in_event_consumers: HashMap<Ident, Vec<String>>,
}

#[derive(Clone)]
pub(super) struct ServiceData {
    pub(super) type_name: Ident,
    pub(super) field_name: String,
    pub(super) path: Path,
    pub(super) out_events: Vec<Ident>,
}

impl Debug for ServiceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "type_name: {type_name}", type_name = self.type_name)?;
        writeln!(f, "field_name: {field_name}", field_name = self.field_name)?;
        for segment in &self.path.segments {
            writeln!(f, "path seg: {seg}", seg = segment.ident)?;
        }
        Ok(())
    }
}
