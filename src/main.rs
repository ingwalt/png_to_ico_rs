use std::fs::{File};
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut _file = File::open("E:\\Backup\\桌面\\cgo\\syncFile\\default32.png")?;
    let mut buf :Vec<u8> = Vec::new();
     let len = _file.read_to_end(&mut buf)?;
     let data = png_to_icon(&buf);
    if let Ok(v) = data  {
        let mut f = File::create("test.ico")?;
        f.write(&v);
    }

     Ok(())
}

fn png_to_icon(pngData :&Vec<u8>) ->Result<Vec<u8>,&str> {
    match read_png_info(pngData) {
        Ok((width,height,depth,size))=>{
            let mut data = pngData.clone();
            let mut ret :Vec<u8>=Vec::new();
            let png_total_size :u32 = 0;
            let len_icon_dir :u32 = 6;
            let len_icon_dir_entry :u32 = 16;
            let len_all_icon_dir_entry :u32 = len_icon_dir_entry;
            let offset :u32 = len_icon_dir+len_all_icon_dir_entry+png_total_size;
            ret.append(& mut icon_dir(1));
            ret.append(& mut icon_dir_entry(width,height,depth,size,offset));
            ret.append(& mut data);

            Ok(ret)
        },
        Err(msg)=>{

            Err(msg)
        }
    }
    
}

fn read_png_info(data :&Vec<u8>) -> Result<(u32,u32,u16,u32),&str>{
    /*
		25byte PNG header - BigEndian
		00:	89 50 4e 47 0d 0a 1a 0a // 8byte - magic number
		IHDR chunk
		08:	xx xx xx xx // 4byte - chunk length
		12:	49 48 44 52 // 4byte - chunk type(IHDR)
		16:	xx xx xx xx // 4byte - width
		20:	xx xx xx xx // 4byte - height
		24:	xx          // 1byte - bit depth (bit/pixel)
	*/
    let header_len :usize = 25;
    if data.len() < header_len {
        return  Err("Not PNG");
    }
    let mut header:Vec<u8> = Vec::new();
    for idx in 0..header_len {
        header.push(data[idx]);
    }
    // 8byte header[0:8] - magic number
	let magic:Vec<u8> = vec![0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
    for idx in 0..magic.len() {
        if header[idx]!=magic[idx] {
            return  Err("Not PNG");
        }
    }

    // 4byte header[8:12] - chunk length - skipped

    // 4byte header[12:16] - chunk type IHDR  'I','H','D','R'
    let ihdr :Vec<u8> = vec![0x49,0x48,0x44,0x52];
    for idx in 0..ihdr.len() {
        if header[12+idx]!=ihdr[idx] {
            return  Err("PNG no IHDR chunk");
        }
    }
    // 4byte header[16:20] - width
    let width_arr:[u8;4]=[header[16],header[17],header[18],header[19]];
    let mut width:u32 = u8_array_to_u32(&width_arr);

    // 4byte header[20:24] - height
    let height_arr:[u8;4]=[header[20],header[21],header[22],header[23]];
    let mut height:u32 = u8_array_to_u32(&height_arr);

    if width<=256 && height <=256 {
        if width == 256 {
            width = 0;
        }
        if height == 256 {
            height = 0;
        }
    }else{
        return Err("Width and height cannot be larger than 256.");
    }
    let depth:u16 = u16::from(header[24]);
    let size = data.len() as u32;
    
    Ok((width,height,depth,size))
}

// ICONDIR - return ICONDIR byte array
fn icon_dir(num :u16) ->Vec<u8> {
    /*
		6byte ICONDIR - LittleEndian
		00:   00 00 // 2byte, must be 0
		02:   01 00 // 2byte, 1 for ICO
		04:   xx xx // 2byte, img number
	*/
    let mut ret = vec![0,0,1,0,0,0];
    let temp = num;
    let h4:u8 = (temp>>8) as u8;
    let h5 :u8 = (temp & 0x00FF) as u8 ;
    ret[4] = h5;
    ret[5] = h4;

    ret
}

fn icon_dir_entry(width:u32,height:u32,depth:u16,size:u32,offset:u32) ->Vec<u8> {
    /*
		16byte ICONDIRENTRY - LittleEndian
		00:   xx    // 1byte, width
		01:   xx    // 1byte, height
		02:   00    // 1byte, color palette number, 0 for PNG
		03:   00    // 1byte, reserved, always 0
		04:   00 00 // 2byte, color planes, 0 for PNG
		06:   xx xx // 2byte, color depth
		08:   xx xx xx xx // 4byte, image size
		12:   xx xx xx xx // 4byte, image offset
	*/
    let mut ret:Vec<u8> = vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    ret[0] = width as u8;
    ret[1] = height as u8;
    let depth_arr = u16_to_u8_arr(depth);
    ret[6]=depth_arr[1];
    ret[7]=depth_arr[0];
    let size_arr = u32_to_u8_arr(size);
    ret[8]=size_arr[3];
    ret[9]=size_arr[2];
    ret[10]=size_arr[1];
    ret[11]=size_arr[0];
    let offset_arr = u32_to_u8_arr(offset);
    ret[12]=offset_arr[3];
    ret[13]=offset_arr[2];
    ret[14]=offset_arr[1];
    ret[15]=offset_arr[0];

    ret
}

fn u8_array_to_u32(data:&[u8;4])->u32{
    
    let h1 :u32 = u32::from(data[0]);
    let h2 :u32 = u32::from(data[1]);
    let h3 :u32 = u32::from(data[2]);
    let h4 :u32 = u32::from(data[3]);

    h1<<24 | h2<<16 | h3<< 8 | h4
}

fn u32_to_u8_arr(data:u32)->[u8;4]{
    let mut ret:[u8;4]=[0,0,0,0];
    ret[0]=((data>>24)&0xFF) as u8;
    ret[1]=((data>>16)&0xFF) as u8;
    ret[2]=((data>>8)&0xFF) as u8;
    ret[3]=(data&0xFF) as u8;
    
    ret
}

fn u16_to_u8_arr(data:u16)->[u8;2]{
    let mut ret:[u8;2]=[0,0];
    ret[0]=((data>>8)&0xFF) as u8;
    ret[1]=(data&0xFF) as u8;    
    ret
}