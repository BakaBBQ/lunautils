/*
NOTE: this is only a mapping of my current thoughts

vm_assemblier.rs is responsible to construct the json instructions the
game will reference, the format will be in.

[[0,-1],[0,-1],[0,-1],[1,-1],[1,-1],[1,-1],[1,0]]

you can think this instruction set as bytecodes and the game will
essentially frame a vm for going thru the instructions.

the vm will initialize a simple int value that points to an element in this
array, say: p = 0 and now it will point to [0,-1]

pseudocode:

p = 0
every_frame {
  element = frames_vector[p][0] ; the element will be exposed for rendering, calc etc.
  inst = frames_vector[p][1]

  if(0 == inst)
    p++
  else
    p = inst
}

and this allows a simple state machine to be built upon clojure

the assemblier will probably output two files:
frame_inst.json, move_mapping.json

frame_inst.json will contain the vm instructions
move_mapping.json will contain a simple input -> frame-vm-address lookup table
*/
