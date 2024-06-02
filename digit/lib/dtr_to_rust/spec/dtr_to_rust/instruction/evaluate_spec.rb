# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Evaluate do
  describe '#handle' do
    context 'when it is a call expression' do
      context 'when no inputs' do
        # { instruction: evaluate, input: (storage), assign: CALL_EXPRESSION_RESULT, scope: 0 }
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['storage'],
            assign: 'CALL_EXPRESSION_RESULT',
            scope: 0
          }

          expect(described_class.handle(instruction)).to eq('CALL_EXPRESSION_RESULT = storage();')
        end
      end

      context 'when one input' do
        # { instruction: evaluate, input: (METHOD_CALL_EXPRESSION.instance, "foo"), assign: CALL_EXPRESSION_RESULT, scope: 0 }
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['instance', '"foo"'],
            assign: 'CALL_EXPRESSION_RESULT',
            scope: 0
          }

          expect(described_class.handle(instruction)).to eq('CALL_EXPRESSION_RESULT = instance("foo");')
        end
      end

      context 'when multiple inputs' do
        # { instruction: evaluate, input: (extend_ttl, METHOD_CALL_EXPRESSION, 50, 100), assign: CALL_EXPRESSION_RESULT, scope: 0 }
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['extend_ttl', 50, 100],
            assign: 'CALL_EXPRESSION_RESULT',
            scope: 0
          }

          expect(described_class.handle(instruction)).to eq('CALL_EXPRESSION_RESULT = extend_ttl(50, 100);')
        end
      end
    end

    context 'when it is a method call expression' do
      context 'when no inputs' do
        # { instruction: evaluate, input: (env.storage), assign: METHOD_CALL_EXPRESSION, scope: 0 }
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['env.storage'],
            assign: 'METHOD_CALL_EXPRESSION',
            scope: 0
          }

          expect(described_class.handle(instruction)).to eq('METHOD_CALL_EXPRESSION = env.storage();')
        end
      end

      context 'when one input' do
        # { instruction: evaluate, input: (METHOD_CALL_EXPRESSION.instance, "foo"), assign: METHOD_CALL_EXPRESSION, scope: 0 }
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['METHOD_CALL_EXPRESSION.instance', '"foo"'],
            assign: 'METHOD_CALL_EXPRESSION',
            scope: 0
          }

          expect(described_class.handle(instruction)).to eq('METHOD_CALL_EXPRESSION = METHOD_CALL_EXPRESSION.instance("foo");')
        end
      end

      context 'when multiple inputs' do
        # { instruction: evaluate, input: (extend_ttl, METHOD_CALL_EXPRESSION, 50, 100), assign: METHOD_CALL_RESULT, scope: 0 }
        it 'returns the correct Rust code' do
          instruction = {
            instruction: 'evaluate',
            inputs: ['METHOD_CALL_EXPRESSION.extend_ttl', 50, 100],
            assign: 'METHOD_CALL_RESULT',
            scope: 0
          }

          expect(described_class.handle(instruction)).to eq('METHOD_CALL_RESULT = METHOD_CALL_EXPRESSION.extend_ttl(50, 100);')
        end
      end
    end
  end
end
