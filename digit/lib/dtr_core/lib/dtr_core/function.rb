# frozen_string_literal: true

require 'dtr_core/common'

module DTRCore
  # Represents a state in a DTR file.
  class Function
    include ::DTRCore::Common

    attr_reader :name, :inputs, :output, :instructions

    def initialize(name, inputs, output, instructions)
      @name = name
      @inputs = inputs
      @output = output
      @instructions = instructions
    end

    def sanitize
      @inputs = format_function_inputs(@inputs)
      @instructions = format_function_instruction(@instructions)
    end

    def self.from_definition(definition)
      name = definition[/\s*\[(?<all>[^\]]+)]/, 1]
      inputs = (definition[/Inputs\s*:\s*{\s*(?<inputs>[^}]+)\s*}/, 1])
      # TODO: check output type
      output = definition[/Output:\s*(.+)/, 1]
      instructions = definition[/Instructions:\s*\$(?<inputs>[^\$]+)\$/, 1]

      function_object = new(name, inputs, output, instructions)

      function_object.sanitize

      function_object
    end

    def ==(other)
      name == other.name &&
        inputs == other.inputs &&
        output == other.output &&
        instructions == other.instructions
    end

    private

    def format_function_inputs(inputs)
      return [] if inputs.nil?

      split_strip_select(inputs).map { |x| { name: x.split(':')[0].strip, type_name: x.split(':')[1].strip } }
    end

    def format_function_instruction(instructions)
      split_strip_select(instructions)&.map { |instruction| parse_function_instruction(instruction) }
    end

    def parse_function_instruction(instruction)
      {
        instruction: instruction[/instruction:\s*(?<all>[^\s,]+)/, 1],
        inputs: parse_function_instruction_input(instruction),
        assign: instruction[/\s*assign:\s*(?<all>[^\s\,]+)/, 1]
      }
    end

    def parse_function_instruction_input(definition)
      definition[/\s*input:\s*\((?<all>[^\)]+)\)/, 1]
        &.split(',')&.map(&:strip)
    end
  end
end
