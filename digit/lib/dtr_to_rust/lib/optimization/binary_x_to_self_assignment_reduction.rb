# frozen_string_literal: true

module DTRToRust
  module Optimization
    # This class is responsible for reducing binary operations that assign the result to the same variable
    class BinaryXToSelfAssignmentReduction
      def initialize(instructions)
        @instructions = instructions
        @cur_instruction = nil
        @optimized_instructions = []
        @memoize_assigns = {}
        @to_remove = {}
        @last_was_eavluate = nil
      end

      def self.apply(instructions)
        new(instructions).apply
      end

      def apply
        @instructions.each_with_index do |instruction, index|
          @cur_instruction = instruction

          if skip_instruction?
            @optimized_instructions << @cur_instruction
            next unless @cur_instruction.assign && @cur_instruction.assign == @cur_instruction.assign.upcase

            @memoize_assigns[@cur_instruction.assign] = {
              inputs: @cur_instruction.inputs,
              index:
            }
            next
          end

          apply_to_instruction(index)
        end
        actually_optimized_instructions = []
        @optimized_instructions.each_with_index do |instruction, index|
          actually_optimized_instructions << instruction unless @to_remove[index]
        end

        actually_optimized_instructions
      end

      private

      def skip_instruction?
        !%w[add subtract divide multiply].include?(@cur_instruction.instruction)
      end

      def apply_to_instruction(index)
        match_on0 = @cur_instruction.inputs[0] == @cur_instruction.assign
        match_on1 = @cur_instruction.inputs[1] == @cur_instruction.assign

        unless @cur_instruction.inputs.length == 2 &&
               (match_on1 || match_on0)

          @optimized_instructions << @cur_instruction
          return
        end

        optimized_inputs = []

        if @memoize_assigns[@cur_instruction.inputs[0]]
          optimized_inputs << @memoize_assigns[@cur_instruction.inputs[0]][:inputs]
          @to_remove[@memoize_assigns[@cur_instruction.inputs[0]][:index]] = true
        else
          optimized_inputs << @cur_instruction.inputs[0]
        end

        if @memoize_assigns[@cur_instruction.inputs[1]]
          optimized_inputs << @memoize_assigns[@cur_instruction.inputs[1]][:inputs]
          @to_remove[@memoize_assigns[@cur_instruction.inputs[1]][:index]] = true
        else
          optimized_inputs << @cur_instruction.inputs[1]
        end

        optimized_inputs.flatten!
        assignment = match_on0 ? optimized_inputs[0] : optimized_inputsinputs[1]

        @optimized_instructions << DTRCore::Instruction.new(@cur_instruction.instruction, optimized_inputs, assignment,
                                                            @cur_instruction.scope)

        return unless @cur_instruction.assign && @cur_instruction.assign == @cur_instruction.assign.upcase

        @memoize_assigns[@cur_instruction.assign] = {
          inputs: optimized_inputs,
          index:
        }
      end
    end
  end
end
