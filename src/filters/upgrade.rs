//! HTTP protocol upgrade filters

use crate::filter::filter_fn_one;
use crate::{reject, Filter, Rejection};
use futures_util::future;
use hyper::upgrade::OnUpgrade;

/// Extract the hyper HTTP protocol upgrade token for the current request.
pub fn on_upgrade() -> impl Filter<Extract = (OnUpgrade,), Error = Rejection> + Copy {
    filter_fn_one(|route| {
        let res = match route.extensions_mut().remove::<OnUpgrade>() {
            Some(on_upgrade) => Ok(on_upgrade),
            None => {
                tracing::debug!("connection has no upgrade callback present");
                Err(reject())
            }
        };
        future::ready(res)
    })
}
