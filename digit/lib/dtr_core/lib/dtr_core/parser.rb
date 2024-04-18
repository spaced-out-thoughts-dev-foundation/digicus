# frozen_string_literal: true

require 'dtr_core/common'

module DTRCore
  # Parses a DTR file and returns a Contract object.
  class Parser
    include ::DTRCore::Common

    def initialize(file_path)
      raise "Unable to find file: #{file_path}." unless File.exist?(file_path)

      @content = File.read(file_path)
    end

    def self.parse(file_path)
      new(file_path).parse
    end

    def parse
      DTRCore::Contract.new(parse_contract_name_section, parse_state_section, parse_function_section)
    end

    private

    attr_reader :content
    attr_accessor :sections

    def parse_contract_name_section
      contract_name_section = first_match_for_content(/\[Contract\]:\s*(.+)/)

      raise 'Missing contract name.' if contract_name_section.nil?

      contract_name_section
    end

    def parse_state_section
      state_section = first_match_for_content(/\[State\]:\s*((?:\s*\*\s*\[.+?\]\n(?:\s*  \* .+\n?)*)*)/)

      return nil if state_section.nil?

      state_definitions = state_section
                          .split(/\n\s*\*\s*\[/).map { |x| "[#{x.strip}" }
                          .map { |definition| state_definition_to_state_object(definition) }

      raise 'Empty state section.' if state_definitions.empty?

      state_definitions
    end

    def clean_state_definition_name(definition)
      definition.gsub(/[\*\n\[]/, '').strip
    end

    def state_definition_to_state_object(definition)
      name = clean_state_definition_name definition[/^\[([^\]]+)\]/, 1]

      type = definition[/Type:\s*(\w+)/, 1]

      initial_value = DTRCore::TypeValidator.new(type, definition[/Initial Value:\s*(.+)/, 1])
                                            .validate_then_coerce_initial_value!

      DTRCore::State.new(name, type, initial_value)
    end

    def parse_function_section
      function_section = first_match_for_content(/\[Functions\]:(?<all>.*):\[Functions\]/m)

      return nil if function_section.nil?

      function_definitions = parse_parse_function_section(function_section)

      raise 'Empty function section.' if function_definitions.empty?

      function_definitions
    end

    def parse_parse_function_section(function_section)
      function_section.split('-()')
                      .map { |x| function_definition_to_function_object(x.strip.to_s) }
                      .reject { |x| x.name.nil? }
    end

    def function_definition_to_function_object(definition)
      name = definition[/\s*\[(?<all>[^\]]+)]/, 1]
      inputs = format_function_inputs(definition[/Inputs\s*:\s*{\s*(?<inputs>[^}]+)\s*}/, 1])
      # TODO: check output type
      output = definition[/Output:\s*(.+)/, 1]
      instructions = format_function_instruction(definition[/Instructions:\s*\$(?<inputs>[^\$]+)\$/, 1])

      DTRCore::Function.new(name, inputs, output, instructions)
    end

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
