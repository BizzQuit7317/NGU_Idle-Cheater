import pymem

def save_data(save_file, data):
    with open("save", "a") as f:
        f.write(f"{data}\n")

# Attach to the game process by its name
game = pymem.Pymem('NGUIdle.exe')
print(f"Attached to process: {game.process_id}")

"""
Current Address
Cap - 0x137A0E98420
"""

# Read value from a specific address
address = 0x137A0E98420# Replace with the actual address
value = game.read_int(address)
print(f"Current value at {hex(address)}: {value}")

# Write a new value
#new_value = 20
#game.write_int(address, new_value)
#print(f"New value at {hex(address)}: {new_value}")

i = 0
while True:
    current_offset = i * 0x10
    address += current_offset
    value = game.read_int(address)
    if value != 0:
        print(f"Current value at {hex(address)}: {value}")
        save_data("save.txt", f"Current value at {hex(address)}: {value} with offset of {i}")
    i += 1

#for i in [0x0, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70]:
#    address -= i
#    value = game.read_int(address)
#    print(f"Current value at {hex(address)}: {value}")


