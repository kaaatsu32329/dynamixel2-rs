use std::path::Path;

mod logging;
mod options;

use options::{Command, MotorId, Options};

fn main() {
	if let Err(()) = do_main(clap::Parser::parse()) {
		std::process::exit(1);
	}
}

fn do_main(options: Options) -> Result<(), ()> {
	logging::init(module_path!(), options.verbose as i8);
	match &options.command {
		Command::Ping { motor_id } => {
			let mut bus = open_bus(&options)?;
			match motor_id {
				&MotorId::Id(motor_id) => {
					log::debug!("Sending ping command to motor {}", motor_id);
					let response = bus.ping(motor_id).map_err(|e| log::error!("Command failed: {}", e))?;
					log_ping_response(&response);
				},
				MotorId::Broadcast => {
					log::debug!("Scanning bus for connected motors");
					bus.scan_cb(|response| match response {
						Ok(response) => log::info!("{:?}", response),
						Err(e) => log::warn!("Communication error: {}", e),
					})
					.map_err(|e| log::error!("Command failed: {}", e))?;
				},
			}
		},
		Command::Reboot { motor_id } => {
			let mut bus = open_bus(&options)?;
			log::debug!("Sending reboot command with motor ID {}", motor_id.raw());
			bus.reboot(motor_id.raw()).map_err(|e| log::error!("Command failed: {}", e))?;
			log::info!("Ok");
		},
		Command::Read8 { motor_id, address } => {
			let mut bus = open_bus(&options)?;
			log::debug!("Reading an 8-bit value from motor {} at address {}", motor_id.raw(), address);
			let response = bus
				.read_u8(motor_id.assume_unicast()?, *address)
				.map_err(|e| log::error!("Command failed: {}", e))?;
			log::info!("{:?} (0x{:02X})", response, response.data);
		},
		Command::Read16 { motor_id, address } => {
			let mut bus = open_bus(&options)?;
			log::debug!("Reading a 16-bit value from motor {} at address {}", motor_id.raw(), address);
			let response = bus
				.read_u16(motor_id.assume_unicast()?, *address)
				.map_err(|e| log::error!("Command failed: {}", e))?;
			log::info!("{:?} (0x{:04X})", response, response.data);
		},
		Command::Read32 { motor_id, address } => {
			let mut bus = open_bus(&options)?;
			log::debug!("Reading a 32-bit value from motor {} at address {}", motor_id.raw(), address);
			let response = bus
				.read_u32(motor_id.assume_unicast()?, *address)
				.map_err(|e| log::error!("Command failed: {}", e))?;
			log::info!(
				"{:?} (0x{:04X} {:04X})",
				response,
				(response.data >> 16) & 0xFFFF,
				response.data & 0xFFFF
			);
		},
		Command::Write8 { motor_id, address, value } => {
			let mut bus = open_bus(&options)?;
			log::debug!(
				"Writing 8-bit value {} (0x{:02X}) to motor {} at address {}",
				value,
				value,
				motor_id.raw(),
				address
			);
			let response = bus
				.write_u8(motor_id.raw(), *address, *value)
				.map_err(|e| log::error!("Write failed: {}", e))?;
			log::info!("Ok (Hardware error: {})", response.alert);
		},
		Command::Write16 { motor_id, address, value } => {
			let mut bus = open_bus(&options)?;
			log::debug!(
				"Writing 16-bit value {} (0x{:04X}) to motor {} at address {}",
				value,
				value,
				motor_id.raw(),
				address
			);
			let response = bus
				.write_u16(motor_id.raw(), *address, *value)
				.map_err(|e| log::error!("Command failed: {}", e))?;
			log::info!("Ok (Hardware error: {})", response.alert);
		},
		Command::Write32 { motor_id, address, value } => {
			let mut bus = open_bus(&options)?;
			log::debug!(
				"Writing 32-bit value {} (0x{:04X} {:04X}) to motor {} at address {}",
				value,
				value >> 16,
				value & 0xFFFF,
				motor_id.raw(),
				address
			);
			let response = bus
				.write_u32(motor_id.raw(), *address, *value)
				.map_err(|e| log::error!("Command failed: {}", e))?;
			log::info!("Ok (Hardware error: {})", response.alert);
		},
		Command::ShellCompletion { shell, output } => {
			write_shell_completion(*shell, output.as_deref())?;
		},
	}

	Ok(())
}

fn open_bus(options: &Options) -> Result<dynamixel2::Bus<Vec<u8>, Vec<u8>>, ()> {
	let bus = dynamixel2::Bus::open(&options.serial_port, options.baud_rate, std::time::Duration::from_millis(50))
		.map_err(|e| log::error!("Failed to open serial port: {}: {}", options.serial_port.display(), e))?;
	log::debug!(
		"Using serial port {} with baud rate {}",
		options.serial_port.display(),
		options.baud_rate
	);
	Ok(bus)
}

fn log_ping_response(response: &dynamixel2::instructions::PingResponse) {
	log::info!("Motor ID: {}", response.motor_id);
	log::info!("Model: {}", response.model);
	log::info!("Firmware: {}", response.firmware);
}

fn write_shell_completion(shell: clap_complete::Shell, path: Option<&Path>) -> Result<(), ()> {
	use clap::CommandFactory;
	use std::io::Write;

	let mut buffer = Vec::with_capacity(4 * 1024);

	let mut command = Options::command();
	clap_complete::generate(shell, &mut command, env!("CARGO_BIN_NAME"), &mut buffer);
	if !buffer.ends_with(b"\n") {
		buffer.push(b'\n');
	}

	let path = path.unwrap_or_else(|| Path::new("-"));
	if path == Path::new("-") {
		log::debug!("Writing shell completion for {} to stdout", shell);
		let stdout = std::io::stdout();
		stdout
			.lock()
			.write_all(&buffer)
			.map_err(|e| log::error!("Failed to write to stdout: {}", e))?;
	} else {
		log::debug!("Writing shell completion for {} to {}", shell, path.display());
		let mut output = std::fs::File::create(path).map_err(|e| log::error!("Failed to create {}: {}", path.display(), e))?;
		output
			.write_all(&buffer)
			.map_err(|e| log::error!("Failed to write to {}: {}", path.display(), e))?;
	}

	Ok(())
}
