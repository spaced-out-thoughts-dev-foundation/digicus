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
      @content += "#![no_std]\nuse soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};\n\n"
    end

    def generate_contract_name
      @content += "#[contract]\npub struct #{dtr_contract.name};\n\n"
    end

    def generate_state
      return if dtr_contract.state.nil?

      @content += 'pub struct State {'
      dtr_contract.state.each do |state|
        @content += "  pub #{state.name}: #{state.type},"
      end
      @content += "}\n\n"
    end

    def generate_functions
      @content += "#[contractimpl]\nimpl #{dtr_contract.name} {#{generate_functions_each(dtr_contract.functions)}}\n"
    end

    def generate_functions_each(functions)
      functions&.map do |function|
        "\n    pub fn #{function.name}(#{generate_function_args(function)}) " \
          "-> #{function.output} {\n#{generate_instructions_each(function.instructions)}\n    }\n"
      end&.join("\n")
    end

    def generate_function_args(function)
      all_inputs = [] + function.inputs

      all_inputs.map { |x| "#{x[:name]}: #{x[:type_name]}" }.join(', ')
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
  end
end
