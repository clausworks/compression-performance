use std::env;
use exitcode;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::io::Write;

/*
mod trie;
use trie::CodeTrie;
*/


///////////////////////////////////////////////////////////////////////////////
// COMMAND LINE
///////////////////////////////////////////////////////////////////////////////

const DEFAULT_RC: u32 = 12;

fn die_usage(_e: &str) -> ! {
    eprintln!("Usage: lzwCompress [-rN] file");
    eprintln!(concat!(
        "Where: N is a number from 9 to 15 specifying the recycle code ",
        "as a power of 2"
    ));
    //eprintln!("Error message: {}", e);
    std::process::exit(exitcode::USAGE);
}

fn parse_arg_rc(rc_arg: &str) -> Result<u32, &'static str> {
    let re = Regex::new(r"^-r\d{1,2}$").unwrap();

    match re.is_match(rc_arg) {
        false => Err("Invalid format for -rN"),
        true => {
            match rc_arg[2..].parse::<u32>() {
                Ok(v @ 9..=15) => Ok(v),
                Ok(_) => Err("Out of range"),
                Err(_) => Err("Could not parse u32 integer"),
            }
        }
    }
}

fn parse_args() -> Result<ProgramArgs, &'static str> {
    let args: Vec<String> = env::args().collect();
    //println!("Args: {:?}", args);

    match args.len() {
        2 => {
            Ok(ProgramArgs {rc: DEFAULT_RC, fname: args[1].clone()})
        },
        3 => {
            let rc = parse_arg_rc(&args[1])?;
            Ok(ProgramArgs {rc: rc, fname: args[2].clone()})
        },
        _ => Err("Invalid number of args")
    }
}

///////////////////////////////////////////////////////////////////////////////
// FILE I/O 
///////////////////////////////////////////////////////////////////////////////

/*
#[derive(Debug)]
struct ProgramState {
    rc: u32,
    f_in: File,
    f_out: File
}
*/

#[derive(Debug)]
struct ProgramArgs {
    rc: u32,
    fname: String,
}

fn open_io_files(fname_in: &str, fname_out: &str) -> Result<(File, File), std::io::Error>  {
    //println!("{fname_in}, {fname_out}");
    let f_in = File::open(fname_in)?;
    let f_out = File::create(fname_out)?;
    Ok((f_in, f_out))
}

fn init() -> Option<(u32, File, File)> {
    // Ensure arguments are valid
    match parse_args() {
        Ok(args) => {
            let (f_in, f_out) = open_io_files(&args.fname, &[&args.fname, ".lzw"].join(""))
                        .unwrap();
            Some((args.rc, f_in, f_out))
        },
        Err(e) => {
            die_usage(e);
        }
    }
}

/*
fn copy_files(f_in: &mut File, f_out: &mut File) -> Result<(), std::io::Error> {
    let mut buf = vec![0u8]; // single byte buffer
    for byte in f_in.bytes() {
        buf[0] = byte?;
        f_out.write_all(&buf)?;
    }
    Ok(())
}
*/

fn write_byte(f_out: &mut File, b: u8) -> Result<(), std::io::Error> {
    //println!("WRITE BYTE >> {0:08b}:{0:02x}:{0}", b);
    f_out.write_all(&[b])?;
    Ok(())
}


///////////////////////////////////////////////////////////////////////////////
// CODETRIE 
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Node {
    code: u16,
    next: [u16; CodeTrie::ALPHABET_SIZE] // next value (byte) in sequence
}

#[derive(Debug)]
pub struct CodeTrie {
    //size: u16, // number of nodes allocated
    //next_index: u16, // next available node (index)
    next_code: u16, // next unused code, after alphabet and end-of-data (EOD)
    nodes: Vec<Node>,
}

impl CodeTrie {
    const ALPHABET_SIZE: usize = 256;
    const EOD_CODE: u16 = 256;  // should be same as ALPHABET_SIZE
    //const DEFAULT_TRIE_SIZE: u16 = 256;

    /// Create a new trie to store codes.
    pub fn new(/*size: u16*/) -> CodeTrie {
        CodeTrie {
            next_code: CodeTrie::EOD_CODE + 1, // code EOD reserved
            nodes: vec![
                // Root node
                Node {
                    code: 0,
                    next: [0; CodeTrie::ALPHABET_SIZE]
                }
            ]
        }
    }

    /// Add an entry for a length-N string (N>1), i.e., one whose code is
    /// created on demand.
    pub fn add_entry(&mut self, b: u8, cur_node: usize) {
        // Index of new node
        self.nodes[cur_node].next[b as usize] = self.nodes.len() as u16;
        // Create new node
        self.nodes.push(Node {
            code: self.next_code,
            next: [0; CodeTrie::ALPHABET_SIZE]
        });

        println!("Added code to dictionary: {}", self.next_code);

        self.next_code += 1;
    }

    /// Add an entry for a length-1 string, i.e., one whose code is the raw
    /// character value.
    pub fn add_entry_alphabetic(&mut self, b: u8) {
        // Check if code is already in dictionary
        if self.nodes[0].next[b as usize] == 0 {
            // Index of new node
            self.nodes[0].next[b as usize] = self.nodes.len() as u16;
            // Create new node
            self.nodes.push(Node {
                code: b as u16,
                next: [0u16; CodeTrie::ALPHABET_SIZE]
            });
            // Don't increment next_code. The raw value is the code.
        }
    }

    /*
    pub fn reset(&mut self) {
        self.next_code = CodeTrie::EOD_CODE + 1; // code EOD reserved
        self.nodes = Vec::new();
    }
    */
}

///////////////////////////////////////////////////////////////////////////////
// OUTPUT BUFFER & BIT TWIDDLING
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct PackedBuf {
    bit_buf: u32,
    bit_count: u32,
    packet_size: u32
}

impl PackedBuf {
    const BYTE_BITS: u32 = 8;
    const DEFAULT_PACKET_SIZE: u32 = 9;

    fn new() -> PackedBuf {
        PackedBuf {
            bit_buf: 0,
            bit_count: 0,
            packet_size: Self::DEFAULT_PACKET_SIZE
        }
    }

    fn push(&mut self, code: u16) {
        self.bit_buf <<= self.packet_size;
        self.bit_buf += code as u32;
        self.bit_count += self.packet_size;
    }

    /// Returns true if buffer can be emptied by get_leftover_byte
    fn almost_empty(&self) -> bool {
        self.bit_count <= Self::BYTE_BITS // FIXME: bug? Use `<` instead? Appears in C program.
    }

    fn get_next_byte(&mut self) -> u8 {
        assert!(self.bit_count >= Self::BYTE_BITS);
        let mask = 0xFFu32 << (self.bit_count - Self::BYTE_BITS);
        let mut ret = self.bit_buf & mask;
        ret >>= self.bit_count - Self::BYTE_BITS;
        self.bit_buf &= !mask;
        self.bit_count -= Self::BYTE_BITS;
        ret as u8
    }

    fn get_leftover_byte(&mut self) -> Option<u8> {
        assert!(self.bit_count <= Self::BYTE_BITS);
        if self.bit_count == 0 {
            None
        }
        else {
            let mask = (1 << self.bit_count) - 1;
            let mut ret = mask & self.bit_buf;
            ret <<= Self::BYTE_BITS - self.bit_count;
            self.bit_buf = 0;
            self.bit_count = 0;
            Some(ret as u8)
        }
    }

    fn update_packet_size(&mut self, code: u16) {
        self.packet_size = {
            if      code < (1 <<  9) {  9 }
            else if code < (1 << 10) { 10 }
            else if code < (1 << 11) { 11 }
            else if code < (1 << 12) { 12 }
            else if code < (1 << 13) { 13 }
            else if code < (1 << 14) { 14 }
            else if code < (1 << 15) { 15 }
            else { 16 }  // 16 always causes a recycle
        };
    }
}

///////////////////////////////////////////////////////////////////////////////
// MAIN LZW ALGORITHM 
///////////////////////////////////////////////////////////////////////////////


enum CompressionResult {
    EmptyFile,
    NonemptyFile
}

#[derive(Debug)]
struct Compressor {
    dict: CodeTrie,
    out_buf: PackedBuf,
    f_out: File,
    f_in: File,
    rc: u32,
}

impl Compressor {
    fn new(rc: u32, f_in: File, f_out: File) -> Compressor {
        Compressor {
            dict: CodeTrie::new(),
            out_buf: PackedBuf::new(),
            f_in: f_in,
            f_out: f_out,
            rc: rc,
        }
    }

    fn compress(&mut self) -> Result<CompressionResult, std::io::Error> {
        let mut i = 0usize; // current node index 
        let mut buf = vec![0u8]; // single byte buffer

        // Read the first byte to see if the file is empty
        if self.f_in.read(&mut buf)? == 0 {
            return Ok(CompressionResult::EmptyFile); // immediate EOF -- empty file
        }

        // Write initial byte (recycle code) for non-empty files
        write_byte(&mut self.f_out, self.rc as u8)?;

        // There are more efficient ways to do this (e.g. using a buffered
        // reader/writer), but the C implementation does byte-by-byte, so we'll
        // do that for comparability.
        loop {
            let b = buf[0];
            // update alphabetic index
            self.dict.add_entry_alphabetic(b);
            // reached unknown sequence
            if self.dict.nodes[i].next[b as usize] == 0 {
                // write known portion of sequence
                self.write_known(b, i)?;
                // reset sequence to the byte that made it unknown
                i = self.dict.nodes[0].next[b as usize] as usize;
            }
            // current sequence is known: advance
            else {
                i = self.dict.nodes[i].next[b as usize] as usize;
            }

            // Read new byte for next iteration
            if self.f_in.read(&mut buf)? == 0 {
                break; // EOF
            }
        }

        self.finish_write(i)?;

        Ok(CompressionResult::NonemptyFile)
    }

    // TODO: clean up this mess (writeKnown, finishWrite, etc.)
    fn write_packed_code(&mut self, code: u16) -> Result<(), std::io::Error> {
        println!("Writing code {0} ({1} bits)", code, self.out_buf.packet_size);
        //println!("  pre push:      {:032b}", self.out_buf.bit_buf);
        self.out_buf.push(code);
        //println!("  post push:     {:032b}", self.out_buf.bit_buf);
        while !self.out_buf.almost_empty() {
            let b = self.out_buf.get_next_byte();
            //println!("  after removal: {:032b}", self.out_buf.bit_buf);
            write_byte(&mut self.f_out, b)?;
        }
        Ok(())
    }

    fn write_known(&mut self, b: u8, i: usize) -> Result<(), std::io::Error> {
        self.write_packed_code(self.dict.nodes[i].code)?;
        self.out_buf.update_packet_size(self.dict.next_code);
        if !self.recycle(b) {
            self.dict.add_entry(b, i);
        }
        Ok(())
    }

    fn finish_write(&mut self, i: usize) -> Result<(), std::io::Error> {
        self.write_packed_code(self.dict.nodes[i].code)?;
        self.out_buf.update_packet_size(self.dict.next_code);
        self.recycle(0); // null byte, workaround ??
        self.write_packed_code(CodeTrie::EOD_CODE)?;
        if let Some(b) = self.out_buf.get_leftover_byte() {
            write_byte(&mut self.f_out, b)?;
        }
        Ok(())
    }

    fn recycle(&mut self, b: u8) -> bool {
        if self.out_buf.packet_size > self.rc {
            self.dict = CodeTrie::new();
            self.dict.add_entry_alphabetic(b);
            self.out_buf.update_packet_size(self.dict.next_code);
            true
        }
        else {
            false
        }
    }

}


///////////////////////////////////////////////////////////////////////////////
// MAIN 
///////////////////////////////////////////////////////////////////////////////

fn main() {
    let (rc, f_in, f_out) = init().expect("init() failed"); // should always work
    //copy_files(&mut (prog_state.f_in), &mut (prog_state.f_out)).unwrap();
    //test_trie();
    let mut compressor = Compressor::new(rc, f_in, f_out);

    compressor.compress().unwrap();
}
