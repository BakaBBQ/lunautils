// NOP LIT DEL JMP COND JAS GNP GCP ADD SUB MULT DIV TEX GPX GPY GVX GVY SPX SPY SVX SVY SAX SAY GIP END
pub static NOP: i32 = 0x00;
pub static LIT: i32 = 0x01;
pub static DEL: i32 = 0x02;
pub static JMP: i32 = 0x03;

pub static COND: i32 = 0x04;
pub static JAS: i32 = 0x05;
pub static GNP: i32 = 0x06;
pub static GCP: i32 = 0x07;

pub static ADD: i32 = 0x10;
pub static SUB: i32 = 0x11;
pub static MULT: i32 = 0x12;
pub static DIV: i32 = 0x13;

pub static TEX: i32 = 0x30;

pub static GPX: i32 = 0x40;
pub static GPY: i32 = 0x41;

pub static GVX: i32 = 0x42;
pub static GVY: i32 = 0x43;

pub static SPX: i32 = 0x46;
pub static SPY: i32 = 0x47;

pub static SVX: i32 = 0x48;
pub static SVY: i32 = 0x48;

pub static SAX: i32 = 0x50;
pub static SAY: i32 = 0x51;

pub static GIP: i32 = 0x60;
pub static END: i32 = 0xff;

/*
enum Inst {
  NOP = 0x00,
  LIT = 0x01,
  DEL = 0x02,
  JMP = 0x03,

  COND = 0x04, // conditional jump: performs as jmp if the top two stacks are equal, otherwise continues
  JAS = 0x05, // jump according to stack, functions similarly to jmp, but reads the address from the stack
  GNP = 0x06, // pops the next pointer: (pointer + 1)
  GCP = 0x07, // pops the current pointer

  // math operations
  ADD = 0x10,
  SUB = 0x11,
  MULT = 0x12,
  DIV = 0x13,

  //hooks
  TEX = 0x30, // change current texture index, also changes the character's hitbox data

  //physics stuff. getters, pops the value. Velocities has nothing to do with current frames
  GPX = 0x40,
  GPY = 0x41,

  GVX = 0x42, // vx
  GVY = 0x43, // vy

  GAX = 0x44,
  GAY = 0x45,

  //setters
  SPX = 0x46,
  SPY = 0x47,

  SVX = 0x48,
  SVY = 0x49,

  SAX = 0x50,
  SAY = 0x51,

  //input
  GIP = 0x60,

  END = 0xff, //end this frame
}
*/
