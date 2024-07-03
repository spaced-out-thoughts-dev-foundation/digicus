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

      @content += dtr_contract.non_translatables if dtr_contract.non_translatables
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
        Address
        BytesN
        contract
        contractimpl
        contracttype
        contracterror
        symbol_short
        vec
        Env
        Symbol
        Vec
        Val
        log
        token
      ]

      used_imports = []

      @content.split.each do |word|
        imports_super_set.each do |import|
          used_imports << import if word.include?(import)
        end
      end

      used_imports.uniq!

      # TODO: unsure how to check this one
      used_imports << 'auth::Context'
      used_imports << 'IntoVal'
      used_imports << 'unwrap::UnwrapOptimized'

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
        else
          @content += "const #{state_value.name}: #{Common::TypeTranslator.translate_type(state_value.type)} = #{state_value.initial_value};\n"
        end
      end

      @content += "\n"
    end

    def generate_interface
      @content += "#[contractimpl]\nimpl #{dtr_contract.name} {#{generate_functions_each(dtr_contract.interface,
                                                                                         false)}}\n"
    end

    def generate_helpers
      @content += "#{generate_functions_each(dtr_contract.helpers, true)}\n"
    end

    def generate_functions_each(functions, is_helper)
      function_names = functions&.map(&:name)

      functions&.map do |function|
        optimized_instructions = optimize_instructions(function.instructions)

        instruction_blocks = Aggregator::ScopeBlockAggregator.aggregate(optimized_instructions)

        # HACK
        function_inputs = {}
        function.inputs.each do |x|
          function_inputs[x[:name]] = {
            input_has_ref: Common::TypeTranslator.translate_type(x[:type_name]).include?('&')
          }
        end

        return_string = "\n#{is_helper ? '' : '    '}pub fn #{function.name}(#{generate_function_args(function)}) "
        return_string += generate_function_output(function)
        return_string += " {\n"
        if function.output
          return_string += "        let Thing_to_return: #{Common::TypeTranslator.translate_type(function.output)};\n"
        end
        return_string += "#{generate_instructions_for_blocks(instruction_blocks, function_names,
                                                             is_helper, function_inputs)}"

        if @last_scope.positive?
          while @last_scope.positive?
            return_string += "\n" + form_rust_string('}', @last_scope,
                                                     is_helper)
            @last_scope -= 1
          end
        end
        return_string += "\n#{is_helper ? '' : '    '}}\n"

        return_string
      end&.join("\n")
    end

    def optimize_instructions(instructions)
      stage1_optimized = Optimization::FieldToAssignmentConversion.apply(instructions)
      stage2_optimized = Optimization::BinaryXToSelfAssignmentReduction.apply(stage1_optimized)
      Optimization::ChainedInvocationAssignmentReduction.apply(stage2_optimized)
    end

    def generate_function_output(function)
      return '' if function.output.nil?

      "-> #{Common::TypeTranslator.translate_type(function.output)}"
    end

    def generate_function_args(function)
      all_inputs = [] + function.inputs

      all_inputs.map { |x| "#{x[:name]}: #{Common::TypeTranslator.translate_type(x[:type_name])}" }.join(', ')
    end

    def generate_instructions_for_blocks(instruction_blocks, function_names, is_helper, function_inputs)
      new_assignment_name_to_scope_map = {}
      instruction_blocks.map do |block|
        spacing_scope = block[:decorated_value]
        content = ''
        if @last_scope && @last_scope > spacing_scope
          while @last_scope > spacing_scope
            content += form_rust_string("}\n", @last_scope, is_helper)
            @last_scope -= 1
          end
        end
        @last_scope = spacing_scope
        content_to_add, new_assignment_name_to_scope_map =
          generate_instructions_each(block[:block], spacing_scope, function_names, is_helper,
                                     new_assignment_name_to_scope_map, function_inputs)

        content += content_to_add
      end.join("\n")
    end

    def generate_instructions_each(instructions, spacing_scope, function_names, is_helper,
                                   assignment_name_to_scope_map, function_inputs)
      new_assignment_name_to_scope_map = assignment_name_to_scope_map.clone
      content_to_return = instructions.map do |instruction|
        content = ''
        content_to_add, new_assignment_name_to_scope_map =
          generate_instruction(instruction, spacing_scope, function_names, is_helper, new_assignment_name_to_scope_map,
                               function_inputs)

        content += content_to_add
        content
      end.join("\n")

      [content_to_return, new_assignment_name_to_scope_map]
    end

    def form_rust_string(instruction_string, scope, is_helper)
      "#{spacing(scope, is_helper)}#{instruction_string}"
    end

    def spacing(scope, is_helper)
      '    ' * (is_helper ? 0 : scope + 1)
    end

    def generate_instruction(instruction, spacing_scope, function_names, is_helper, assignment_name_to_scope_map,
                             function_inputs)

      puts "#{instruction}"
      handler = InstructionHandler.new(instruction, spacing_scope, function_names,
                                       dtr_contract.user_defined_types || [], is_helper, assignment_name_to_scope_map, function_inputs)

      result = handler.generate_rust
      if instruction.assign
        unless assignment_name_to_scope_map[instruction.assign]
          assignment_name_to_scope_map[instruction.assign] =
            []
        end

        assignment_name_to_scope_map[instruction.assign] << spacing_scope
      end
      [result, assignment_name_to_scope_map]
    end

    def generate_user_defined_types
      return if dtr_contract.user_defined_types.nil?

      dtr_contract.user_defined_types.each do |udt|
        @content += DTRToRust::UserDefinedTypes::Handler.generate(udt)
      end
    end
  end
end
