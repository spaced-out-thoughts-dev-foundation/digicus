# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Evaluate do
  describe '#handle' do
    context 'when input requires a reference' do
      it 'returns the correct Rust code' do
        instruction = DTRCore::Instruction.new('evaluate', ['env.storage', 'env', 'foo'], nil, 0)

        expect(described_class.handle(instruction))
          .to eq('        &env.storage(&env, &foo);')
      end
    end

    context 'when it is a call expression' do
      context 'when no inputs' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['storage'], 'CALL_EXPRESSION_RESULT', 0)

          expect(described_class.handle(instruction))
            .to eq('        let mut CALL_EXPRESSION_RESULT = &storage();')
        end
      end

      context 'when one input' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['instance', '"foo"'], 'CALL_EXPRESSION_RESULT', 0)

          expect(described_class.handle(instruction))
            .to eq('        let mut CALL_EXPRESSION_RESULT = &instance("foo");')
        end
      end

      context 'when multiple inputs' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['extend_ttl', 50, 100], 'CALL_EXPRESSION_RESULT', 0)

          expect(described_class.handle(instruction))
            .to eq('        let mut CALL_EXPRESSION_RESULT = &extend_ttl(50, 100);')
        end
      end

      context 'when no assign' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['storage'], nil, 0)

          expect(described_class.handle(instruction)).to eq('        &storage();')
        end
      end
    end

    context 'when it is a method call expression' do
      context 'when no inputs' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['env.storage'], 'METHOD_CALL_EXPRESSION', 0)

          expect(described_class.handle(instruction))
            .to eq('        let mut METHOD_CALL_EXPRESSION = &env.storage();')
        end
      end

      context 'when one input' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['METHOD_CALL_EXPRESSION.instance', '"foo"'],
                                                 'METHOD_CALL_EXPRESSION', 0)

          expect(described_class.handle(instruction))
            .to eq('        let mut METHOD_CALL_EXPRESSION = &METHOD_CALL_EXPRESSION.instance("foo");')
        end
      end

      context 'when multiple inputs' do
        it 'returns the correct Rust code' do
          instruction = DTRCore::Instruction.new('evaluate', ['METHOD_CALL_EXPRESSION.extend_ttl', 50, 100],
                                                 'METHOD_CALL_RESULT', 0)

          expect(described_class.handle(instruction))
            .to eq('        let mut METHOD_CALL_RESULT = &METHOD_CALL_EXPRESSION.extend_ttl(50, 100);')
        end
      end
    end
  end
end
