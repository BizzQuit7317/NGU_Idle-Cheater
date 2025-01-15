import pymem

# Function to scan the memory for a specific value and type
def find_addresses(game, target_value):
    matching_addresses = []
    
    # Get the memory ranges of the process (we're using a very broad range)
    # The start and end of the memory range is just an example. Adjust as needed.
    memory_range_start = 0x0000000000000000  # start of the memory space
    memory_range_end = 0x7fffffffffffffff  # end of the memory space (example limit)

    # Iterate through the memory ranges
    for address in range(memory_range_start, memory_range_end, 0x1000):  # Check every 4KB block
        try:
            # Read memory at the current address, if possible
            value = game.read_int(address)
            
            # Check if the value matches the target value
            if value == target_value:
                matching_addresses.append(address)
        except pymem.exception.MemoryReadError:
            # Handle any memory read errors gracefully
            pass

    return matching_addresses

# Attach to the game process
game = pymem.Pymem('NGUIdle.exe')
print(f"Attached to process: {game.process_id}")

# Set the target value you're looking for
target_value = 99999  # The value you're looking for in the process

# Find all addresses with this value
addresses = find_addresses(game, target_value)

# Print all matching addresses
if addresses:
    print(f"Found {len(addresses)} addresses with the value {target_value}:")
    for address in addresses:
        print(f"Address: {hex(address)}")
else:
    print(f"No addresses found with the value {target_value}.")
