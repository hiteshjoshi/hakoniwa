use std::ffi::CString;

use crate::Result;

pub fn exec<SA: AsRef<str>>(prog: &str, argv: &[SA]) -> Result<()> {
    let prog = CString::new(prog).unwrap_or_default();
    let argv: Vec<_> = argv
        .iter()
        .map(|arg| CString::new(arg.as_ref()).unwrap_or_default())
        .collect();
    let env: [CString; 0] = [];
    super::syscall::execve(&prog, &argv, &env)
}
