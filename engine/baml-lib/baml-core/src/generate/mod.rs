mod dir_writer;
mod generate_pipeline;
// mod generate_python_client;
mod generate_python_client_old;
mod generate_ruby_client;
mod generate_ts_client;
pub mod ir;

pub(crate) use generate_pipeline::generate_pipeline;