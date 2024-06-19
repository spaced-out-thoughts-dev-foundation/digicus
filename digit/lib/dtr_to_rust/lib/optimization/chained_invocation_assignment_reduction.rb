# frozen_string_literal: true

module DTRToRust
  module Optimization
    # Optimizes the instructions by reducing chained invocation assignments
    class ChainedInvocationAssignmentReduction
      def initialize(instructions)
        @instructions = instructions
        @cur_instruction = nil
        @optimized_instructions = []
        @memoize_assigns = {}
        @to_remove = {}
        @last_was_eavluate = nil
      end

      def apply
        @instructions.each_with_index do |instruction, index|
          @cur_instruction = instruction
          apply_to_instruction(index)
        end
        actually_optimized_instructions = []
        @optimized_instructions.each_with_index do |instruction, index|
          actually_optimized_instructions << instruction unless @to_remove[index]
        end

        actually_optimized_instructions
      end

      def self.apply(instructions)
        new(instructions).apply
      end

      def apply_to_instruction(index)
        @optimized_inputs = []

        @cur_instruction.inputs.each do |input|
          apply_to_instruction_input(input)
        end
        @optimized_instructions << DTRCore::Instruction.new(@cur_instruction.instruction, @optimized_inputs,
                                                            @cur_instruction&.assign, @cur_instruction.scope)

        @memoize_assigns = {} unless clear_memoize?
        @memoize_assigns[@cur_instruction.assign] = {
          inputs: @optimized_inputs,
          index:
        }
      end

      def clear_memoize?
        if @last_was_eavluate.nil?
          @last_was_eavluate = @cur_instruction.instruction == 'evaluate'
          return false
        end

        if @cur_instruction.instruction == 'evaluate'
          if @last_was_eavluate
            false
          else
            @last_was_eavluate = true
            true
          end
        elsif @last_was_eavluate
          @last_was_eavluate = false
          true
        else
          false
        end
      end

      def apply_to_instruction_input(input)
        done_a_thing = false
        @memoize_assigns.each do |key, value|
          next unless do_a_thing?(input, key, input.split('.')[0])

          input = input.gsub(key, "#{value[:inputs][0]}(#{value[:inputs][1..].join(', ')})")
          @optimized_inputs << input
          done_a_thing = true
          @to_remove[value[:index]] = true
          next
        end

        @optimized_inputs << input unless done_a_thing
      end

      def do_a_thing?(input, key, input_beginning)
        input &&
          key &&
          input_beginning == key &&
          !(input.start_with?('"') &&
          input.end_with?('"')) &&
          @cur_instruction.instruction == 'evaluate'
      end
    end
  end
end
