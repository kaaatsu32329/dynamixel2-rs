use super::instruction_id;
use crate::endian::write_u16_le;
use crate::{bus::StatusPacket, Bus, Response, TransferError};

impl<ReadBuffer, WriteBuffer> Bus<ReadBuffer, WriteBuffer>
where
	ReadBuffer: AsRef<[u8]> + AsMut<[u8]>,
	WriteBuffer: AsRef<[u8]> + AsMut<[u8]>,
{
	/// Read an arbitrary number of bytes from a specific motor.
	///
	/// This function will not work correctly if the motor ID is set to [`packet_id::BROADCAST`][crate::instructions::packet_id::BROADCAST].
	/// Use [`Self::sync_read`] to read from multiple motors with one command.
	pub fn read(&mut self, motor_id: u8, address: u16, count: u16) -> Result<StatusPacket<'_, ReadBuffer, WriteBuffer>, TransferError> {
		let response = self.transfer_single(motor_id, instruction_id::READ, 4, |buffer| {
			write_u16_le(&mut buffer[0..], address);
			write_u16_le(&mut buffer[2..], count);
		})?;
		crate::error::InvalidParameterCount::check(response.parameters().len(), count.into()).map_err(crate::ReadError::from)?;
		Ok(response)
	}

	/// Read an 8 bit register from a specific motor.
	///
	/// This function will not work correctly if the motor ID is set to [`packet_id::BROADCAST`][crate::instructions::packet_id::BROADCAST].
	/// Use [`Self::sync_read`] to read from multiple motors with one command.
	pub fn read_u8(&mut self, motor_id: u8, address: u16) -> Result<Response<u8>, TransferError> {
		let response = self.read(motor_id, address, 1)?;
		Ok(response.into())
	}

	/// Read 16 bit register from a specific motor.
	///
	/// This function will not work correctly if the motor ID is set to [`packet_id::BROADCAST`][crate::instructions::packet_id::BROADCAST].
	/// Use [`Self::sync_read`] to read from multiple motors with one command.
	pub fn read_u16(&mut self, motor_id: u8, address: u16) -> Result<Response<u16>, TransferError> {
		let response = self.read(motor_id, address, 2)?;
		Ok(response.into())
	}

	/// Read 32 bit register from a specific motor.
	///
	/// This function will not work correctly if the motor ID is set to [`packet_id::BROADCAST`][crate::instructions::packet_id::BROADCAST].
	/// Use [`Self::sync_read`] to read from multiple motors with one command.
	pub fn read_u32(&mut self, motor_id: u8, address: u16) -> Result<Response<u32>, TransferError> {
		let response = self.read(motor_id, address, 4)?;
		Ok(response.into())
	}
}
