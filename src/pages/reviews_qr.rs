use resuma::prelude::*;

use crate::landing::{seo_landing, Landing};

pub fn page(_req: FlowRequest) -> View {
    seo_landing(Landing::Reviews)
}
