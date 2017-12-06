$DEBUG = nil

def run_until_loop_detected(blocks, cycles, previous_states)
  puts "blocks: #{blocks.inspect}, cycle: #{cycles + 1}, previous_states: #{previous_states.inspect}" if $DEBUG

  if previous_index = previous_states.find_index(blocks)
    puts "Loop of size #{cycles - previous_index} found in #{cycles} cycles!"
    cycles
  else
    updated_blocks = blocks.dup
    target_index = blocks.find_index(blocks.max)
    blocks_to_reallocate, updated_blocks[target_index] = updated_blocks[target_index], 0

    blocks_to_reallocate.times do
      target_index = (target_index + 1) % updated_blocks.size
      updated_blocks[target_index] += 1
    end

    run_until_loop_detected(updated_blocks, cycles + 1, previous_states << blocks)
  end
end

# run_until_loop_detected([0, 2, 7, 0], 0, [])
run_until_loop_detected([2,8,8,5,4,2,3,1,5,5,1,2,15,13,5,14], 0, [])
