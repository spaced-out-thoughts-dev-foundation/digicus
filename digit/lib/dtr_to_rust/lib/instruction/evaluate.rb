# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the Evaluate instruction.
    class Evaluate < Handler
      def handle
        rust_string = handle_keyword_method_invocation
        form_rust_string(rust_string)
      end

      private

      def handle_keyword_method_invocation
        case @instruction.inputs[0]
        when 'equal_to'
          handle_equal_to
        when '!'
          handle_unary_negation
        else
          handle_non_keyword_method_invocation
        end
      end

      def handle_non_keyword_method_invocation
        inputs = @instruction.inputs[1..]
        evaluated_method_name = @instruction.inputs[0]
        assignment = @instruction.assign

        assignment_rust = "let mut #{assignment} = "
        # TODO: make this less hacky evaluated_method_name.end_with?('set')
        body_rust = "#{invocation_name(evaluated_method_name)}(#{inputs_to_rust_string(inputs,
                                                                                       don_t_append_ref)});"
        "#{assignment.nil? ? '' : assignment_rust}#{body_rust}"
      end

      def handle_equal_to
        inputs = @instruction.inputs[1..]
        @instruction.inputs[0]
        assignment = @instruction.assign

        assignment_rust = "let #{assignment} = "
        lhs = Common::ReferenceAppender.call(inputs[0])
        rhs = Common::ReferenceAppender.call(inputs[1])
        body_rust = "#{lhs} == #{rhs};"

        "#{assignment.nil? ? '' : assignment_rust}#{body_rust}"
      end

      def handle_unary_negation
        inputs = @instruction.inputs[1..]
        @instruction.inputs[0]
        assignment = @instruction.assign

        assignment_rust = "let #{assignment} = "
        body_rust = "!(#{inputs[0]});"

        "#{assignment.nil? ? '' : assignment_rust}#{body_rust}"
      end

      def don_t_append_ref
        @instruction.inputs[0].end_with?('set') || @instruction.inputs[0].end_with?('unwrap_or')
      end

      def invocation_name(evaluated_method_name)
        if @function_names.include?(evaluated_method_name)
          "Self::#{evaluated_method_name}"
        else
          evaluated_method_name
        end
      end

      def inputs_to_rust_string(inputs, ref_nums)
        inputs.map { |input| Common::ReferenceAppender.call(input, ref_nums:) }.join(', ')
      end
    end
  end
end
