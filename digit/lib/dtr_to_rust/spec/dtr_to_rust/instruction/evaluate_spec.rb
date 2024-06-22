# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Evaluate do
  describe '#handle' do
    context 'when input requires a reference' do
      it 'returns the correct Rust code' do
        instruction = DTRCore::Instruction.new('evaluate', ['env.storage', 'env', 'foo'], nil, 0)

        expect(described_class.handle(instruction, 0, [], [], false))
          .to eq('        env.storage(&env, &foo);')
      end
    end

    context 'when it is a call expression' do
      context 'when no inputs' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['storage'], 'CALL_EXPRESSION_RESULT', 0)

          expect(described_class.handle(instruction, 0, [], [], false))
            .to eq('        let mut CALL_EXPRESSION_RESULT = storage();')
        end
      end

      context 'when one input' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['instance', '"foo"'], 'CALL_EXPRESSION_RESULT', 0)

          expect(described_class.handle(instruction, 0, [], [], false))
            .to eq('        let mut CALL_EXPRESSION_RESULT = instance("foo");')
        end
      end

      context 'when multiple inputs' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['extend_ttl', 50, 100], 'CALL_EXPRESSION_RESULT', 0)

          expect(described_class.handle(instruction, 0, [], [], false))
            .to eq('        let mut CALL_EXPRESSION_RESULT = extend_ttl(50, 100);')
        end
      end

      context 'when no assign' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['storage'], nil, 0)

          expect(described_class.handle(instruction, 0, [], [], false)).to eq('        storage();')
        end
      end
    end

    context 'when it is a method call expression' do
      context 'when no inputs' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['env.storage'], 'METHOD_CALL_EXPRESSION', 0)

          expect(described_class.handle(instruction, 0, [], [], false))
            .to eq('        let mut METHOD_CALL_EXPRESSION = env.storage();')
        end
      end

      context 'when one input' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['METHOD_CALL_EXPRESSION.instance', '"foo"'],
                                                 'METHOD_CALL_EXPRESSION', 0)

          expect(described_class.handle(instruction, 0, [], [], false))
            .to eq('        let mut METHOD_CALL_EXPRESSION = METHOD_CALL_EXPRESSION.instance("foo");')
        end
      end

      context 'when multiple inputs' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['METHOD_CALL_EXPRESSION.extend_ttl', 50, 100],
                                                 'METHOD_CALL_RESULT', 0)

          expect(described_class.handle(instruction, 0, [], [], false))
            .to eq('        let mut METHOD_CALL_RESULT = METHOD_CALL_EXPRESSION.extend_ttl(50, 100);')
        end
      end
    end

    context 'when one of the inputs is a string' do
      it 'returns the correct Rust code' do
        instruction = DTRCore::Instruction.new('evaluate', ['log_to_env', 'env', '"Yes, the answer to life is 42!"'],
                                               'CALL_EXPRESSION_RESULT', 0)

        expect(described_class.handle(instruction, 0, [], [], false))
          .to eq('        let mut CALL_EXPRESSION_RESULT = log_to_env(&env, "Yes, the answer to life is 42!");')
      end
    end

    context 'when it is a keyword method call' do
      context 'when equal_to' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['equal_to', 10, 20], 'EQUAL_TO_RESULT', 0)

          expect(described_class.handle(instruction, 0, [], [],
                                        false)).to eq('        let EQUAL_TO_RESULT = 10 == 20;')
        end

        context 'when unary argument' do
          it 'returns the correct Rust code' do
            instruction = DTRCore::Instruction.new('evaluate', ['!', 'UNARY_ARGUMENT_0'],
                                                   'CONDITIONAL_JUMP_ASSIGNMENT', 0)

            expect(described_class.handle(instruction, 0, [], [],
                                          false)).to eq('        let CONDITIONAL_JUMP_ASSIGNMENT = !(UNARY_ARGUMENT_0);')
          end
        end
      end
    end
  end
end
