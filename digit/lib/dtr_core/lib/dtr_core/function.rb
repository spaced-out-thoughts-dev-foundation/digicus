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

    def to_s
      return_string = ''

      return_string += name_to_s
      return_string += inputs_to_s
      return_string += output_to_s
      return_string += instructions_to_s

      return_string
    end

    private

    def name_to_s
      "  -() [#{name}]\n"
    end

    def inputs_to_s
      input_formatted = inputs.map { |x| "#{x[:name]}: #{x[:type_name]}" }.join("\n    ")
      "    * Inputs:\n    {\n    #{input_formatted}\n    }\n"
    end

    def output_to_s
      return '' if output.nil?

      "    * Output: #{output}\n"
    end

    def instructions_to_s
      return_string = ''

      return_string += "    * Instructions:\n"
      return_string += "      $\n"
      @instructions.each do |x|
        return_string += "        #{x}\n"
      end
      return_string += "      $\n"

      return_string
    end

    def format_function_inputs(inputs)
      return [] if inputs.nil?

      split_strip_select(inputs).map { |x| { name: x.split(':')[0].strip, type_name: x.split(':')[1].strip } }
    end

    def format_function_instruction(instructions)
      split_strip_select(instructions)&.map { |instruction| parse_function_instruction(instruction) }
    end

    def parse_function_instruction(instruction)
      instruction = DTRCore::Instruction.new(
        instruction[/instruction:\s*(?<all>[^\s,]+)/, 1],
        parse_function_instruction_input(instruction),
        instruction[/\s*assign:\s*(?<all>[^\s\,]+)/, 1],
        instruction[/\s*scope:\s*(?<all>[^\s\,]+)/, 1].to_i || 0
      )

      raise "Invalid instruction: #{instruction}" unless instruction.valid?

      instruction
    end

    def parse_function_instruction_input(definition)
      raw_inputs = definition[/\s*input:\s*\((?<all>[^\)]+)\)/, 1]
      return nil if raw_inputs.nil?

      cur_word = ''
      inputs_to_return = []
      in_string = false

      raw_inputs.each_char do |char|
        if in_string
          if char == '"'
            in_string = false
            inputs_to_return.push("\"#{cur_word}\"")
            cur_word = ''
          else
            cur_word += char
          end
        elsif cur_word.empty? && char == '"'
          in_string = true
        elsif char == ','
          inputs_to_return.push(cur_word)
          cur_word = ''
          in_string = false
        elsif char == ' ' && cur_word.empty?
          next
        else
          cur_word += char
        end
      end

      inputs_to_return.push(cur_word) unless cur_word.empty?

      inputs_to_return.filter! { |x| !x.empty? }

      inputs_to_return&.map(&:strip)
    end
  end
end
