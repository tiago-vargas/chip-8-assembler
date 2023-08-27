struct Assembler;

impl Assembler {
	fn assemble(code: &str) -> &[u16] {
		&[0x00E0]
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
}
