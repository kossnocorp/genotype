use crate::prelude::internal::*;

mod stdio;
pub use stdio::*;

pub trait GtcNoticeSink {
    fn print_notices(&self, notices: Vec<GtNotice>) {
        for notice in notices {
            self.print_notice(notice);
        }
    }

    fn print_notice(&self, notice: GtNotice);
}
