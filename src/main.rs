extern crate minifb;
extern crate font8x8;

use std::time::{Duration, Instant};

use font8x8::legacy::BASIC_LEGACY; 
use font8x8::legacy::CONTROL_LEGACY; 

//extern crate png;

//use std::fs::File;

const MIX_CHANNELS: usize = 26;


use minifb::{CursorStyle, MouseButton, MouseMode, Window, Key, Scale, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

struct TextBlock {
  text: Vec<u8>,
  x: usize,
  y: usize,
  right: usize,
  bottom: usize,
}

impl TextBlock {
  fn new(text: String, x: usize, y: usize) -> TextBlock {
    TextBlock{ text: text.as_bytes().to_vec(), x: x , y: y, right: x + text.len() * 8  , bottom :y+8 }
  }
}





fn main() {

    let mut collision: Vec<usize> = vec![0; 100];

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut channel_buffers: Vec<u32> = vec![0; 56 * 128 * 14];
    let mut mouse_buffer: Vec<usize> = vec![0;100];

    let mut text_blocks: Vec<TextBlock> = vec![];

    text_blocks.push( TextBlock::new( "FILE".to_string(), 0,0));

    // for x in (0..26) {
    //     text_blocks.push( TextBlock::new( format!("CHAN {}",x), x*8*8+36, 200));
    // }

    
    let mut window = match Window::new("Scriblr - Press ESC to exit", WIDTH, HEIGHT,
                                       WindowOptions {
                                           scale: Scale::X1,
                                           ..WindowOptions::default()
                                       }) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };
    let size = 5;

    let mut mouse_down = false;
    let mut action = true;
    let mut menup = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        let start = Instant::now();

        window.get_mouse_pos(MouseMode::Discard).map(|mouse| {
            let screen_pos = ((mouse.1 as usize) * WIDTH) + mouse.0 as usize;
            //println!("{:?}", window.get_unscaled_mouse_pos(MouseMode::Discard).unwrap());

            if window.get_mouse_down(MouseButton::Left) {
              // start buffer?

              // am I inside the block?
              
              for b in text_blocks.iter() {
                if (mouse.0 as usize) >= b.x && (mouse.0 as usize) <= b.right
                  && (mouse.1 as usize) >= b.y && (mouse.1 as usize) <= b.bottom{
                    menup = true;
                    println!("menu should be up");
                  }else{
                    menup = false;
                  }
              }

              if ! mouse_down {
                mouse_buffer.clear();
                mouse_down = true;
              }

              // if menup {
              //   text_blocks.push( TextBlock::new( "NEW".to_string(), 0,8));
              //   text_blocks.push( TextBlock::new( "SAVE".to_string(), 0,16));
              //   text_blocks.push( TextBlock::new( "SAVE AS ...".to_string(), 0,16));
              //   text_blocks.push( TextBlock::new( "EXIT".to_string(), 0,24));
              // }else{
              //   text_blocks.pop();
              //   text_blocks.pop();
              //   text_blocks.pop();
              //   text_blocks.pop();
              // }

              // text_blocks[0].text = format!("{:.*}",2,mouse.0).as_bytes().to_vec();
              // text_blocks[1].text = format!("{:.*}",2,mouse.1).as_bytes().to_vec();

              mouse_buffer.push(screen_pos);
              // if window.is_key_down(Key::LeftShift) {
              //   buffer[screen_pos] = 0;
              // }else {
              buffer[screen_pos] = 0x00aaaaaa;
              
           
              // buffer[screen_pos+WIDTH] = 0x00ff0000;
              
              // for m in 0..size {
              //   for n in 0..size {
              //     buffer[screen_pos+(WIDTH*n)+m] = 0x00ffffff;
              //     buffer[screen_pos-(WIDTH*n)-m] = 0x00ffffff;
              //   }
              // }
              //action = true;
          
              window.set_cursor_style(CursorStyle::Crosshair);
              
            }else{ // handle mouse up
              if mouse_down {
                //text_blocks[1].text = format!("VECTOR LENGTH {}",&mouse_buffer.len()).as_bytes().to_vec();

                for x in &mouse_buffer {
                  buffer[*x]   = 0x00ff0000;
                }
              }
            }
            if window.get_mouse_down(MouseButton::Right) {
              buffer[screen_pos] = 0;
            }
            
        });

        // window.get_scroll_wheel().map(|scroll| {
        //     println!("Scrolling {} - {}", scroll.0, scroll.1);
        // });

        // We unwrap here as we want this code to exit if it fails
        //window.update_with_buffer(&buffer).unwrap();
        //buffer.
        if action {
          for tb in text_blocks.iter() {
                let width_offset = (WIDTH * tb.y) + tb.x;
                //println!("{}",tb.text.to_string());
                for itd in 0..8 {
                  let mut charoffset = itd * WIDTH;
                  for c in tb.text.clone() {
                    let x = &BASIC_LEGACY[c as usize][itd];
                    for bit in 0..8 {
                      match *x & 1 << bit {
                          0 => buffer[(width_offset + charoffset + bit) as usize] = 0x00000000,
                          _ => buffer[(width_offset + charoffset + bit) as usize] = 0x00ffffff, 
                      }
                    }
                  
                    charoffset += 8;
                    
                  }
                }
              }
          window.update_with_buffer(&buffer).unwrap();
          println!("RenderLOOP took {:?}", start.elapsed().as_micros());
        }        
    }
}
