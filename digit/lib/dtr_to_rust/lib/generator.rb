# frozen_string_literal: true

require 'dtr_core'

module DTRToRust
  # Generates Rust code from a DTR contract
  class Generator
    def initialize(file_path)
      @file_path = file_path
      @dtr_contract = ::DTRCore::Parser.parse(file_path)

      @content = ''
    end

    def generate
      generate_contract_header
      generate_contract_name
      generate_state
      generate_functions

      @content
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
      @content += "#[contractimpl]\nimpl #{dtr_contract.name} {#{generate_functions_each(dtr_contract.functions)}\n}\n"
    end

    def generate_functions_each(functions)
      functions.map do |function|
        "\n    pub fn #{function.name}(#{generate_function_args(function)}) -> #{function.output} {\n    }"
      end.join("\n")
    end

    def generate_function_args(function)
      all_inputs = [{ name: 'env', type_name: 'Env' }] + function.inputs

      all_inputs.map { |x| "#{x[:name]}: #{x[:type_name]}" }.join(', ')
    end
  end
end
