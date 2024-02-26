mod having;

use std::fmt::Result;
// we can use `as` keyword to rename the imported item
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
