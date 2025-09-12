/*
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // -- snip --
}

fn function2() -> io::Repeat<()> {
    // -- snip --
}
 */
// Or

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result{
    // -- snip --
    Ok(())
}

fn function2() -> IoResult<()>{
    // -- snip --
    Ok(())
}