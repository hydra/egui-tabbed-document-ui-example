use crate::app::Config;

pub struct Context<'a> {
    pub config: &'a mut Config,
}