use std::collections::{HashMap, LinkedList};


use super::{nbtvalue::NbtValue, error::NbtError};



pub(crate) struct NbtReader{
    value: NbtValue
}

impl NbtReader{
    pub(crate) fn read_bytes(bytes: &[u8]) -> Result<NbtValue, NbtError>{
        
        let mut compound_map_list = LinkedList::new(); 
        let mut position = 0;
        while position < bytes.len(){
            let byte = bytes[position];

            position += 1;
            let (tag_name, tag_value) = match_nbt_value(&mut position, bytes, byte, true, &mut compound_map_list)?;
            if let Some(tag_value) = tag_value{
                compound_map_list.back_mut().unwrap().1.insert(tag_name.map_or(String::new(), |v| v), tag_value);
            }
           
            dbg!(&compound_map_list);
        }
        Ok(NbtValue::Compound(compound_map_list.pop_back().unwrap().1))
    }
}

type CompoundMapList = LinkedList<(Option<String>, HashMap<String, NbtValue>)>;

fn match_nbt_value(position: &mut usize, bytes: &[u8], tag_id: u8, find_name: bool, compound_map_list: &mut CompoundMapList) -> Result<(Option<String>, Option<NbtValue>), NbtError>{
    match tag_id{
        0 => {
            let (tag_name, tag_value) = (compound_map_list.len() > 1).then(||compound_map_list.pop_back().ok_or(NbtError::NBTFormatError)).ok_or(NbtError::NBTFormatError)??;
            compound_map_list.back_mut().ok_or(NbtError::NBTFormatError)?.1.insert(tag_name.map_or(String::new(), |v| v), NbtValue::Compound(tag_value));
            Ok((None, None))
        }
        1 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            Ok((tag_name, Some(NbtValue::Byte(read_byte(position, bytes)?))))
        }
        2 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            Ok((tag_name, Some(NbtValue::Short(read_short(position, bytes)?))))
        }
        3 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            Ok((tag_name, Some(NbtValue::Int(read_int(position, bytes)?))))
        }
        4 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            Ok((tag_name, Some(NbtValue::Long(read_long(position, bytes)?))))
        }
        5 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            Ok((tag_name, Some(NbtValue::Float(read_float(position, bytes)?))))
        }
        6 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            Ok((tag_name, Some(NbtValue::Double(read_double(position, bytes)?))))
        }
        7 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            Ok((tag_name, Some(NbtValue::ByteArray(read_byte_array(position, bytes)?))))
        }
        8 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            Ok((tag_name, Some(NbtValue::String(read_string(position, bytes)?))))
        }
        9 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            let values = read_list(position, bytes, None, compound_map_list)?;
            Ok((tag_name, Some(NbtValue::List(values))))
        }
        10 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            compound_map_list.push_back((tag_name, HashMap::new()));
            Ok((None, None))
        }
        11 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            let values = read_int_array(position, bytes)?;
            Ok((tag_name, Some(NbtValue::IntArray(values))))
        }
        12 => {
            let tag_name = if find_name { Some(read_string(position, bytes)?) } else { None };
            let values = read_long_array(position, bytes)?;
            Ok((tag_name, Some(NbtValue::LongArray(values))))
        }
        _ => {
            Err(NbtError::NBTFormatError)
        }
    }

}


fn read_byte(position: &mut usize, bytes: &[u8]) -> Result<i8, NbtError>{
    let value = bytes.get(*position).map_or(Err(NbtError::NBTFormatError), |b| Ok(*b as i8));
    *position += 1;
    value
}

fn read_short(position: &mut usize, bytes: &[u8]) -> Result<i16, NbtError>{
    let value = bytes.get(*position..=*position+1).map_or(Err(NbtError::NBTFormatError), |b| Ok(i16::from_be_bytes(b.try_into().map_err(|_| NbtError::NBTFormatError)?)));
    *position += 2;
    value
}

fn read_int(position: &mut usize, bytes: &[u8]) -> Result<i32, NbtError>{
    let value = bytes.get(*position..=*position+3).map_or(Err(NbtError::NBTFormatError), |b| Ok(i32::from_be_bytes(b.try_into().map_err(|_| NbtError::NBTFormatError)?)));
    *position += 4;
    value
}

fn read_long(position: &mut usize, bytes: &[u8]) -> Result<i64, NbtError>{
    let value = bytes.get(*position..=*position+7).map_or(Err(NbtError::NBTFormatError), |b| Ok(i64::from_be_bytes(b.try_into().map_err(|_| NbtError::NBTFormatError)?)));
    *position += 8;
    value
}

fn read_float(position: &mut usize, bytes: &[u8]) -> Result<f32, NbtError>{
    let value = bytes.get(*position..=*position+3).map_or(Err(NbtError::NBTFormatError), |b| Ok(f32::from_be_bytes(b.try_into().map_err(|_| NbtError::NBTFormatError)?)));
    *position += 4;
    value
}

fn read_double(position: &mut usize, bytes: &[u8]) -> Result<f64, NbtError>{
    let value = bytes.get(*position..=*position+7).map_or(Err(NbtError::NBTFormatError), |b| Ok(f64::from_be_bytes(b.try_into().map_err(|_| NbtError::NBTFormatError)?)));
    *position += 8;
    value
}

fn read_byte_array(position: &mut usize, bytes: &[u8]) -> Result<Vec<i8>, NbtError>{
    let item_count = read_int(position, bytes)?  as usize;
    let value = bytes.get(*position..*position+item_count).map_or(Err(NbtError::NBTFormatError), |b| Ok(b.iter().map(|n| *n as i8).collect::<Vec<i8>>()));
    *position += item_count;
    value
}

fn read_string(position: &mut usize, bytes: &[u8]) -> Result<String, NbtError>{
    let length = u16::from_be_bytes(bytes[*position..=*position+1].try_into().unwrap());
    *position += 2;
    let name = String::from_utf8(bytes[*position..*position+length as usize].to_vec())?;
    *position += length as usize;
    Ok(name)
}

fn read_list(position: &mut usize, bytes: &[u8], list_type: Option<u8>, compound_map_list: &mut CompoundMapList) -> Result<Vec<NbtValue>, NbtError>{
    let list_type = list_type.map_or(read_byte(position, bytes)? as u8, |x| x);
    let item_count = read_int(position, bytes)? as usize;
    let mut values = Vec::with_capacity(item_count);
    for _ in 0..item_count{
        values.push(match_nbt_value(position, bytes, list_type, false, compound_map_list)?.1.ok_or(NbtError::NBTCastError)?)
    }
    *position += item_count;
    Ok(values)
}

fn read_int_array(position: &mut usize, bytes: &[u8]) -> Result<Vec<i32>, NbtError>{
    let item_count = read_int(position, bytes)? as usize;
    let mut values = Vec::with_capacity(item_count);
    for _ in 0..item_count{
        values.push(read_int(position, bytes)?);
    }
    *position += item_count;
    Ok(values)
}

fn read_long_array(position: &mut usize, bytes: &[u8]) -> Result<Vec<i64>, NbtError>{
    let item_count = read_int(position, bytes)? as usize;
    let mut values = Vec::with_capacity(item_count);
    for _ in 0..item_count{
        values.push(read_long(position, bytes)?);
    }
    *position += item_count;
    Ok(values)
}