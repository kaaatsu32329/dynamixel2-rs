// Reference: https://emanual.robotis.com/docs/en/dxl/x/xm430-w350/

pub mod xm430_w350 {
	pub const ADDR_MODEL_NUMBER: u16 = 0; // size 2
	pub const ADDR_MODEL_INFORMATION: u16 = 2; // size 4
	pub const ADDR_FIRMWARE_VERSION: u16 = 6; // size 1
	pub const ADDR_ID: u16 = 7; // size 1
	pub const ADDR_BAUD_RATE: u16 = 8; // size 1
	pub const ADDR_DRIVE_MODE: u16 = 10; // size 2
	pub const ADDR_OPERATING_MODE: u16 = 11; // size 1
	pub const ADDR_SECONDARY_ID: u16 = 12; // size 1
	pub const ADDR_PROTOCOL_TYPE: u16 = 13; // size 1
	pub const ADDR_HOMING_OFFSET: u16 = 20; // size 4
	pub const ADDR_MOVING_THRESHOLD: u16 = 24; // size 4
	pub const ADDR_TEMPERATURE_LIMIT: u16 = 31; // size 1
	pub const ADDR_MAX_VOLTAGE_LIMIT: u16 = 32; // size 2
	pub const ADDR_MIN_VOLTAGE_LIMIT: u16 = 34; // size 2
	pub const ADDR_PWM_LIMIT: u16 = 36; // size 2
	pub const ADDR_CURRENT_LIMIT: u16 = 38; // size 2
	pub const ADDR_VELOCITY_LIMIT: u16 = 44; // size 4
	pub const ADDR_MAX_POSITION_LIMIT: u16 = 48; // size 4
	pub const ADDR_MIN_POSITION_LIMIT: u16 = 52; // size 4
	pub const ADDR_STARTUP_CONFIGURATION: u16 = 60; // size 1
	pub const ADDR_SHUTDOWN: u16 = 63; // size 1

	pub const ADDR_TORQUE_ENABLE: u16 = 64; // size 1
	pub const ADDR_LED: u16 = 65; // size 1
	pub const ADDR_STATUS_RETURN_LEVEL: u16 = 68; // size 1
	pub const ADDR_REGISTERED_INSTRUCTION: u16 = 69; // size 1
	pub const ADDR_HARDWARE_ERROR_STATUS: u16 = 70; // size 1
	pub const ADDR_VELOCITY_I_GAIN: u16 = 76; // size 2
	pub const ADDR_VELOCITY_P_GAIN: u16 = 78; // size 2
	pub const ADDR_POSITION_D_GAIN: u16 = 80; // size 2
	pub const ADDR_POSITION_I_GAIN: u16 = 82; // size 2
	pub const ADDR_POSITION_P_GAIN: u16 = 84; // size 2
	pub const ADDR_FEEDFORWARD_2ND_GAIN: u16 = 88; // size 2
	pub const ADDR_FEEDFORWARD_1ST_GAIN: u16 = 90; // size 2
	pub const ADDR_BUS_WATCHING: u16 = 98; // size 1
	pub const ADDR_GOAL_PWM: u16 = 100; // size 2
	pub const ADDR_GOAL_CURRENT: u16 = 102; // size 2
	pub const ADDR_GOAL_VELOCITY: u16 = 104; // size 4
	pub const ADDR_PROFILE_ACCELERATION: u16 = 108; // size 4
	pub const ADDR_PROFILE_VELOCITY: u16 = 112; // size 4
	pub const ADDR_GOAL_POSITION: u16 = 116; // size 4
	pub const ADDR_REALTIME_TICK: u16 = 120; // size 2
	pub const ADDR_MOVING: u16 = 122; // size 1
	pub const ADDR_MOVING_STATUS: u16 = 123; // size 1
	pub const ADDR_PRESENT_PWM: u16 = 124; // size 2
	pub const ADDR_PRESENT_CURRENT: u16 = 126; // size 2
	pub const ADDR_PRESENT_VELOCITY: u16 = 128; // size 4
	pub const ADDR_PRESENT_POSITION: u16 = 132; // size 4
	pub const ADDR_VELOCITY_TRAJECTORY: u16 = 136; // size 4
	pub const ADDR_POSITION_TRAJECTORY: u16 = 140; // size 4
	pub const ADDR_PRESENT_INPUT_VOLTAGE: u16 = 144; // size 2
	pub const ADDR_PRESENT_TEMPERATURE: u16 = 146; // size 1
	pub const ADDR_BACKUP_READY: u16 = 147; // size 1
}
