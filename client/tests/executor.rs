#[cfg(test)]
mod integration_tests;
protocol_proc::create_docker!("client", "protocol", "protocol-proc");
protocol::executor!();
