# frozen_string_literal: true

module DTRCore
  # Parses a DTR file and returns a Contract object.
  class Parser
    def initialize(file_path)
      raise "Unable to find file: #{file_path}." unless File.exist?(file_path)

      @content = File.read(file_path)

      # nil is a placeholder for the actual values
      # thus, the actual values are not known yet
      # and so if we see nil we know that section
      # was not included in the dtr file
      @sections = {
        contract_name: nil,
        state: nil,
        functions: nil
      }
    end

    def self.parse(file_path)
      new(file_path).parse
    end

    def parse
      parse_contract_name_section
      parse_state_section
      parse_function_section

      DTRCore::Contract.new(sections[:contract_name], sections[:state], sections[:functions])
    end

    private

    attr_reader :content
    attr_accessor :sections

    def parse_contract_name_section
      contract_name_pattern = /\[Contract\]:\s*(.+)/

      contract_name_section = content.match(contract_name_pattern)&.captures&.first

      raise 'Missing contract name.' if contract_name_section.nil?

      sections[:contract_name] = contract_name_section
    end

    def parse_state_section
      state_pattern = /\[State\]:\s*((?:\s*\*\s*\[.+?\]\n(?:\s*  \* .+\n?)*)*)/

      state_section = content.match(state_pattern)&.captures&.first

      return if state_section.nil?

      state_definitions = state_section
                          .split(/\n\s*\*\s*\[/).map { |x| "[#{x.strip}" }
                          .map { |definition| state_definition_to_state_object(definition) }

      raise 'Empty state section.' if state_definitions.empty?

      sections[:state] = state_definitions
    end

    def clean_state_definition_name(definition)
      definition
        &.gsub('*', '')
        &.gsub("\n", '')
        &.gsub('[', '')
        &.strip
    end

    def state_definition_to_state_object(definition)
      name = clean_state_definition_name definition[/^\[([^\]]+)\]/, 1]

      type = definition[/Type:\s*(\w+)/, 1]

      initial_value_raw = definition[/Initial Value:\s*(.+)/, 1]

      validate_type_name_and_initial_value!(type, initial_value_raw)

      DTRCore::State.new(name, type, coerce_initial_value(type, initial_value_raw))
    end

    def parse_function_section
      function_pattern = /\[Functions\]:(?<all>.*):\[Functions\]/m
      function_section = content.match(function_pattern)&.captures&.first

      return if function_section.nil?

      function_definitions = parse_parse_function_section(function_section)

      raise 'Empty function section.' if function_definitions.empty?

      sections[:functions] = function_definitions
    end

    def parse_parse_function_section(function_section)
      function_section
        .split('-()')
        .map { |x| x.strip.to_s }
        .map { |definition| function_definition_to_function_object(definition) }
        .reject { |x| x.name.nil? }
    end

    def function_definition_to_function_object(definition)
      name = definition[/\s*\[(?<all>[^\]]+)]/, 1]
      inputs = format_function_inputs(definition[/Inputs\s*:\s*{\s*(?<inputs>[^}]+)\s*}/, 1])
      output = definition[/Output:\s*(.+)/, 1]
      instructions = format_function_instruction(definition[/Instructions:\s*\$(?<inputs>[^\$]+)\$/, 1])

      DTRCore::Function.new(name, inputs, output, instructions)
    end

    def format_function_inputs(inputs)
      return [] if inputs.nil?

      split_strip_select(inputs)
        .map { |x| x.split(':') }
        .map { |x| { name: x[0]&.strip, type_name: x[1]&.strip } }
    end

    def format_function_instruction(instructions)
      return [] if instructions.nil?

      split_strip_select(instructions)
        .map { |instruction| parse_function_instruction(instruction) }
    end

    def split_strip_select(some_list)
      some_list
        .split("\n")
        .map(&:strip)
        .select { |x| x.length&.> 0 }
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
        &.split(',')
        &.map { |x| x&.strip&.gsub('"', '')&.gsub("'", '') }
    end

    def coerce_initial_value(type_name, initial_value)
      case type_name
      when 'I32', 'I64', 'I256', 'U32', 'U64', 'U256'
        initial_value.to_i

      # TODO: check type
      when 'Symbol'
        initial_value
          &.gsub('"', '')
          &.gsub("'", '')
      else
        raise 'Missing Invalid Type Name.'
      end
    end

    def validate_type_name_and_initial_value!(type_name, initial_value)
      raise 'Missing Type Name.' if type_name.nil?
      raise 'Missing Initial Value.' if initial_value.nil?

      case type_name
      when 'I32', 'I64', 'I256', 'U32', 'U64', 'U256'
        validate_numeric!(type_name, initial_value)
      when 'Symbol'
        # no validation needed
      else
        raise 'Missing Invalid Type Name.'
      end
    end

    def validate_numeric!(type_name, initial_value)
      raise 'Invalid initial value for type. Wrong type.' unless initial_value =~ (/^[\-\.\d]\d*(\.?\d*)*/)

      raise "Invalid initial value for type #{type_name}. Out of range." unless initial_value.to_i.between?(
        DTRCore::Number.const_get(:"MIN_#{type_name}"), DTRCore::Number.const_get(:"MAX_#{type_name}")
      )
    end
  end
end
