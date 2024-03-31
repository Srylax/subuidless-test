#[cfg(test)]
mod integration_tests; // Import der Syscall Definitionen
protocol_proc::create_docker!("client", "protocol", "protocol-proc");
protocol::executor!();
