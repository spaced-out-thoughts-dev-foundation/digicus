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

      generate_user_defined_types
      generate_state
      generate_contract_name
      generate_interface
      generate_helpers

      generate_contract_header

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
      imports_super_set = %w[
        contract
        contractimpl
        contracttype
        symbol_short
        vec
        Env
        Symbol
        Vec
        log
      ]

      used_imports = []

      @content.split.each do |word|
        imports_super_set.each do |import|
          used_imports << import if word.include?(import)
        end
      end

      used_imports.uniq!

      # TODO: don't hardcode imports
      @content = "#![no_std]\nuse soroban_sdk::{#{used_imports.join(', ')}};\n\n" + @content
    end

    def generate_contract_name
      @content += "#[contract]\npub struct #{dtr_contract.name};\n\n"
    end

    def generate_state
      return if dtr_contract.state.nil?

      dtr_contract.state.each do |state_value|
        if state_value.type == 'String'
          @content += "const #{state_value.name}: Symbol = symbol_short!(#{state_value.initial_value});\n"
        end
      end

      @content += "\n"
    end

    def generate_interface
      @content += "#{generate_functions_each(dtr_contract.helpers)}\n"
    end

    def generate_helpers
      @content += "#[contractimpl]\nimpl #{dtr_contract.name} {#{generate_functions_each(dtr_contract.interface)}}\n"
    end

    def generate_functions_each(functions)
      functions&.map do |function|
        optimized_instructions =
          Optimization::ChainedInvocationAssignmentReduction.apply(function.instructions)

        return_string = "\n    pub fn #{function.name}(#{generate_function_args(function)}) "
        return_string += generate_function_output(function)
        return_string += " {\n#{generate_instructions_each(optimized_instructions)}\n    }\n"

        return_string
      end&.join("\n")
    end

    def generate_function_output(function)
      return '' if function.output.nil?

      "-> #{Common::TypeTranslator.translate_type(function.output)}"
    end

    def generate_function_args(function)
      all_inputs = [] + function.inputs

      all_inputs.map { |x| "#{x[:name]}: #{Common::TypeTranslator.translate_type(x[:type_name])}" }.join(', ')
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

    def generate_user_defined_types
      return if dtr_contract.user_defined_types.nil?

      dtr_contract.user_defined_types.each do |udt|
        @content += DTRToRust::UserDefinedTypes::Handler.generate(udt)
      end
    end
  end
end
