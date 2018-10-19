#![feature(try_from)]

use std::convert::TryFrom;

#[derive(Debug)]
enum Section<'a> {
    Custom(&'a [u8]),
    Type(&'a [u8]),
    Import(&'a [u8]),
    Function(&'a [u8]),
    Table(&'a [u8]),
    Memory(&'a [u8]),
    Global(&'a [u8]),
    Export(&'a [u8]),
    Start(&'a [u8]),
    Element(&'a [u8]),
    Code(Result<Vec<&'a [u8]>, String>),
    Data(&'a [u8]),
    Unknown(u8, &'a [u8]),
}

impl<'a> TryFrom<(u8, &'a [u8])> for Section<'a> {
    type Error = String;

    fn try_from((id, data): (u8, &'a [u8])) -> Result<Self, Self::Error> {
        Ok(match id {
            0 => Section::Custom(data),
            1 => Section::Type(data),
            2 => Section::Import(data),
            3 => Section::Function(data),
            4 => Section::Table(data),
            5 => Section::Memory(data),
            6 => Section::Global(data),
            7 => Section::Export(data),
            8 => Section::Start(data),
            9 => Section::Element(data),
            10 => Section::Code(parse_code_section(data)),
            11 => Section::Data(data),
            id => Section::Unknown(id, data),
        })
    }
}

fn expect(expected: u8, input: &[u8]) -> Result<&[u8], String> {
    let (&first, rest) = input.split_first().ok_or_else(|| {
        format!("Expected {:#x}, found nothing", expected)
    })?;
    if first == expected {
        Ok(rest)
    } else {
        Err(format!("Expected {:#x}, found {:#x}", expected, first))
    }
}

fn read_u32(mut input: &[u8]) -> Result<(u32, &[u8]), String> {
    let mut result = 0;
    loop {
        let (&first, rest) = match input.split_first() {
            Some(t) => t,
            None => return Err("EOF while reading u32".into()),
        };
        input = rest;

        if (first & 0x80) == 0 {
            result = first as u32;
            break;
        }
        panic!("{:#x}", first);
    }
    Ok((result, input))
}

fn parse_code_section(mut input: &[u8]) -> Result<Vec<&[u8]>, String> {
    println!("1. Input = {:?}", input);

    let (length, rest) = read_u32(input)?;
    input = rest;
    println!("2. Input = {:?}", input);

    let mut result = Vec::with_capacity(length as usize);
    for i in 0..length {
        let (codesize, rest) = read_u32(input)?;
        input = rest;
        result.push(&input[..codesize as usize]);
        input = &input[codesize as usize..];
        println!("{}. Input = {:?}", i + 3, input);
    }
    Ok(result)
}

fn parse(mut input: &[u8]) -> Result<Vec<Section>, String> {
    input = expect(0, input)?;
    input = expect(b'a', input)?;
    input = expect(b's', input)?;
    input = expect(b'm', input)?;

    input = expect(1, input)?;
    input = expect(0, input)?;
    input = expect(0, input)?;
    input = expect(0, input)?;

    let mut result = vec![];

    loop {
        let (&id, rest) = match input.split_first() {
            Some(t) => t,
            None => return Ok(result),
        };
        input = rest;

        let (len, rest) = read_u32(input)?;
        input = rest;
        let len = len as usize;

        if rest.len() < len {
            return Err(format!("Section is too short! {} < {}", rest.len(), len));
        }
        result.push(Section::try_from((id, &input[..len]))?);
        input = &input[len..];
    }
}

fn main() {
    println!("BROKEN: {:#?}", parse(&[
        0, 97, 115, 109,
        1, 0, 0, 0,

        1, 8,
        2, 96, 1, 124, 0, 96, 0, 0,

        3, 3,
        2, 0, 1,

        4, 5,
        1, 112, 1, 1, 1,

        5, 4,
        1, 1, 4, 8,

        6, 18,
        2, 127, 1, 65, 7, 11, 124, 1, 68, 51, 51, 51, 51, 51, 51, 243, 63, 11,

        7, 48,
        6, 2, 102, 110, 0, 0, 3, 102, 110, 50, 0, 1, 5, 116, 97, 98, 108, 101, 1, 0, 6, 103, 108, 111, 98, 97, 108, 3, 0, 7, 103, 108, 111, 98, 97, 108, 50, 3, 1, 6, 109, 101, 109, 111, 114, 121, 2, 0,

        10, 9,
        2,
          3,
            0, 11, 11,
          3,
            0, 11, 11,

        0, 17,
        4,
          110, 97, 109, 101, // b"name"
        1, 10,
          2,
          0, 2, 102, 110, // 0 => b"fn"
          1, 3, 102, 110, 50 // 1 => b"fn2"
    ]));
    println!("OK: {:#?}", parse(&[
        0, 97, 115, 109,
        1, 0, 0, 0,

        1, 8,
        2, 96, 1, 124, 0, 96, 0, 0,

        3, 3,
        2, 0, 1,

        4, 5,
        1, 112, 1, 1, 1,

        5, 4,
        1, 1, 4, 8,

        6, 18,
        2, 127, 1, 65, 7, 11, 124, 1, 68, 51, 51, 51, 51, 51, 51, 243, 63, 11,

        7, 48,
        6, 2, 102, 110, 0, 0, 3, 102, 110, 50, 0, 1, 5, 116, 97, 98, 108, 101, 1, 0, 6, 103, 108, 111, 98, 97, 108, 3, 0, 7, 103, 108, 111, 98, 97, 108, 50, 3, 1, 6, 109, 101, 109, 111, 114, 121, 2, 0,

        10, 7,
        2,
          2,
            0, 11,
          2,
            0, 11,

        0, 15,
        4,
          110, 97, 109, 101, // b"name"
        2,
          2, 102, 110, 0,
          3, 102, 110, 50, 0
    ]));
    println!("Hello,  world!");
}
