struct Assembler;

impl Assembler {
	fn assemble(code: &str) -> &[u16] {
		match code {
			"CLEAR" => &[0x00E0],
			s => &[0x1_NNN],
			_ => todo!()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Assembler;

	#[test]
	fn assembles_clear_screen_as_00e0() {
		let asm = "CLEAR";

		let bytes = Assembler::assemble(asm);

		assert_eq!(bytes, [0x00E0] as [u16; 1]);
	}

	#[test]
	fn assembles_jump_as_1nnn() {
		let asm = "JUMP TO 15A";

		let bytes = Assembler::assemble(asm);

		assert_eq!(bytes, [0x1_15A] as [u16; 1]);
	}
}
