module DTRToRust
  module Aggregator
    # Aggregates instructions into loop blocks
    class LoopAggregator
      def initialize(instructions)
        @instructions = instructions
      end

      def self.aggregate(instructions)
        new(instructions).aggregate
      end

      def aggregate
        sequentially_group_by_loop
      end

      private

      def sequentially_group_by_loop
        return [] if @instructions.empty?

        loop_blocks = []
        current_loop_block = []
        instructions_with_loops = []
        cur_index = 0

        while cur_index < @instructions.length
          instruction = @instructions[cur_index]

          if instruction.instruction == 'label' && instruction.inputs[0].start_with?('loop_top')
            current_loop_block = []
            cur_index += 1

            while @instructions[cur_index].instruction != 'label' || !@instructions[cur_index].inputs[0].start_with?('loop_exit')
              current_loop_block << { type: :instruction, instruction: @instructions[cur_index] }
              cur_index += 1
            end

            cur_index += 1

            instructions_with_loops << { type: :loop, instructions: current_loop_block }
          else
            instructions_with_loops << { type: :instruction, instruction: }

            cur_index += 1
          end
        end

        # loop_blocks << current_loop_block unless current_loop_block.empty?

        instructions_with_loops
      end
    end
  end
end
