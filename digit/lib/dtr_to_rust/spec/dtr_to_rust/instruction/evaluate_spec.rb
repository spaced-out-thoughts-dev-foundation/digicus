# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Evaluate do
  describe '#handle' do
    context 'when input requires a reference' do
      it 'returns the correct Rust code' do
        instruction = {
          instruction: 'evaluate',
          inputs: ['env.storage', 'env', 'foo'],
          assign: nil,
          scope: 0
        }

        expect(described_class.handle(instruction))
          .to eq('env.storage(&env, &foo);')
      end
    end

    context 'when it is a call expression' do
      context 'when no inputs' do
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['storage'],
            assign: 'CALL_EXPRESSION_RESULT',
            scope: 0
          }

          expect(described_class.handle(instruction))
            .to eq('let mut CALL_EXPRESSION_RESULT = storage();')
        end
      end

      context 'when one input' do
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['instance', '"foo"'],
            assign: 'CALL_EXPRESSION_RESULT',
            scope: 0
          }

          expect(described_class.handle(instruction))
            .to eq('let mut CALL_EXPRESSION_RESULT = instance("foo");')
        end
      end

      context 'when multiple inputs' do
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['extend_ttl', 50, 100],
            assign: 'CALL_EXPRESSION_RESULT',
            scope: 0
          }

          expect(described_class.handle(instruction))
            .to eq('let mut CALL_EXPRESSION_RESULT = extend_ttl(50, 100);')
        end
      end

      context 'when no assign' do
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['storage'],
            assign: nil,
            scope: 0
          }

          expect(described_class.handle(instruction)).to eq('storage();')
        end
      end
    end

    context 'when it is a method call expression' do
      context 'when no inputs' do
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['env.storage'],
            assign: 'METHOD_CALL_EXPRESSION',
            scope: 0
          }

          expect(described_class.handle(instruction))
            .to eq('let mut METHOD_CALL_EXPRESSION = env.storage();')
        end
      end

      context 'when one input' do
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['METHOD_CALL_EXPRESSION.instance', '"foo"'],
            assign: 'METHOD_CALL_EXPRESSION',
            scope: 0
          }

          expect(described_class.handle(instruction))
            .to eq('let mut METHOD_CALL_EXPRESSION = METHOD_CALL_EXPRESSION.instance("foo");')
        end
      end

      context 'when multiple inputs' do
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['METHOD_CALL_EXPRESSION.extend_ttl', 50, 100],
            assign: 'METHOD_CALL_RESULT',
            scope: 0
          }

          expect(described_class.handle(instruction))
            .to eq('let mut METHOD_CALL_RESULT = METHOD_CALL_EXPRESSION.extend_ttl(50, 100);')
        end
      end
    end
  end
end
