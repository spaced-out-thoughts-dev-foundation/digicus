# frozen_string_literal: true

require 'spec_helper'

RSpec.describe DTRToRust::Optimization::BinaryXToSelfAssignmentReduction do
  describe '.apply' do
    context 'when the instructions are empty' do
      let(:instructions) { [] }

      it 'returns an empty array' do
        expect(described_class.apply(instructions)).to eq([])
      end
    end

    context 'when +=' do
      let(:instructions) do
        [
          ins('assign', ['state.count'], 'STATE_COUNT', 0),
          ins('add', %w[STATE_COUNT 1], 'STATE_COUNT', 0)
        ]
      end

      let(:expected_instructions) do
        [
          ins('add', ['state.count', '1'], 'state.count', 0)
        ]
      end

      it 'returns the optimized instructions' do
        expect(described_class.apply(instructions)).to eq(expected_instructions)
      end
    end

    context 'when -=' do
      let(:instructions) do
        [
          ins('assign', ['state.count'], 'STATE_COUNT', 0),
          ins('subtract', %w[STATE_COUNT 1], 'STATE_COUNT', 0)
        ]
      end

      let(:expected_instructions) do
        [
          ins('subtract', ['state.count', '1'], 'state.count', 0)
        ]
      end

      it 'returns the optimized instructions' do
        expect(described_class.apply(instructions)).to eq(expected_instructions)
      end
    end

    context 'when *=' do
      let(:instructions) do
        [
          ins('assign', ['state.count'], 'STATE_COUNT', 0),
          ins('multiply', %w[STATE_COUNT 1], 'STATE_COUNT', 0)
        ]
      end

      let(:expected_instructions) do
        [
          ins('multiply', ['state.count', '1'], 'state.count', 0)
        ]
      end

      it 'returns the optimized instructions' do
        expect(described_class.apply(instructions)).to eq(expected_instructions)
      end
    end

    context 'when /=' do
      let(:instructions) do
        [
          ins('assign', ['state.count'], 'STATE_COUNT', 0),
          ins('divide', %w[STATE_COUNT 1], 'STATE_COUNT', 0)
        ]
      end

      let(:expected_instructions) do
        [
          ins('divide', ['state.count', '1'], 'state.count', 0)
        ]
      end

      it 'returns the optimized instructions' do
        expect(described_class.apply(instructions)).to eq(expected_instructions)
      end
    end

    context 'when more complex +=' do
      let(:instructions) do
        [
          ins('assign', ['state.count'], 'BINARY_EXPRESSION_LEFT', 0),
          ins('add', %w[BINARY_EXPRESSION_LEFT incr], 'BINARY_EXPRESSION_LEFT', 0)
        ]
      end

      let(:expected_instructions) do
        [
          ins('add', %w[state.count incr], 'state.count', 0)
        ]
      end

      it 'returns the optimized instructions' do
        expect(described_class.apply(instructions)).to eq(expected_instructions)
      end
    end
  end
end

def ins(instruction, inputs, assign, scope)
  DTRCore::Instruction.new(instruction, inputs, assign, scope)
end
