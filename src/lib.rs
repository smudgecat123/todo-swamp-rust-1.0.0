pub mod parser;
pub mod query;
pub mod runner;
pub mod todo_list;
pub mod trie;

pub use todo_list::*;
pub use query::*;
pub use trie::*;

use std::io::{self, prelude::*};
use std::fs;

#[inline]
pub fn file_run(file_name: &str) -> io::Result<()> {
    let file_in = fs::File::open(format!("{}.in", file_name))?;
    let file_out = fs::File::create(format!("{}.out", file_name))?;

    let mut lines_in = io::BufReader::new(file_in).lines();
    let mut buffer_out = io::BufWriter::new(file_out);

    //let mut tl: TodoList = TodoList::new();
    //let mut tl: TriedoList<Trie1> = TriedoList::new();
    let mut tl: TriedoList<Trie4> = TriedoList::new();

    if let Some(Ok(_s)) = lines_in.next() {
        for line in lines_in {
            if let Ok(l) = line {
                if let Some(r) = runner::run_line(&l, &mut tl) {
                    writeln!(buffer_out, "{}", r)?;
                }
                else { //make bugs more apparent
                    panic!();
                }
            }
        }
    }
    Ok(())
}

#[inline]
pub fn standard_run() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut lines_in = stdin.lock().lines();
    let mut buffer_out = stdout.lock();

    let mut tl: TodoList = TodoList::new();
    if let Some(Ok(_s)) = lines_in.next() { //read first line as query count, loop on remaining lines
        for line in lines_in {
            if let Ok(l) = line {
                if let Some(r) = runner::run_line(&l, &mut tl) {
                    writeln!(buffer_out, "{}", r)?;
                }
            }
        }
        buffer_out.flush()?;
    }
    Ok(())
}
