# frozen_string_literal: true

require 'dtr_core'

module DTRToRust
  # Generates Rust code from a DTR contract
  class Generator
    def initialize(content)
      @dtr_contract = ::DTRCore::Contract.from_dtr_raw(content)
    end

    def generate
      @content = ''

      generate_contract_header
      generate_contract_name
      generate_state
      generate_functions

      @content
    end

    def self.generate_from_file(file_path)
      new(File.read(file_path)).generate
    end

    def self.generate_from_string(dtr_string)
      new(dtr_string).generate
    end

    private

    attr_reader :dtr_contract

    def generate_contract_header
      # TODO: don't hardcode imports
      @content += "#![no_std]\nuse soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, log};\n\n"
    end

    def generate_contract_name
      @content += "#[contract]\npub struct #{dtr_contract.name};\n\n"
    end

    def generate_state
      return if dtr_contract.state.nil?

      dtr_contract.state.each do |state_value|
        if state_value.type == 'String'
          @content += "const #{state_value.name}: Symbol = symbol_short!(\"#{state_value.initial_value}\");\n"
        end
      end

      @content += "\n"
    end

    def generate_functions
      @content += "#[contractimpl]\nimpl #{dtr_contract.name} {#{generate_functions_each(dtr_contract.functions)}}\n"
    end

    def generate_functions_each(functions)
      functions&.map do |function|
        return_string = "\n    pub fn #{function.name}(#{generate_function_args(function)}) "
        return_string += generate_function_output(function)
        return_string += " {\n#{generate_instructions_each(function.instructions)}\n    }\n"

        return_string
      end&.join("\n")
    end

    def generate_function_output(function)
      return '' if function.output.nil?

      "-> #{translate_type(function.output)}"
    end

    def generate_function_args(function)
      all_inputs = [] + function.inputs

      all_inputs.map { |x| "#{x[:name]}: #{translate_type(x[:type_name])}" }.join(', ')
    end

    def generate_instructions_each(instructions)
      instructions.map do |instruction|
        generate_instruction(instruction)
      end.join("\n")
    end

    def generate_instruction(instruction)
      handler = InstructionHandler.new(instruction)
      handler.generate_rust
    end

    def translate_type(type)
      case type
      when 'String'
        'Symbol'
      else
        type
      end
    end
  end
end
